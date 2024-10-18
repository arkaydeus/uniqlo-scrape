# Use the official Rust image as a parent image
FROM rust:1.81 AS builder

# Set the working directory in the container
WORKDIR /usr/src/uniqlo_scraper

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Use a more recent Debian base image (Bookworm)
FROM debian:bookworm-slim

# Install necessary dependencies including chromium-driver
RUN apt-get update && apt-get install -y \
    chromium \
    chromium-driver \
    libglib2.0-0 \
    libnss3 \
    libgconf-2-4 \
    libfontconfig1 \
    && rm -rf /var/lib/apt/lists/*

# Set up the working directory
WORKDIR /app

# Copy the built executable from the builder stage
COPY --from=builder /usr/src/uniqlo_scraper/target/release/uniqlo_scraper .

# Copy the start script
COPY start.sh .
RUN chmod +x start.sh

# Add a non-root user
RUN useradd -m myuser
USER myuser

# Run the start script when the container launches
CMD ["./start.sh"]

# Expose the port the app runs on
EXPOSE 8080
