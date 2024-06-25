use std::{
    error::Error,
    io::{self, Write},
};

mod crud;
use crud::{access_vault, create_vault, delete_vault};

enum LoginAction {
    Create,
    Access,
    Delete,
    Quit,
}

struct VaultCred {
    name: String,
    pass: String,
}

fn print_options() {
    println!("\nWhat would you like to do?");
    println!("1. Create a new password vault");
    println!("2. Access an existing vault");
    println!("3. Delete a vault");
    println!("q. Quit");
    print!("\n[1|2|3|q] -> ");
    io::stdout().flush().unwrap();
}

pub async fn handle_login(pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    print!("\x1B[2J\x1B[1;1H");

    println!("----- Kipi -----");

    loop {
        print_options();
        if let Ok(choice) = read_choice() {
            if let Some(LoginAction::Quit) = choice {
                break Ok(());
            } else if let None = choice {
                println!("Please select a valid action");
                continue;
            }

            let name_prompt = String::from("Enter vault name: ");
            let pass_prompt = String::from("Enter vault password: ");
            if let (Ok(vaultname), Ok(password)) =
                (read_name(name_prompt), read_password(pass_prompt))
            {
                let vaultcred = VaultCred {
                    name: vaultname,
                    pass: password,
                };
                if let Some(LoginAction::Create) = choice {
                    create_vault(&vaultcred, pool).await?;
                } else if let Some(LoginAction::Access) = choice {
                    access_vault(&vaultcred, pool).await?;
                } else if let Some(LoginAction::Delete) = choice {
                    delete_vault(&vaultcred, pool).await?;
                }
            }
        } else {
            println!("(!) Error reading action selection")
        }
    }
}

fn read_choice() -> io::Result<Option<LoginAction>> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;

    Ok(match buffer.as_str() {
        "1\n" => Some(LoginAction::Create),
        "2\n" => Some(LoginAction::Access),
        "3\n" => Some(LoginAction::Delete),
        "q\n" => Some(LoginAction::Quit),
        _ => None,
    })
}

fn read_name(prompt: String) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut name = String::new();
    io::stdin().read_line(&mut name)?;
    name.pop();

    if name.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty vaultname",
        ))
    } else {
        Ok(name)
    }
}

fn read_password(prompt: String) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let pw = rpassword::read_password()?;

    if pw.is_empty() {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty password",
        ))
    } else {
        Ok(pw)
    }
}
