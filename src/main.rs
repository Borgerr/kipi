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

async fn update(
    cred: &Cred, website: &str, pool: &sqlx::PgPool
) -> Result<(), Box<dyn Error>> {
    let query = "UPDATE creds SET usr = $1, pass = $2 WHERE website = $3";

    sqlx::query(query)
        .bind(&cred.usr)
        .bind(&cred.pass)
        .bind(&cred.website)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read(pool: &sqlx::PgPool) -> Result<Cred, Box<dyn Error>> {
    let q_str = "SELECT usr, pass, website FROM creds";
    let query = sqlx::query(q_str);

    let row = query.fetch_one(pool).await?;
    let cred = Cred{
        usr: row.get("usr"),
        pass: row.get("pass"),
        website: row.get("website"),
    };

    Ok(cred)
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
