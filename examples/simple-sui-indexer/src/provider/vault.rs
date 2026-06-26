use anyhow::{Context, Result, bail};
use fastcrypto::hash::HashFunction;
use fastcrypto::traits::{EncodeDecodeBase64, Signer};
use serde::Deserialize;
use shared_crypto::intent::{Intent, IntentMessage};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use sui_types::base_types::SuiAddress;
use sui_types::crypto::{DefaultHash, SUI_PRIV_KEY_PREFIX, SuiKeyPair};
use sui_types::signature::GenericSignature;
use sui_types::transaction::TransactionData;

/// Same shape as bot-snip `vault.json`: `{ publicKey, secretKey, index }`.
#[derive(Debug, Deserialize)]
struct VaultFile {
    #[serde(alias = "secretKey")]
    secret_key: String,
    #[serde(default, alias = "publicKey")]
    public_key: Option<String>,
    #[serde(default)]
    index: u32,
}

pub struct VaultKeypair {
    keypair: SuiKeyPair,
    address: SuiAddress,
}

impl VaultKeypair {
    pub fn from_env() -> Result<Arc<Self>> {
        let path = std::env::var("VAULT_PATH").unwrap_or_else(|_| "./vault.json".to_string());
        Self::from_path(&path)
    }

    pub fn from_path(path: &str) -> Result<Arc<Self>> {
        if Path::new(path).exists() {
            let raw = fs::read_to_string(path).with_context(|| format!("read {path}"))?;
            let vault: VaultFile = serde_json::from_str(&raw).context("parse vault.json")?;
            return Self::from_secret(&vault.secret_key, vault.public_key.as_deref());
        }
        bail!("vault.json not found at {path}; copy from bot-snip")
    }

    fn from_secret(secret: &str, expected_address: Option<&str>) -> Result<Arc<Self>> {
        let secret = secret.trim();
        if secret.is_empty() {
            bail!("vault secret key is empty — expected `secretKey` (bot-snip) or `secret_key`");
        }

        let keypair = if secret.starts_with(SUI_PRIV_KEY_PREFIX) {
            SuiKeyPair::decode(secret).map_err(|e| anyhow::anyhow!("bech32 decode: {e}"))?
        } else {
            SuiKeyPair::decode_base64(secret)
                .or_else(|_| {
                    let bytes =
                        base64::Engine::decode(&base64::engine::general_purpose::STANDARD, secret)?;
                    SuiKeyPair::from_bytes(&bytes).map_err(|e| anyhow::anyhow!("{e}"))
                })
                .map_err(|e| anyhow::anyhow!("base64 decode: {e}"))?
        };

        let address = SuiAddress::from(&keypair.public());
        if let Some(expected) = expected_address {
            let expected = expected.trim();
            if !expected.is_empty() && address.to_string() != expected {
                bail!(
                    "vault publicKey mismatch: file has {expected}, derived {address}"
                );
            }
        }

        Ok(Arc::new(Self { keypair, address }))
    }

    pub fn address(&self) -> SuiAddress {
        self.address
    }

    pub fn address_string(&self) -> String {
        self.address.to_string()
    }

    pub fn sign_transaction(&self, tx_data: &TransactionData) -> GenericSignature {
        let intent_msg = IntentMessage::new(Intent::sui_transaction(), tx_data);
        let mut hasher = DefaultHash::default();
        bcs::serialize_into(&mut hasher, &intent_msg).expect("bcs intent message");
        let digest = hasher.finalize().digest;
        GenericSignature::Signature(self.keypair.sign(&digest))
    }
}
