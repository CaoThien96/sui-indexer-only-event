//! Thin gRPC client for pool object reads and coin metadata (Sui fullnode).

use std::str::FromStr;
use std::time::Duration;

use event_bindings::pool_id::coin_types_from_pool_type_params;
use move_core_types::language_storage::TypeTag;
use sui_rpc::client::Client;
use sui_rpc::field::{FieldMask, FieldMaskUtil};
use sui_rpc::proto::sui::rpc::v2::{GetCoinInfoRequest, GetObjectRequest};
use thiserror::Error;
use tokio::sync::Mutex;
use tonic::Code;

#[derive(Debug, Error)]
pub enum GrpcError {
    #[error("invalid pool object id: {0}")]
    InvalidObjectId(String),
    #[error("gRPC status: {0}")]
    Status(#[from] tonic::Status),
    #[error("pool object has no type")]
    MissingObjectType,
    #[error("failed to parse pool type: {0}")]
    InvalidPoolType(String),
    #[error("pool type has no coin type params")]
    MissingCoinTypes,
    #[error("coin metadata not found for {0}")]
    MetadataNotFound(String),
    #[error("request timed out after {0:?}")]
    Timeout(Duration),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrpcFailureKind {
    NotFound,
    Transient,
    Permanent,
}

pub fn classify_grpc_error(err: &GrpcError) -> GrpcFailureKind {
    match err {
        GrpcError::Status(status) => match status.code() {
            Code::NotFound => GrpcFailureKind::NotFound,
            Code::Unavailable
            | Code::ResourceExhausted
            | Code::DeadlineExceeded
            | Code::Aborted => GrpcFailureKind::Transient,
            _ => GrpcFailureKind::Permanent,
        },
        GrpcError::Timeout(_) => GrpcFailureKind::Transient,
        GrpcError::MetadataNotFound(_) => GrpcFailureKind::Permanent,
        GrpcError::InvalidObjectId(_)
        | GrpcError::MissingObjectType
        | GrpcError::InvalidPoolType(_)
        | GrpcError::MissingCoinTypes => GrpcFailureKind::Permanent,
    }
}

#[derive(Debug, Clone)]
pub struct CoinMetadata {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub decimals: u32,
    pub image_url: Option<String>,
}

pub struct SuiGrpcClient {
    client: Mutex<Client>,
    timeout: Duration,
}

impl SuiGrpcClient {
    pub fn new(url: &str, timeout: Duration) -> Result<Self, GrpcError> {
        let client = Client::new(url).map_err(GrpcError::Status)?;
        Ok(Self {
            client: Mutex::new(client),
            timeout,
        })
    }

    pub async fn get_pool_coin_types(
        &self,
        pool_id: &str,
    ) -> Result<(String, String), GrpcError> {
        let request = GetObjectRequest::default()
            .with_object_id(pool_id)
            .with_read_mask(FieldMask::from_paths(["object_type"]));

        let response = tokio::time::timeout(self.timeout, async {
            let mut client = self.client.lock().await;
            client.ledger_client().get_object(request).await
        })
        .await
        .map_err(|_| GrpcError::Timeout(self.timeout))?
        .map_err(GrpcError::Status)?;

        let object = response
            .into_inner()
            .object
            .ok_or(GrpcError::MissingObjectType)?;

        let type_str = object
            .object_type_opt()
            .ok_or(GrpcError::MissingObjectType)?;

        let tag =
            TypeTag::from_str(type_str).map_err(|e| GrpcError::InvalidPoolType(e.to_string()))?;
        let TypeTag::Struct(struct_tag) = tag else {
            return Err(GrpcError::InvalidPoolType(type_str.to_string()));
        };

        let (coin_a, coin_b) = coin_types_from_pool_type_params(&struct_tag.type_params)
            .ok_or(GrpcError::MissingCoinTypes)?;

        Ok((
            crate::coin_type::normalize(&coin_a),
            crate::coin_type::normalize(&coin_b),
        ))
    }

    pub async fn get_coin_metadata(&self, coin_type: &str) -> Result<CoinMetadata, GrpcError> {
        let normalized = crate::coin_type::normalize(coin_type);
        if normalized == crate::coin_type::SUI_COIN_TYPE {
            return Ok(CoinMetadata {
                name: Some("Sui".to_string()),
                symbol: Some("SUI".to_string()),
                decimals: 9,
                image_url: None,
            });
        }

        let request = GetCoinInfoRequest::default().with_coin_type(&normalized);

        let response = tokio::time::timeout(self.timeout, async {
            let mut client = self.client.lock().await;
            client.state_client().get_coin_info(request).await
        })
        .await
        .map_err(|_| GrpcError::Timeout(self.timeout))?
        .map_err(GrpcError::Status)?;

        let inner = response.into_inner();
        let metadata = inner
            .metadata_opt()
            .filter(|m| m.decimals_opt().is_some())
            .ok_or_else(|| GrpcError::MetadataNotFound(normalized.clone()))?;

        Ok(CoinMetadata {
            name: metadata.name_opt().map(str::to_string),
            symbol: metadata.symbol_opt().map(str::to_string),
            decimals: metadata.decimals_opt().unwrap_or(9),
            image_url: metadata.icon_url_opt().map(str::to_string),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn parses_pool_type_string_to_coin_types() {
        let pool_type = "0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::pool::Pool<0x15a837268acd6d5f1f02784048e129393cff48b9cd55b6b2839cbd60e31faa27::dogtrain::DOGTRAIN, 0x2::sui::SUI, 0x91bfbc386a41afcfd9b2533058d7e915a1d3829089cc268ff4333d54d6339ca1::fee100bps::FEE100BPS>";
        let tag = TypeTag::from_str(pool_type).unwrap();
        let TypeTag::Struct(struct_tag) = tag else {
            panic!("expected struct");
        };
        let (a, b) = coin_types_from_pool_type_params(&struct_tag.type_params).unwrap();
        let a = crate::coin_type::normalize(&a);
        let b = crate::coin_type::normalize(&b);
        assert!(a.contains("dogtrain::DOGTRAIN"));
        assert_eq!(b, crate::coin_type::SUI_COIN_TYPE);
    }

    #[test]
    fn classifies_not_found_as_permanent_skip() {
        let err = GrpcError::Status(tonic::Status::new(Code::NotFound, "missing"));
        assert_eq!(classify_grpc_error(&err), GrpcFailureKind::NotFound);
    }

    #[test]
    fn classifies_unavailable_as_transient() {
        let err = GrpcError::Status(tonic::Status::new(Code::Unavailable, "busy"));
        assert_eq!(classify_grpc_error(&err), GrpcFailureKind::Transient);
    }
}
