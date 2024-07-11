```markdown
# Solana Rust Blockchain Development with Docker

This repository provides a development environment for building blockchain applications using Solana and Rust, with a PostgreSQL database and hot reloading on file changes. The environment is set up using Docker and Docker Compose.

## Prerequisites

Ensure you have the following installed on your machine:

- Docker
- Docker Compose

## Project Structure

```
/your-project
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
├── .env
├── Dockerfile
└── docker-compose.yml
```

## Setup Instructions

1. **Clone the repository**:

    ```sh
    git clone https://github.com/your-username/your-project.git
    cd your-project
    ```

2. **Create a `.env` file**:

    Create a `.env` file in the root of your project with the following content:

    ```dotenv
    POSTGRES_USER=user
    POSTGRES_PASSWORD=password
    POSTGRES_DB=mydatabase
    POSTGRES_HOST=db
    POSTGRES_PORT=5432
    RUST_LOG=debug
    ```

3. **Build and start the services**:

    ```sh
    docker-compose up --build
    ```

    This command will build the Docker images and start the services defined in the `docker-compose.yml` file.
    Be patient, the Cargo Run command can take a bit of time to start after the PostgreSQL DB is initialized.
    Also Cargo Run takes a VERY long time to complete after it starts running.

    Keep in mind the Rust image is Greater than 4GB in size...YIKES!!!

4. **Access the application**:

    The Rust application will be running with hot reloading enabled. Any changes you make to the source files will automatically restart the application.

5. **Connect to the PostgreSQL database**:

    - Host: `localhost`
    - Port: `5432`
    - Username: `user`
    - Password: `password`
    - Database: `mydatabase`

    You can connect to the database using any PostgreSQL client or from within your Rust application.

## Using the Solana CLI

The Solana CLI is installed and available in the development container. You can use it to interact with the Solana blockchain. For example:

```sh
docker-compose exec solana_rust_app solana --version
```

## Sample Code

The `main.rs` file contains a simple example that connects to the Solana blockchain and a PostgreSQL database. It performs the following actions:

1. Connects to the PostgreSQL database and creates a `balances` table.
2. Inserts a dummy entry into the `balances` table.
3. Queries the dummy entry from the `balances` table.
4. Connects to the Solana blockchain and queries the balance of a specific public key.

### Example Output

When you run the application, you should see output similar to:

```
Public Key: DummyPublicKey, Balance: 0
Hello, Solana! The balance of H3Av3R6y5zXYeq7MTZVG2J3eZ1gKVEFyR3F4FVeCjC2H is <balance> lamports.
```

## Dependencies

- **Rust**: The Rust programming language.
- **Solana CLI**: Command-line interface for interacting with the Solana blockchain.
- **PostgreSQL**: Relational database system.
- **cargo-watch**: Tool for automatically rebuilding and running Cargo projects on file changes.

## Customizing the Setup

- **Add Rust dependencies**: Update the `Cargo.toml` file to include any additional Rust dependencies your project requires.
- **Modify database configuration**: Update the `.env` file to change the PostgreSQL database configuration.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any improvements or suggestions.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

```