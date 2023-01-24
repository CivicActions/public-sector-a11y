## Dockerfile to Generate Rust Containers

# Use the official rust image as the base image
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/rust-checker

# Copy the Cargo.toml and Cargo.lock files to the container
COPY rust-checker/Cargo.* ./
COPY rust-checker/yarn.lock ./

# Copy the rest of the application's source code to the container
COPY rust-checker/ .

# Install the dependencies and build the application
RUN cargo build --release

# Use the official alpine image as the base image
FROM alpine:latest

# Set the working directory
WORKDIR /usr/src/rust-checker

# Copy the binary from the builder stage
COPY --from=builder /usr/src/rust-checker/target/release/rust-checker .

# Set the GOOGLE_CLOUD_KEY environment variable
ENV GOOGLE_CLOUD_KEY="${{ secrets.GOOGLE_CLOUD_KEY }}"

# Expose the port that the application binds to
EXPOSE 8080

# Start the application
CMD ["./rust-checker"]

