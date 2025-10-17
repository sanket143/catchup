watch:
	cargo lambda watch

web-dev:
	cd web && npm run dev

migrate:
	sqlx migrate run

migrate-revert:
	sqlx migrate revert

deploy:
	cargo lambda build --release --target aarch64-unknown-linux-gnu --output-format zip && cd aws && npx cdk bootstrap && npx cdk deploy && cd ..

build-web:
	cd web && rm -rf dist && npm run build-only

