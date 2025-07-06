run:
	cargo run

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

build-web:
	cd web && npm run build-only

