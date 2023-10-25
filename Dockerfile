# step 1: build the rust application
FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml ./
RUN mkdir src
COPY src/ src/

RUN cargo build --release

# step 2: create the runtime image
FROM ubuntu:latest

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl-dev

COPY --from=builder /app/target/release/rocket . 

EXPOSE 8000

CMD ["./rocket"] 
