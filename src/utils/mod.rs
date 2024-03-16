#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{Connection, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        Ok(SqliteConnection::connect("sqlite:db.db").await?)
    }
}

mod globals;
pub use self::globals::*;
