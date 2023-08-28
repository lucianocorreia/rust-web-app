# server
.PHONY: server
server:
	@cargo watch -q -c -w src/ -w .cargo/ -x "run"

# client
.PHONY: client
client:
	@cargo watch -q -c -w examples/ -x "run --example quick_dev"

# dbup
.PHONY: dbup
dbup:
	@docker run --rm --name pg -p 5432:5432  -e POSTGRES_PASSWORD=welcome  postgres:15
