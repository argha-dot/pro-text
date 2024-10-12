#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::{migrate::MigrateDatabase, Connection, Sqlite, SqliteConnection};

    pub async fn db() -> Result<SqliteConnection, ServerFnError> {
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        // let db_url = "sqlite:db.sqlite".to_string();

        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("Database does not exist, creating it");
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("Database created"),
                Err(e) => panic!("Error creating database: {:?}", e),
            }
        }

        Ok(SqliteConnection::connect(&db_url).await?)
    }
}

mod globals;
mod route_stuff;

pub use self::globals::*;
pub use self::route_stuff::*;
