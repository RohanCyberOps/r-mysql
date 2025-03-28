use mysql::{Opts, Pool, params};
use mysql::prelude::Queryable;
use std::io::{self, Write};
use tokio;
use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Consider using environment variables for credentials
    let db_url = "mysql://root:Rohan15@@localhost/rohanpy";
    let db_opts = Opts::from_url(db_url)?;

    // Create a connection pool
    let pool = Pool::new(db_opts)?;

    // Initialize database schema
    initialize_db(&pool).await?;

    // Main application loop
    run_application(&pool).await?;

    Ok(())
}

async fn initialize_db(pool: &Pool) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name TEXT NOT NULL,
            age INT NOT NULL
        )"
    )?;
    Ok(())
}

async fn run_application(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        display_menu();
        let choice = prompt_user("Enter your choice: ");

        match choice.as_str() {
            "1" => insert_user(pool).await?,
            "2" => view_users(pool).await?,
            "3" => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
    Ok(())
}

async fn insert_user(pool: &Pool) -> Result<(), Box<dyn std::error::Error>> {
    let name = prompt_user("Enter name: ");
    let age: u32 = match prompt_user("Enter age: ").parse() {
        Ok(age) if age <= 120 => age, // Basic validation
        _ => {
            println!("Invalid age. Please enter a number between 0 and 120.");
            return Ok(());
        }
    };

    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO users (name, age) VALUES (:name, :age)",
        params! {
            "name" => name,
            "age" => age,
        }
    )?;
    info!("User inserted successfully!");
    Ok(())
}

async fn view_users(pool: &Pool) -> Result<(), mysql::Error> {
    let mut conn = pool.get_conn()?;
    let results = conn.query("SELECT id, name, age FROM users")?;

    if results.is_empty() {
        println!("No users found.");
    } else {
        println!("Users:");
        for row in results {
            let (id, name, age): (u32, String, u32) = mysql::from_row(row);
            println!("ID: {}, Name: {}, Age: {}", id, name, age);
        }
    }
    Ok(())
}

fn display_menu() {
    println!("\nMenu:");
    println!("1. Insert User");
    println!("2. View Users");
    println!("3. Exit");
}

fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}