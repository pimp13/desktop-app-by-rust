use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

use super::password::{hash_password, verify_password};
use super::validation::{validate_email, validate_password, validate_username};
use super::{AuthError, AuthResult, Session};

const MAX_FAILED_ATTEMPTS: i64 = 5;
const LOCKOUT_MINUTES: i64 = 15;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

pub struct AuthStore {
    conn: Mutex<Connection>,
}

impl AuthStore {
    pub fn open(path: &Path) -> AuthResult<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| AuthError::Other(e.to_string()))?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "
            PRAGMA foreign_keys = ON;
            PRAGMA journal_mode = WAL;

            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT NOT NULL UNIQUE COLLATE NOCASE,
                email TEXT NOT NULL UNIQUE COLLATE NOCASE,
                display_name TEXT NOT NULL,
                password_hash TEXT NOT NULL,
                failed_attempts INTEGER NOT NULL DEFAULT 0,
                locked_until TEXT,
                created_at TEXT NOT NULL,
                last_login_at TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
            CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
            ",
        )?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn register(
        &self,
        username: &str,
        email: &str,
        display_name: &str,
        password: &str,
    ) -> AuthResult<User> {
        let username = username.trim();
        let email = email.trim().to_lowercase();
        let display_name = display_name.trim();

        validate_username(username)?;
        validate_email(&email)?;
        validate_password(password)?;

        if display_name.is_empty() {
            return Err(AuthError::Validation("نام نمایشی الزامی است".into()));
        }
        if display_name.len() > 64 {
            return Err(AuthError::Validation(
                "نام نمایشی بیش از ۶۴ کاراکتر است".into(),
            ));
        }

        let password_hash = hash_password(password)?;
        let now = Utc::now();
        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;

        let existing: Option<String> = conn
            .query_row(
                "SELECT username FROM users WHERE username = ?1 COLLATE NOCASE OR email = ?2 COLLATE NOCASE LIMIT 1",
                params![username, email],
                |row| row.get(0),
            )
            .ok();

        if let Some(existing_user) = existing {
            if existing_user.eq_ignore_ascii_case(username) {
                return Err(AuthError::UsernameTaken);
            }
            return Err(AuthError::EmailTaken);
        }

        conn.execute(
            "INSERT INTO users (username, email, display_name, password_hash, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![username, email, display_name, password_hash, now.to_rfc3339()],
        )?;

        let id = conn.last_insert_rowid();
        Ok(User {
            id,
            username: username.to_string(),
            email,
            display_name: display_name.to_string(),
            created_at: now,
            last_login_at: None,
        })
    }

    pub fn login(&self, identifier: &str, password: &str) -> AuthResult<Session> {
        let identifier = identifier.trim();
        if identifier.is_empty() || password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;

        let row = conn.query_row(
            "SELECT id, username, email, display_name, password_hash, failed_attempts, locked_until
             FROM users
             WHERE username = ?1 COLLATE NOCASE OR email = ?1 COLLATE NOCASE",
            params![identifier],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                    row.get::<_, String>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, Option<String>>(6)?,
                ))
            },
        );

        let (id, username, email, display_name, password_hash, failed, locked_until) = match row {
            Ok(v) => v,
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                return Err(AuthError::InvalidCredentials);
            }
            Err(e) => return Err(AuthError::Database(e)),
        };

        if let Some(until) = locked_until {
            if let Ok(until_dt) = DateTime::parse_from_rfc3339(&until) {
                if Utc::now() < until_dt.with_timezone(&Utc) {
                    return Err(AuthError::AccountLocked);
                }
            }
        }

        drop(conn);

        if !verify_password(password, &password_hash)? {
            self.record_failure(id, failed)?;
            return Err(AuthError::InvalidCredentials);
        }

        self.record_success(id)?;

        Ok(Session::new(id, username, email, display_name))
    }

    fn record_failure(&self, user_id: i64, current_failed: i64) -> AuthResult<()> {
        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;
        let next = current_failed + 1;
        if next >= MAX_FAILED_ATTEMPTS {
            let until = (Utc::now() + Duration::minutes(LOCKOUT_MINUTES)).to_rfc3339();
            conn.execute(
                "UPDATE users SET failed_attempts = ?1, locked_until = ?2 WHERE id = ?3",
                params![next, until, user_id],
            )?;
        } else {
            conn.execute(
                "UPDATE users SET failed_attempts = ?1 WHERE id = ?2",
                params![next, user_id],
            )?;
        }
        Ok(())
    }

    fn record_success(&self, user_id: i64) -> AuthResult<()> {
        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;
        conn.execute(
            "UPDATE users SET failed_attempts = 0, locked_until = NULL, last_login_at = ?1 WHERE id = ?2",
            params![Utc::now().to_rfc3339(), user_id],
        )?;
        Ok(())
    }

    pub fn get_user(&self, user_id: i64) -> AuthResult<Option<User>> {
        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;
        let result = conn.query_row(
            "SELECT id, username, email, display_name, created_at, last_login_at FROM users WHERE id = ?1",
            params![user_id],
            |row| {
                let created_raw: String = row.get(4)?;
                let last_raw: Option<String> = row.get(5)?;
                Ok(User {
                    id: row.get(0)?,
                    username: row.get(1)?,
                    email: row.get(2)?,
                    display_name: row.get(3)?,
                    created_at: DateTime::parse_from_rfc3339(&created_raw)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                    last_login_at: last_raw.and_then(|s| {
                        DateTime::parse_from_rfc3339(&s)
                            .ok()
                            .map(|d| d.with_timezone(&Utc))
                    }),
                })
            },
        );

        match result {
            Ok(user) => Ok(Some(user)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(AuthError::Database(e)),
        }
    }

    pub fn user_count(&self) -> AuthResult<i64> {
        let conn = self.conn.lock().map_err(|e| AuthError::Other(e.to_string()))?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM users", [], |row| row.get(0))?;
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn register_and_login_flow() {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("securedesk-test-{stamp}.db"));
        let store = AuthStore::open(&path).unwrap();

        let user = store
            .register("pouya", "pouya@example.com", "Pouya", "Password1")
            .unwrap();
        assert_eq!(user.username, "pouya");

        let session = store.login("pouya@example.com", "Password1").unwrap();
        assert_eq!(session.user_id, user.id);

        assert!(matches!(
            store.login("pouya", "bad-password"),
            Err(AuthError::InvalidCredentials)
        ));

        let _ = std::fs::remove_file(path);
    }
}
