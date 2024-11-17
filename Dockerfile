# syntax=docker/dockerfile:1

ARG NODE_VERSION=23.0.0
FROM node:${NODE_VERSION}-alpine

# Install system dependencies needed for Rust and Dioxus CLI
RUN apk add --no-cache curl gcc musl-dev perl make

# Install Rust and Dioxus CLI
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable && \
    echo 'source $HOME/.cargo/env' >> /etc/profile && \
    source $HOME/.cargo/env && \
    PATH="$HOME/.cargo/bin:$PATH" && \
    cargo install dioxus-cli --root /usr/local && \
    chmod +x /usr/local/bin/dx  # Set permissions for dx only

# Ensure Cargo bin directory is in PATH for all users
ENV PATH="/root/.cargo/bin:/usr/local/bin:${PATH}"

# Use production node environment by default
ENV NODE_ENV=production

WORKDIR /usr/src/app

# Copy package files into the image and install dependencies
COPY package.json package-lock.json ./
RUN npm ci --omit=dev && npm install concurrently tailwindcss --no-save

# Create necessary directories and set permissions
RUN mkdir -p dist/assets/styles target && chown -R node:node dist target

# Switch to non-root user for running the application
USER node

# Copy the rest of the source files into the image
COPY . .

# Expose the port that the application listens on
EXPOSE 8080

# Run the application
CMD ["npm", "run", "serve"]
