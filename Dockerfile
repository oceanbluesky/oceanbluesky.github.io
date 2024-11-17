# syntax=docker/dockerfile:1

ARG NODE_VERSION=23.0.0
FROM node:${NODE_VERSION}-alpine

# Install Rust dependencies
RUN apk add --no-cache curl gcc musl-dev perl make

# Switch to the `node` user
USER node

# Install Rust and Dioxus CLI as `node`
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    echo 'source $HOME/.cargo/env' >> ~/.profile && \
    source $HOME/.cargo/env && \
    PATH="$HOME/.cargo/bin:$PATH" && \
    cargo install dioxus-cli

# Set Cargo path
ENV PATH="/home/node/.cargo/bin:${PATH}"

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
RUN mkdir -p dist/assets/styles

# Copy the rest of the source files into the image
COPY . .

# Expose the port that the application listens on
EXPOSE 8080

# Run the application
CMD ["npm", "run", "serve"]
