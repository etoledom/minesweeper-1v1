run:
    cargo run -p minesboomer_server & cargo run -p minesboomer & cargo run -p minesboomer

server:
    cargo run -p minesboomer_server

client:
    -pkill -f "trunk serve" || true
    -lsof -nP -iTCP:3000 -sTCP:LISTEN -t | xargs kill -9 2>/dev/null || true
    rm -rf client/dist
    cd client && trunk serve --port 3000

clean-ports:
    for p in 8080 3000; do pids=$(lsof -nP -iTCP:$p -sTCP:LISTEN -t); if [ -n "$pids" ]; then kill $pids; sleep 1; still=$(lsof -nP -iTCP:$p -sTCP:LISTEN -t); if [ -n "$still" ]; then kill -9 $still; fi; fi; done; lsof -nP -iTCP:8080 -sTCP:LISTEN || true; lsof -nP -iTCP:3000 -sTCP:LISTEN || true

build-web:
    cd client && trunk build --release
    rm -rf server/dist
    cp -r client/dist server/dist
