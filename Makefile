run:
	cargo run

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

