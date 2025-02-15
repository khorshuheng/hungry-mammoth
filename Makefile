.SHELLFLAGS = -e -c
SERVER_HOST=localhost
SERVER_PORT=8000

static:
	cd client && npm i && npm run build && rm -rf ../server/static && mkdir ../server/static && cp -r ./dist/* ../server/static

lint:
	cd server; cargo clippy -- -D warnings

dep:
	cd server; docker compose down
	cd server; docker compose up --wait
	cd server; cargo sqlx migrate run

start-server: stop-server dep
	cd server; nohup cargo run > server.log 2>&1 & echo $$! > server.pid
	for i in {1..10}; do curl -s -o /dev/null -w "%{http_code}" http://${SERVER_HOST}:${SERVER_PORT}/api/health | grep -q "200" && break || sleep 1; done; if [ $$i -eq 10 ]; then echo "Server did not start in time" && exit 1; fi

stop-server:
	cd server; if [ -f server.pid ]; then kill `cat server.pid` && rm server.pid; fi

e2e: start-server
	cd client; npm test
	cd server; if [ -f server.pid ]; then kill `cat server.pid` && rm server.pid; else true; fi
