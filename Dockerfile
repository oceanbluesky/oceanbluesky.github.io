# Use a Rust image with a compatible version
FROM rust:1.73

# Set the working directory
WORKDIR /usr/src/myapp

# Copy source code into the container
COPY . .

# Install Node.js and npm for Tailwind CSS
RUN apt-get update && apt-get install -y nodejs npm

# Install Tailwind CSS and build the CSS file
RUN npm install
RUN npx tailwindcss build -o dist/assets/styles/tailwind.css

# Install dependencies and build the Rust application
RUN cargo update && cargo build --release

# Define the default command
CMD ["./target/release/github_bio"]
