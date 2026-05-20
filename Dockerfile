FROM rust:1.94 AS builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .

# Build WASM client
RUN cd client && trunk build --release

# Build server
RUN cargo build --release -p minesboomer_server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/minesboomer_server /usr/local/bin/
COPY --from=builder /app/client/dist /dist

CMD ["minesboomer_server"]
