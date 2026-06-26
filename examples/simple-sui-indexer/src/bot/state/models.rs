use diesel::deserialize::{self, FromSql};
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenStatus {
    Created,
    Listing,
    Done,
}

impl TokenStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::Listing => "listing",
            Self::Done => "done",
        }
    }

    pub fn from_db(value: &str) -> Self {
        match value {
            "listing" => Self::Listing,
            "done" => Self::Done,
            _ => Self::Created,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dex {
    Cetus,
    Turbos,
}

impl Dex {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cetus => "CETUS",
            Self::Turbos => "TURBOS",
        }
    }

    pub fn from_db(value: &str) -> Option<Self> {
        match value {
            "CETUS" => Some(Self::Cetus),
            "TURBOS" => Some(Self::Turbos),
            _ => None,
        }
    }
}

impl ToSql<Text, Pg> for TokenStatus {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for TokenStatus {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        Ok(Self::from_db(
            std::str::from_utf8(bytes.as_bytes()).map_err(|e| e.to_string())?,
        ))
    }
}

impl ToSql<Text, Pg> for Dex {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(self.as_str().as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for Dex {
    fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
        Dex::from_db(std::str::from_utf8(bytes.as_bytes()).map_err(|e| e.to_string())?)
            .ok_or_else(|| "unknown dex".into())
    }
}

#[derive(Debug, Clone)]
pub struct BotToken {
    pub id: String,
    pub symbol: String,
    pub status: TokenStatus,
    pub pool_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BotPool {
    pub id: String,
    pub token_id: String,
    pub dex: Dex,
}

#[derive(Debug, Clone)]
pub struct ParsedSwap {
    pub event_id: String,
    pub tx_digest: String,
    pub event_seq: String,
    pub pool: String,
    pub is_buy: bool,
    pub sui_amount: u128,
    pub token_amount: u128,
    pub maker: String,
    pub dex: Dex,
}
