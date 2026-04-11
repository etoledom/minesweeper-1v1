FROM rust:1.94 as builder
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk

WORKDIR /app
COPY . .

# Build WASM client
RUN cd client && trunk build --release

# Copy dist to server
RUN cp -r client/dist server/dist

# Build server
RUN cargo build --release -p minesboomer_server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/minesboomer_server /usr/local/bin/
COPY --from=builder /app/server/dist /dist

CMD ["minesboomer_server"]
