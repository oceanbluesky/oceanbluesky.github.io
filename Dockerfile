# Use Rust nightly since Dioxus often requires latest features
FROM rustlang/rust:nightly-bookworm-slim

# Prevent interactive prompts during build
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    curl \
    git \
    jq \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g npm@latest

# Install Dioxus CLI
RUN cargo install dioxus-cli

# Set working directory
WORKDIR /app

# Copy Rust dependency files
COPY Cargo.toml Cargo.lock ./

# Install the correct wasm-bindgen-cli version by reading the dependency tree
RUN cargo metadata --format-version 1 \
    | jq -r '.packages[] | select(.name == "wasm-bindgen") | .version' \
    | xargs -I {} cargo install -f wasm-bindgen-cli --version {}

# Copy package files first to leverage Docker cache
COPY package.json ./
RUN npm install

# Create dummy src to build dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"dummy\")}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Create required directories for Tailwind
RUN mkdir -p src/styles dist/assets/styles

# Copy the rest of the application
COPY . .

# Ensure the output directory for Tailwind exists
RUN mkdir -p dist/assets/styles

# Expose port
EXPOSE 8080

# CMD
CMD ["npm", "run", "serve"]