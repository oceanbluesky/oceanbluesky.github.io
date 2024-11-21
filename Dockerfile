# syntax=docker/dockerfile:1


FROM duel80003/rust-dioxus:latest 


# Use production node environment by default
ENV NODE_ENV=production

# Set the working directory
WORKDIR /usr/src/app

# Copy package.json and install dependencies
COPY package.json package-lock.json ./
RUN npm ci --omit=dev && npm install concurrently tailwindcss --no-save

# Copy the rest of the application code
COPY . .

# Expose the port for the application
EXPOSE 8080

# Run the application
CMD ["npm", "run", "serve"]