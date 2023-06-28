.PHONY: migrate.local

migrate-local:
		echo "EdgeDB version" && edgedb -V
		edgedb migration create
		edgedb migrate