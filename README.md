# Rust Warp Docker tutorial

This repository mainly is an example of how to structure a Dockerfile for a Rust backend application. 

On my blog I explain the steps that went into creating the Dockerfile and explains the commands used in general: (Docker image for Rust backend: learn how to)[https://tms-dev-blog.com/lean-docker-image-for-rust-backend/]

## Running the application

First build the docker image:

```bash
docker build -t rust-warp-docker:latest .
```

Then run the Docker container with the image:

```bash
docker run -p 3000:5000 -e INSTANCE_NAME=docker1 -d rust-warp-docker
```

The `INSTANCE_NAME` environment variable value is used in a welcome message displayed on the page served by the backend.
