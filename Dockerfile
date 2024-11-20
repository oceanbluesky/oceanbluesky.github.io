# syntax=docker/dockerfile:1

ARG NODE_VERSION=23.0.0
FROM node:${NODE_VERSION}-alpine

# Install necessary system dependencies for Rust and Dioxus CLI
RUN apk add --no-cache curl gcc musl-dev perl make

# Install Rust and Dioxus CLI as root, then adjust permissions
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable && \
    source $HOME/.cargo/env && \
    PATH="$HOME/.cargo/bin:$PATH" && \
    cargo install dioxus-cli --root /usr/local && \
    chmod +x /usr/local/bin/dx

# Add Cargo and Rust to the PATH and transfer ownership to the `node` user
ENV PATH="/root/.cargo/bin:/usr/local/bin:${PATH}"
RUN chown -R node:node /root/.cargo /root/.rustup /usr/local/bin/dx

# Use production node environment by default
ENV NODE_ENV=production

WORKDIR /usr/src/app

# Copy package files into the image, install dependencies
COPY package.json package-lock.json ./
RUN npm ci --omit=dev && npm install concurrently tailwindcss --no-save

# Create necessary directories with permissions for the node user
RUN mkdir -p dist/assets/styles target && chown -R node:node dist target /usr/src/app

# Switch to non-root `node` user for the remaining operations
USER node

# Copy the rest of the application source files
COPY . .

# Expose the port for the application
EXPOSE 8080

# Run the application as non-root
CMD ["npm", "run", "serve"]
