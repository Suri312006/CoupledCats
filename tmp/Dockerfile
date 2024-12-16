# Start from a base image with the necessary environment (e.g., Node.js, Python, or Rust)
FROM debian:12 

# Set the working directory inside the container
WORKDIR /app

# Copy application files into the container
COPY . .

RUN apt update && apt upgrade
RUN apt install -y curl build-essential pkg-config
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y 
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo build --release

# If you have any environment variables, set them here (optional)
# ENV MY_VAR=my_value

# Expose any ports that your app listens on
EXPOSE 50051 

# Start the application (e.g., for Node.js: npm start, for Rust: ./target/release/your_app_name)
CMD ["./target/release/CoupledCats"] 
