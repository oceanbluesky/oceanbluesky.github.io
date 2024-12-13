# Stage 1: Build the Dioxus application and Tailwind CSS
FROM rustlang/rust:nightly-bookworm-slim AS builder

# Prevent interactive prompts during build
ENV DEBIAN_FRONTEND=noninteractive

# Install Python 3, system dependencies, and other required packages
RUN apt-get update && apt-get install -y \
    python3 \
    python3-venv \
    python3-pip \
    build-essential \
    pkg-config \
    libssl-dev \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    curl \
    git \
    jq \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

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

# Copy package.json and install Node dependencies
COPY package.json ./
RUN npm install

# Copy the rest of the application
COPY . .

# Build Tailwind CSS and the Dioxus application
RUN npm run build:css
RUN dx build --release --platform web



# SECOND IMAGE


# Stage 2: Create a lightweight image for serving
FROM debian:bullseye-slim

# Prevent interactive prompts during runtime
ENV DEBIAN_FRONTEND=noninteractive

# Set working directory
WORKDIR /app

# Copy build output from the builder stage
COPY --from=builder /app/dist /app/dist

# Install a lightweight HTTP server
RUN apt-get update && apt-get install -y \
    curl \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

# Expose port
EXPOSE 8080

# Start the server to serve static files
CMD ["sh", "-c", "cd dist && python3 -m http.server 8080"]


# Explanation
# Multi-Stage Build:
# Stage 1 (Builder):
# Includes all build tools (Rust, Node.js, Dioxus CLI).
# Builds Tailwind CSS (npm run build:css) and the Dioxus application (dx build).
# Stage 2 (Final Image):
# Copies the built dist folder from the builder stage.
# Uses a lightweight base image (debian:bullseye-slim).
# Avoids including unnecessary tools (Rust, Node.js) in the final image.
# Static File Serving:
# The dist directory is served using Python's built-in HTTP server.
# Smaller Final Image:
# The final image is lightweight since it only includes the static files and a minimal HTTP server.