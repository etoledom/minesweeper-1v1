run:
    cargo run -p minesboomer_server & cargo run -p minesboomer & cargo run -p minesboomer

server:
    cargo run -p minesboomer_server

client:
    cargo run -p minesboomer

build-web:
    cd client && trunk build --release
    rm -rf server/dist
    cp -r client/dist server/dist
