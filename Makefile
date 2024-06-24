.PHONY: start-db

start-db:
	docker run \
		--name postgres \
		-e POSTGRES_PASSWORD=postgres \
		-e PGDATA=/var/lib/postgresql/data/pgdata \
		-v ./pgdata:/var/lib/postgresql/data \
		-p 5432:5432 \
		--rm \
		postgres

start-redis:
	docker run \
		--name redis \
		-p 6379:6379 \
		--rm \
		redis