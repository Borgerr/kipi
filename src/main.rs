use sqlx::Connection;
use sqlx::Row;
use std::error::Error;

struct Cred {
    pub usr: String,
    pub pass: String,
    pub website: String,
}

async fn create(cred: &Cred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO creds (usr, pass, website) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(&cred.usr)
        .bind(&cred.pass)
        .bind(&cred.website)
        .execute(pool)
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let url = String::from(
        args.nth(1)
        .expect("(!) need to provide PostgreSQL URL as first argument"),
    );

    let pool = sqlx::postgres::PgPool::connect(&url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    Ok(())
}
