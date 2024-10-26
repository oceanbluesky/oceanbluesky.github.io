# Start from the official Rust image
FROM rust:latest AS builder

# Create a new directory for the Rust project
WORKDIR /usr/src/app

# Copy the Cargo.toml and Cargo.lock to set up dependencies
COPY Cargo.toml Cargo.lock ./

# Build only dependencies to cache them, improving build times
RUN cargo fetch
RUN cargo build --release
RUN rm -rf target/release/deps/xAI_site*

# Copy the entire project source into the container
COPY . .

# Compile the project for release
RUN cargo build --release

# Use a lightweight runtime image to run the compiled binary
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/xAI_site /usr/local/bin/xAI_site

# Expose the port your app will run on, e.g., 8080
EXPOSE 8080

# Set the startup command to run the binary
CMD ["xAI_site"]
