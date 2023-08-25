# server
.PHONY: server
server:
	@cargo watch -q -c -w src/ -w .cargo/ -x "run"


# client
.PHONY: client
client:
	@cargo watch -q -c -w examples/ -x "run --example quick_dev"

