use crate::models::User;
use leptos::*;

#[server(AddUser)]
pub async fn create_user(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    let username = username.clone();
    let rows = sqlx::query!("SELECT * FROM users WHERE username = $1", username)
        .fetch_all(&mut conn)
        .await?;

    if rows.len() > 0 {
        return Err(ServerFnError::ServerError(
            "Username Not Available".to_string(),
        ));
    }

    println!("Createing User");
    match sqlx::query(
        r#"
        INSERT INTO users (username, password_hash)
        VALUES ($1, $2)
        "#,
    )
    .bind(&username)
    .bind(&password)
    .execute(&mut conn)
    .await
    {
        Ok(_) => {
            println!("User Created");
            Ok(())
        }
        Err(e) => {
            println!("Error Creating User {:?}", e);
            Err(ServerFnError::ServerError(e.to_string()))
        }
    }
}

/// Login server function
#[server(Login)]
pub async fn login(username: String, password: String) -> Result<User, ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    logging::log!("Login {:?}", password);
    match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&username)
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

/// User Exists
#[server(UserExists)]
pub async fn check_user_exists(user_name: String) -> Result<bool, ServerFnError> {
    use crate::utils::ssr::*;

    let mut conn = db().await?;

    match sqlx::query_scalar::<_, bool>("SELECT EXISTS (SELECT 1 FROM users WHERE username = $1)")
        .bind(user_name.clone())
        .fetch_one(&mut conn)
        .await
    {
        Ok(val) => Ok(val),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
