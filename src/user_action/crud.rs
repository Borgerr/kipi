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

// TODO: combat SQL injection
// current implementation is the happiest path there ever was

pub async fn create_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if verify_vaultcred(vc, pool).await? {
        println!("Vault using that name already exists, try a different one or log in");
    } else {
        let query = "INSERT INTO vaults (nam, pass) VALUES ($1, $2)";
        sqlx::query(query)
            .bind(&vc.name)
            .bind(&vc.pass)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn delete_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if verify_vaultcred(vc, pool).await? {
        // TODO: delete vault
        println!("Deleting vault...");
    }
    todo!("handle vault access")
}

pub async fn access_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if verify_vaultcred(vc, pool).await? {
        // TODO: handle vault access
        println!("Accessing vault...");
    }
    todo!("handle vault access")
}

fn select_action() -> AccessAction {
    loop {
        println!("Select action:\n1. Create\n2. Read\n3. Update\n4. Delete");
        print!("[1|2|3|4] -> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap(); // TODO: handle this error

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

async fn verify_vaultcred(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<bool, Box<dyn Error>> {
    let query = "SELECT EXISTS(SELECT * from vaults WHERE nam = $1 AND pass = $2) as result";

    let res = sqlx::query(query)
        .bind(&vc.name)
        .bind(&vc.pass)
        .fetch_one(pool)
        .await?;

    let b: bool = res.get("result");
    Ok(b)
}
