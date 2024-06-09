use sqlx::Row;
use std::{
    error::Error,
    io::{self, Write},
};

use super::VaultCred;

struct Cred {
    pub usr: String,
    pub pass: String,
    pub website: String,
}

enum AccessAction {
    // TODO: finish planning with this
    // should integrate with `select_action`
    Create { website: String },
    Read { website: String },
    Update { website: String },
    Delete { website: String },
}

pub async fn create_vault(vc: VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub async fn delete_vault(vc: VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    verify_vaultcred(vc, pool).await?;
    todo!()
}

pub async fn access_vault(vc: VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    verify_vaultcred(vc, pool).await?;
    todo!()
}

fn select_action() -> AccessAction {
    println!("Select action:\n1. Create\n2. Read\n3. Update\n4. Delete");
    print!("[1|2|3|4] -> ");
    io::stdout().flush().unwrap();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap(); // TODO: handle this error

    loop {
        match buffer.as_str() {
            "1\n" => {
                break AccessAction::Create {
                    website: get_website(),
                }
            }
            "2\n" => {
                break AccessAction::Read {
                    website: get_website(),
                }
            }
            "3\n" => {
                break AccessAction::Update {
                    website: get_website(),
                }
            }
            "4\n" => {
                break AccessAction::Delete {
                    website: get_website(),
                }
            }
            _ => continue,
        }
    }
}

fn get_website() -> String {
    let mut buffer = String::new();
    print!("Website: ");
    io::stdin().read_line(&mut buffer).unwrap(); // TODO: handle this error
    String::from(buffer.trim())
}

async fn verify_vaultcred(vc: VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // TODO: verify previous table with vault credentials exists, and that these are the correct credentials
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
