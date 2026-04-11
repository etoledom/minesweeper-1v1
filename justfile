run:
    cargo run -p minesboomer_server & cargo run -p minesboomer & cargo run -p minesboomer

server:
    cargo run -p minesboomer_server

client:
    cd client && trunk serve --port 3000 --proxy-backend=http://localhost:8080/ws

build-web:
    cd client && trunk build --release
    rm -rf server/dist
    cp -r client/dist server/dist
