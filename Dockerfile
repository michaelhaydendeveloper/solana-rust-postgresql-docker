# Use the official Rust image as the base image
FROM rust:latest

# Install dependencies
RUN apt-get update && \
    apt-get install -y libssl-dev pkg-config build-essential curl libudev-dev

# Install Solana CLI (latest stable version)
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.14.17/install)"

# Add Solana CLI to PATH
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Create a new directory for the app
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY ./Cargo.toml ./Cargo.lock ./

# Copy the source code to the container
COPY ./src ./src

# Install the necessary Rust dependencies
RUN cargo build

# Install cargo-watch for hot reloading
RUN cargo install cargo-watch

# Command to run cargo watch
CMD ["cargo", "watch", "-x", "run"]
