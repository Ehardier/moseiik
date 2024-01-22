# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
# WORKDIR /app

# Copy the entire project directory into the container
COPY . .

# Install system dependencies if needed (adjust as necessary)
RUN apt-get update && \
    apt-get install -y libssl-dev

# Build the Rust application
RUN cargo build --release

# Expose any necessary ports (if your application is a server)
# EXPOSE 8080

# Set the default command to run your application
#CMD ["./target/release/your_binary_name"]

ENTRYPOINT [ "cargo", "test", "--release", "--" ]

