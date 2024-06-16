use sqlx::Row;
use std::{
    error::Error,
    io::{self, Write},
};

use super::VaultCred;

struct Cred {
    pub usr: String,
    pub pass: String,
    pub service: String,
}

enum AccessAction {
    // TODO: finish planning with this
    // should integrate with `select_action`
    Create { service: String },
    Read { service: String },
    Update { service: String },
    Delete { service: String },
}

pub async fn create_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // TODO: check postgreSQL/sqlx docs, create a new table or a new user with name and pass
    // within the database, and verify that it's in long-term.
    // Should also look into ways to combat SQL injection
    let query = "INSERT INTO vaults (nam, pass) VALUES ($1, $2)";

    sqlx::query(query)
        .bind(&vc.name)
        .bind(&vc.pass)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    verify_vaultcred(vc, pool).await?;
    todo!()
}

pub async fn access_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
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
                    service: get_service(),
                }
            }
            "2\n" => {
                break AccessAction::Read {
                    service: get_service(),
                }
            }
            "3\n" => {
                break AccessAction::Update {
                    service: get_service(),
                }
            }
            "4\n" => {
                break AccessAction::Delete {
                    service: get_service(),
                }
            }
            _ => continue,
        }
    }
}

fn get_service() -> String {
    let mut buffer = String::new();
    print!("Service: ");
    io::stdin().read_line(&mut buffer).unwrap(); // TODO: handle this error
    String::from(buffer.trim())
}

async fn verify_vaultcred(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    // TODO: verify previous table with vault credentials exists, and that these are the correct credentials
    let query = "SELECT EXISTS(SELECT * from vaults WHERE nam = $1, pass = $2)";

    sqlx::query(query)
        .bind(&vc.name)
        .bind(&vc.pass)
        .execute(pool)
        .await?;

    // TODO: extract the 1 or 0 result
    // https://www.tutorialspoint.com/best-way-to-test-if-a-row-exists-in-a-mysql-table

    todo!()
}
