dev: cargo-dev copy
build: cargo-rel copy

copy:
	mkdir -p dist
	rm -rf dist/*
	cp -r output/* dist
	cp -r static/* dist

cargo-dev:
	cargo run

cargo-rel:
	cargo run --release