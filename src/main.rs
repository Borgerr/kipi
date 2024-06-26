use std::error::Error;

mod user_action;

use user_action::handle_login;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let url = String::from(
        args.nth(1)
            .expect("(!) need to provide PostgreSQL URL as first argument"),
    );

    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    handle_login(&pool).await?;

    Ok(())
}
