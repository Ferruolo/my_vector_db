# Use the official Rust image as a builder
FROM rust:1.75-slim-bullseye AS builder

# Create a new empty project
WORKDIR /usr/src/app
RUN cargo new --bin server 
WORKDIR /usr/src/app/server

# Copy over manifests
COPY Cargo.toml ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy actual source code
COPY src ./src

# Build for release
RUN cargo clean
RUN cargo build --release


# Create final, minimal image
FROM debian:bullseye-slim AS runtime

# Copy the built binary from builder
COPY --from=builder /usr/src/app/server/target/release/test_rust_backend /usr/local/bin/server

# Expose the port the server listens on
EXPOSE 3000

# Run the server
CMD ["server"]
