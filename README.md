# MySQL Rust Application

This project is a Rust application that interacts with a MySQL database. It allows users to insert and view user records in the database.

## Features

- Connect to a MySQL database
- Initialize the database schema
- Insert user records
- View user records

## Prerequisites

- Rust (latest stable version)
- MySQL server

## Setup

1. **Clone the repository:**

    ```sh
    git clone https://github.com/Rohandevops/r-mysqlgit
    cd r-mysql
    ```

2. **Set up the MySQL database:**

    - Ensure MySQL server is running.
    - Create a database named `rohanpy`.
    - Update the database URL in the `src/main.rs` file if necessary.

3. **Build the project:**

    ```sh
    cargo build
    ```

4. **Run the project:**

    ```sh
    cargo run
    ```

## Usage

1. **Insert User:**
    - Follow the prompts to enter the user's name and age.

2. **View Users:**
    - View the list of users stored in the database.

3. **Exit:**
    - Exit the application.

## Error Handling

- The application uses the `log` crate to log errors.
- Errors are propagated using the `Result` type and handled appropriately.

## Dependencies

- `mysql`
- `tokio`
- `log`
- `env_logger`

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
