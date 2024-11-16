# syntax=docker/dockerfile:1

ARG NODE_VERSION=23.0.0
FROM node:${NODE_VERSION}-alpine

# Install Rust and Dioxus CLI for running `dx` commands
RUN apk add --no-cache curl gcc musl-dev && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    source $HOME/.cargo/env && \
    cargo install dioxus-cli

# Use production node environment by default
ENV NODE_ENV=production

WORKDIR /usr/src/app

# Copy package files into the image
COPY package.json package-lock.json ./

# Install all dependencies (omit dev dependencies for production)
RUN npm ci --omit=dev

# Install specific devDependencies needed for `serve` script
RUN npm install concurrently tailwindcss --no-save

# Create the `dist` directory with proper permissions
RUN mkdir -p dist/assets/styles && chown -R node:node dist

# Run the application as a non-root user
USER node

# Copy the rest of the source files into the image
COPY . .

# Expose the port that the application listens on
EXPOSE 8080

# Run the application
CMD ["npm", "run", "serve"]
