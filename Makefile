dev:
	cd client && npm i && npm run build && rm -rf ../server/static && mkdir ../server/static && cp -r ./dist/* ../server/static

dep:
	cd server
	docker compose up -d
