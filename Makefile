build-release:
	cargo build --release

release-mac: build-release
	strip target/release/hors
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hors-mac.tar.gz ./hors
	ls -lisah ./release/hors-mac.tar.gz

release-win: build-release
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hors-win.tar.gz ./hors.exe

release-ubuntu: build-release
	strip target/release/hors
	mkdir -p release
	tar -C ./target/release/ -czvf ./release/hors-ubuntu.tar.gz ./hors
	ls -lisah ./release/hors-ubuntu.tar.gz

release-linux-musl: 
	cargo build --release --target=x86_64-unknown-linux-musl
	strip target/x86_64-unknown-linux-musl/release/hors
	mkdir -p release
	tar -C ./target/x86_64-unknown-linux-musl/release/ -czvf ./release/hors-linux-musl.tar.gz ./hors
