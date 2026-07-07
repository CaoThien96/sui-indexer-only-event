use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use async_trait::async_trait;
use object_store::ObjectStore;
use object_store::path::Path as ObjectPath;
use prost::Message;
use prost_types::FieldMask;
use sui_rpc::client::Client as RpcClient;
use sui_rpc::field::FieldMaskUtil;
use sui_rpc::proto::sui::rpc::v2::GetCheckpointRequest;
use sui_rpc::proto::sui::rpc::v2 as proto;
use sui_types::full_checkpoint_content::Checkpoint;
use tonic::Code;
use url::Url;

#[derive(Debug, thiserror::Error)]
pub enum FetchError {
    #[error("checkpoint not found")]
    NotFound,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

fn decode_remote_store_bytes(bytes: &[u8]) -> Result<Checkpoint> {
    let decompressed = zstd::decode_all(bytes).context("zstd decompress checkpoint")?;
    let proto_checkpoint = proto::Checkpoint::decode(&decompressed[..])
        .context("prost decode checkpoint")?;
    Checkpoint::try_from(&proto_checkpoint).map_err(|e| anyhow!("proto to checkpoint: {e}"))
}

async fn fetch_via_grpc(client: &RpcClient, sequence: u64) -> Result<Checkpoint, FetchError> {
    let request = GetCheckpointRequest::by_sequence_number(sequence).with_read_mask(
        FieldMask::from_paths([
            "summary.bcs",
            "signature",
            "contents.bcs",
            "transactions.transaction.bcs",
            "transactions.effects.bcs",
            "transactions.effects.unchanged_loaded_runtime_objects",
            "transactions.events.bcs",
            "objects.objects.bcs",
        ]),
    );

    let response = client
        .clone()
        .ledger_client()
        .get_checkpoint(request)
        .await
        .map_err(|status| match status.code() {
            Code::NotFound => FetchError::NotFound,
            _ => FetchError::Other(anyhow!(status)),
        })?;

    let inner = response.into_inner();
    tokio::task::spawn_blocking(move || {
        Checkpoint::try_from(inner.checkpoint()).map_err(|e| anyhow!("grpc checkpoint: {e}"))
    })
    .await
    .context("grpc decode join")?
    .map_err(FetchError::Other)
}

async fn fetch_via_remote_store(
    store: &Arc<dyn ObjectStore>,
    sequence: u64,
) -> Result<Checkpoint, FetchError> {
    let path = ObjectPath::from(format!("{sequence}.binpb.zst"));
    let bytes = store.get(&path).await.map_err(|e| match e {
        object_store::Error::NotFound { .. } => FetchError::NotFound,
        other => FetchError::Other(anyhow!(other)),
    })?;
    let bytes = bytes.bytes().await.map_err(|e| FetchError::Other(anyhow!(e)))?;
    tokio::task::spawn_blocking(move || decode_remote_store_bytes(&bytes))
        .await
        .context("remote store decode join")?
        .map_err(FetchError::Other)
}

#[async_trait]
pub trait CheckpointSource: Send + Sync {
    async fn get_checkpoint(&self, sequence: u64) -> Result<Checkpoint, FetchError>;
}

pub struct ArchivalGrpcSource {
    client: RpcClient,
}

impl ArchivalGrpcSource {
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            client: RpcClient::new(url).context("archival gRPC client")?,
        })
    }
}

#[async_trait]
impl CheckpointSource for ArchivalGrpcSource {
    async fn get_checkpoint(&self, sequence: u64) -> Result<Checkpoint, FetchError> {
        fetch_via_grpc(&self.client, sequence).await
    }
}

pub struct RemoteStoreSource {
    store: Arc<dyn ObjectStore>,
}

impl RemoteStoreSource {
    pub fn new(url: &str) -> Result<Self> {
        let parsed = Url::parse(url).context("invalid REMOTE_STORE_URL")?;
        let store = object_store::http::HttpBuilder::new()
            .with_url(parsed.as_str())
            .build()
            .context("remote store http builder")?;
        Ok(Self {
            store: Arc::new(store),
        })
    }
}

#[async_trait]
impl CheckpointSource for RemoteStoreSource {
    async fn get_checkpoint(&self, sequence: u64) -> Result<Checkpoint, FetchError> {
        fetch_via_remote_store(&self.store, sequence).await
    }
}

/// Tries archival gRPC first, then HTTPS remote store on `NotFound`.
pub struct CheckpointFetcher {
    archival: Option<ArchivalGrpcSource>,
    remote_store: Option<RemoteStoreSource>,
}

impl CheckpointFetcher {
    pub fn from_env() -> Result<Self> {
        let archival = Some(ArchivalGrpcSource::new(&crate::config::sui_archival_grpc_url())?);
        let remote_store = crate::config::remote_store_url()
            .map(|url| RemoteStoreSource::new(&url))
            .transpose()?;

        if archival.is_none() && remote_store.is_none() {
            anyhow::bail!("SUI_ARCHIVAL_GRPC_URL or REMOTE_STORE_URL must be set");
        }

        Ok(Self {
            archival,
            remote_store,
        })
    }

    pub async fn get_checkpoint(&self, sequence: u64) -> Result<Checkpoint, FetchError> {
        if let Some(archival) = &self.archival {
            match archival.get_checkpoint(sequence).await {
                Ok(cp) => return Ok(cp),
                Err(FetchError::NotFound) => {}
                Err(e) => return Err(e),
            }
        }

        if let Some(remote) = &self.remote_store {
            return remote.get_checkpoint(sequence).await;
        }

        Err(FetchError::NotFound)
    }
}

