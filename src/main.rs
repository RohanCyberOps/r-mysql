use mysql::{Opts, Pool, params};
use mysql::prelude::Queryable;
use std::io::{self, Write}; // Import necessary I/O traits
use tokio;

#[tokio::main]
async fn main() -> Result<(), mysql::Error> {
    // Specify the MySQL connection URL
    let db_opts = Opts::from_url("mysql://root:Rohan15@@localhost/rohanpy")?;

    // Create a connection pool with minimum and maximum connections
    let pool = Pool::new_manual(10, 20, db_opts)?;

    // Get a connection from the pool (synchronously)
    let mut conn = pool.get_conn()?;

    // Execute the SQL to create the table if it doesn't exist
    conn.query_drop(
        r"CREATE TABLE IF NOT EXISTS users (
            id INT PRIMARY KEY AUTO_INCREMENT,
            name TEXT NOT NULL,
            age INT NOT NULL
        )"
    )?; // Removed .await, it's a synchronous call

    loop {
        // Display the menu
        println!("1. Insert User");
        println!("2. View Users");
        println!("3. Exit");

        // Get user choice
        print!("Enter your choice: ");
        io::stdout().flush()?; // Ensure the prompt is displayed

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim(); // Trim any whitespace

        match choice {
            "1" => {
                // Insert User
                let name = prompt_user("Enter name: ");
                let age: u32 = prompt_user("Enter age: ").parse().unwrap();

                conn.exec_drop(
                    "INSERT INTO users (name, age) VALUES (:name, :age)",
                    params! {
                        "name" => name,
                        "age" => age,
                    }
                )?; // Insert the user into the database
                println!("User inserted successfully!");
            }
            "2" => {
                // View Users
                let results = conn.query("SELECT id, name, age FROM users")?; // Query the users

                // Print the results
                println!("Users:");
                for row in results {
                    let (id, name, age): (u32, String, u32) = mysql::from_row(row); // Extract the fields
                    println!("ID: {:?}, Name: {:?}, Age: {:?}", id, name, age);
                }
            }
            "3" => {
                println!("Exiting...");
                break; // Exit the loop
            }
            _ => {
                println!("Invalid choice. Please try again.");
            }
        }
    }

    Ok(())
}

// Function to prompt user input
fn prompt_user(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush to ensure prompt shows before input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string() // Trim whitespace and return
}
