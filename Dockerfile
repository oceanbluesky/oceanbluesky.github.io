ARG VARIANT="nightly-bookworm-slim"
FROM rustlang/rust:${VARIANT}

# Suppress interactive prompts in Docker builds
ENV DEBIAN_FRONTEND=noninteractive

# Install required dependencies, including Node.js and OpenSSL
RUN apt-get update && \
    apt-get install -qq -y \
    build-essential \
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    curl \
    pkg-config \
    libssl-dev && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -qq -y nodejs && \
    npm install -g concurrently && \
    apt-get clean

# Set working directory
WORKDIR /app

# Copy Node.js dependencies for TailwindCSS
COPY package*.json ./

# Install Node.js dependencies
RUN npm install

# Copy source files into the image
COPY . .

# Build TailwindCSS before building the Rust project
RUN npm run build:css

# Build the Rust project
RUN cargo build --release

# Expose the application port
EXPOSE 8080

# Start the application
CMD ["npm", "run", "serve"]
