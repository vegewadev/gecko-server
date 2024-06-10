# Gecko Server REST API

This Rust project is designed to run on a server and provide a RESTful API for interacting with analytics data stored in a MongoDB database.

## Prerequisites

Before running this project, ensure that you have the following prerequisites installed:

- Rust programming language (latest stable version)
- MongoDB (either locally or a remote instance)

## Installation

1. Clone the repository:

```
git clone https://github.com/vegewadev/gecko-server.git
```

2. Change to the project directory:

```
cd gecko-server
```

3. Build the project:

```
cargo build --release
```

## Running the Project

1. Set the required environment variables:

Replace the `CONNECTION_STRING` value with the appropriate connection string for your MongoDB instance.

2. Run the compiled binary:

```
./target/release/gecko-server
```

## Configuration

The project can be configured using environment variables:

- `CONNECTION_STRING`: The connection string for the MongoDB instance (required).

## License

This project is licensed under the [MIT License](LICENSE).

# Using Docker (amd64 only)

You can run this project using Docker by pulling the pre-built image from GHCR:

```
docker pull ghcr.io/vegewadev/gecko-server:latest
```

Then, run the container with the necessary environment variables:

```
docker run -e CONNECTION_STRING="mongodb://mongo:27017/" -e RUST_BACKTRACE=1 -p 8000:8000 ghcr.io/vegewadev/gecko-server
```

Replace `CONNECTION_STRING` with the appropriate connection string for your MongoDB instance.

The container will expose the Gecko Server on port 8000. Adjust the port mapping (`-p 8000:8000`) if needed.

## Configuration

The project can be configured using environment variables:

- `CONNECTION_STRING`: The connection string for the MongoDB instance (required).
