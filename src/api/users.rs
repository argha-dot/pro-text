use crate::models::User;
use leptos::*;

#[server(AddUser)]
pub async fn create_user(user_name: String) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    match sqlx::query("INSERT INTO users (username, username_hash) VALUES ($1, NULL)")
        .bind(user_name)
        .execute(&mut conn)
        .await
    {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

/// Login server function
#[server(Login)]
pub async fn login(user_name: String) -> Result<User, ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(user_name)
        .fetch_one(&mut conn)
        .await
    {
        Ok(val) => {
            logging::log!("{:?}", val);
            Ok(val)
        }
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
