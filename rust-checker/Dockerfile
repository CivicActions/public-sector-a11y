# Dockerfile to build the Rust Container

# Use the official Rust image as the base image
FROM rust:latest

# Create a new directory for the application
RUN mkdir /app

# Copy the application code into the container
COPY . /app

# Change to the application directory
WORKDIR /app

# Build the application with release flags
RUN cargo build --release

# Expose port 8080
EXPOSE 8080

# Run the application
CMD ["/app/target/release/Rusty-Checks"]
docker build -t Rusty-Checks@latest .
