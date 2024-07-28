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
