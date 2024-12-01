FROM rust:1.80.0

WORKDIR /backend

# Copy the contents of your Rust project to the container
COPY ./backend .

WORKDIR /backend

# Build the Rust project
RUN cargo build --release
