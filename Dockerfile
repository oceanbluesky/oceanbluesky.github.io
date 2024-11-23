# Use a stable Rust image
FROM rust:1.67

# Set the working directory
WORKDIR /usr/src/myapp

# Copy source code into the container
COPY . .

# Install dependencies and build the Rust application
RUN cargo install --path .

# Define the default command
CMD ["myapp"]
