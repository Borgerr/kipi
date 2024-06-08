use sqlx::Row;
use std::error::Error;

use super::VaultCred;

struct Cred {
    pub usr: String,
    pub pass: String,
    pub website: String,
}

pub async fn create_vault(vc: VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    todo!()
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

async fn update(cred: &Cred, website: &str, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
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
    let cred = Cred {
        usr: row.get("usr"),
        pass: row.get("pass"),
        website: row.get("website"),
    };

    Ok(cred)
}
