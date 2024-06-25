use copypasta::{ClipboardContext, ClipboardProvider};
use sqlx::Row;
use std::{
    error::Error,
    io::{self, Write},
};

use super::{read_name, read_password, VaultCred};

enum AccessAction {
    // TODO: finish planning with this
    // should integrate with `select_action`
    Create { service: String },
    Read { service: String },
    Update { service: String },
    Delete { service: String },
    Quit,
}

enum ReadQuery {
    Username,
    Password,
}

// TODO: combat SQL injection
// current implementation is the happiest path there ever was
// and each query is just begging to be injected

pub async fn create_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if verify_vaultcred(vc, pool).await? {
        // TODO: relying on vaultcred allows for vaults with same names but different passwords
        // change this to some other check
        println!("Vault using that name already exists, try a different one or log in");
        return Ok(());
    }
    // create row in vaults
    let vaults_query = "INSERT INTO vaults (nam, pass) VALUES ($1, $2)";
    let table_query = format!(
        "CREATE TABLE {} (
        nam varchar,
        pass varchar,
        service varchar
        );",
        vc.name
    );

    let vaults_create = sqlx::query(&vaults_query)
        .bind(&vc.name)
        .bind(&vc.pass)
        .execute(pool);

    // create table for vault
    let table_create = sqlx::query(&table_query).bind(&vc.name).execute(pool);

    match tokio::join!(vaults_create, table_create) {
        (Err(e), Err(_)) => Err(Box::new(e)),
        (Err(e), Ok(_)) => Err(Box::new(e)),
        (Ok(_), Err(e)) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

pub async fn delete_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if !verify_vaultcred(vc, pool).await? {
        println!("Wrong vault credentials, try again");
        return Ok(());
    }
    let vaults_query = format!("DROP TABLE {}", vc.name);
    let table_query = "DELETE FROM vaults WHERE nam = $1 AND pass = $2";

    let vaults_drop = sqlx::query(&vaults_query).execute(pool);
    let table_drop = sqlx::query(&table_query)
        .bind(&vc.name)
        .bind(&vc.pass)
        .execute(pool);

    match tokio::join!(vaults_drop, table_drop) {
        (Err(e), Err(_)) => Err(Box::new(e)),
        (Err(e), Ok(_)) => Err(Box::new(e)),
        (Ok(_), Err(e)) => Err(Box::new(e)),
        _ => Ok(()),
    }
}

pub async fn access_vault(vc: &VaultCred, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    if !verify_vaultcred(vc, pool).await? {
        println!("Wrong vault credentials, try again");
        return Ok(());
    }
    println!("Accessing vault...");

    loop {
        match select_action() {
            AccessAction::Create { service } => create_action(service, vc, pool).await?,
            AccessAction::Read { service } => read_action(service, vc, pool).await?,
            AccessAction::Update { service } => update_action(service, vc, pool).await?,
            AccessAction::Delete { service } => delete_action(service, vc, pool).await?,
            AccessAction::Quit => break,
        }
    }

    Ok(())
}

async fn create_action(
    service: String,
    vc: &VaultCred,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    if service_exists(&service, vc, pool).await? {
        println!("An entry for this service already exists. Please try something else");
        return Ok(());
    }

    let name = read_name(String::from("Enter username: "))?;
    let pass = read_password(String::from("Enter password: "))?;

    let query = format!(
        "INSERT INTO {} (nam, pass, service) VALUES ($1, $2, $3)",
        vc.name
    );

    sqlx::query(&query)
        .bind(&name)
        .bind(&pass)
        .bind(&service)
        .execute(pool)
        .await?;

    Ok(())
}

async fn read_action(
    service: String,
    vc: &VaultCred,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    if !service_exists(&service, vc, pool).await? {
        println!("This service doesn't exist in this vault. Please try something else");
        return Ok(());
    }

    let query = match select_user_or_pass() {
        ReadQuery::Username => format!(
            "SELECT (SELECT nam FROM {} WHERE service=$1) as result",
            vc.name
        ),
        ReadQuery::Password => format!(
            "SELECT (SELECT pass FROM {} WHERE service=$1) as result",
            vc.name
        ),
    };

    let res = sqlx::query(&query).bind(&service).fetch_one(pool).await?;

    let mut ctx =
        ClipboardContext::new().expect("Something went wrong when getting the clipboard context");
    ctx.set_contents(res.get("result"))
        .expect("Something went wrong when setting contents of clipboard");
    let _ = ctx.get_contents().unwrap(); // needed for some reason to actually place in clip tray

    println!("Information copied into your clip tray");

    Ok(())
}

async fn update_action(
    service: String,
    vc: &VaultCred,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    if !service_exists(&service, vc, pool).await? {
        println!("This service doesn't exist in this vault. Please try something else");
        return Ok(());
    }

    // really just a delete followed by an update
    delete_action(service.clone(), vc, pool).await?;
    create_action(service, vc, pool).await?;

    Ok(())
}

async fn delete_action(
    service: String,
    vc: &VaultCred,
    pool: &sqlx::PgPool,
) -> Result<(), Box<dyn Error>> {
    if !service_exists(&service, vc, pool).await? {
        println!("This service doesn't exist in this vault. Please try something else");
        return Ok(());
    }

    let query = format!("DELETE FROM {} WHERE service = $1", vc.name);
    sqlx::query(&query).bind(service).execute(pool).await?;

    Ok(())
}

fn select_user_or_pass() -> ReadQuery {
    loop {
        println!("Do you want to copy the username or the password?\n1. Username\n2. Password");
        print!("[1|2] -> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.as_str() {
            "1\n" => break ReadQuery::Username,
            "2\n" => break ReadQuery::Password,
            _ => continue,
        }
    }
}

fn select_action() -> AccessAction {
    loop {
        println!("Select action:\n1. Create\n2. Read\n3. Update\n4. Delete\nq. Quit");
        print!("[1|2|3|4|q] -> ");
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
            "q\n" => break AccessAction::Quit,
            _ => continue,
        }
    }
}

fn get_service() -> String {
    let mut buffer = String::new();
    print!("Service: ");
    io::stdout().flush().unwrap();
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

async fn service_exists(
    service: &String,
    vc: &VaultCred,
    pool: &sqlx::PgPool,
) -> Result<bool, Box<dyn Error>> {
    let query = format!(
        "SELECT EXISTS(SELECT * from {} WHERE service = $1) as result",
        vc.name
    );

    let res = sqlx::query(&query).bind(&service).fetch_one(pool).await?;

    let b = res.get("result");
    Ok(b)
}
