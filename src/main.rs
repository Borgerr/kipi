use sqlx::Connection;
use sqlx::Row;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args();
    let url = String::from(
        args.nth(1)
            .expect("(!) need to provide PostgreSQL URL as first argument"),
    );

    let mut conn = sqlx::postgres::PgConnection::connect(&url).await?;

    let res = sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&mut conn)
        .await?;
    let sum: i32 = res.get("sum");
    println!("1 + 1 = {}", sum);

    Ok(())
}
