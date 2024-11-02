use super::{deserialize_dt, serialize_dt};
use crate::rejections::{HashError, InvalidLogin};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tokio::sync::OnceCell;
use uuid::Uuid;
use warp::Rejection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Admin {
    pub id: String,
    pub username: String,
    pub password: String,
    #[serde(serialize_with = "serialize_dt", deserialize_with = "deserialize_dt")]
    pub created_at: DateTime<Utc>,
}

impl Default for Admin {
    fn default() -> Admin {
        Admin {
            id: Uuid::new_v4().to_string(),
            username: "".to_string(),
            password: "".to_string(),
            created_at: chrono::offset::Utc::now(),
        }
    }
}