TARGET=asmjs-unknown-emscripten


all: site

site:
	cargo build --target=$(TARGET)
	cp ./target/asmjs-unknown-emscripten/debug/poc.js ./docs/poc.js
