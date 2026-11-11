use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

use super::{AuthError, AuthResult};

const SESSION_HOURS: i64 = 72;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub token: String,
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: i64, username: String, email: String, display_name: String) -> Self {
        let now = Utc::now();
        Self {
            token: Uuid::new_v4().to_string(),
            user_id,
            username,
            email,
            display_name,
            created_at: now,
            expires_at: now + Duration::hours(SESSION_HOURS),
        }
    }

    pub fn is_valid(&self) -> bool {
        Utc::now() < self.expires_at
    }
}

pub struct SessionManager {
    path: PathBuf,
}

impl SessionManager {
    pub fn new(data_dir: &std::path::Path) -> AuthResult<Self> {
        fs::create_dir_all(data_dir).map_err(|e| AuthError::Other(e.to_string()))?;
        Ok(Self {
            path: data_dir.join("session.json"),
        })
    }

    pub fn save(&self, session: &Session) -> AuthResult<()> {
        let json = serde_json::to_string_pretty(session)
            .map_err(|e| AuthError::Other(e.to_string()))?;
        fs::write(&self.path, json).map_err(|e| AuthError::Other(e.to_string()))?;
        Ok(())
    }

    pub fn load(&self) -> AuthResult<Option<Session>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let raw = fs::read_to_string(&self.path).map_err(|e| AuthError::Other(e.to_string()))?;
        let session: Session =
            serde_json::from_str(&raw).map_err(|e| AuthError::Other(e.to_string()))?;
        if session.is_valid() {
            Ok(Some(session))
        } else {
            let _ = self.clear();
            Err(AuthError::SessionExpired)
        }
    }

    pub fn clear(&self) -> AuthResult<()> {
        if self.path.exists() {
            fs::remove_file(&self.path).map_err(|e| AuthError::Other(e.to_string()))?;
        }
        Ok(())
    }
}
