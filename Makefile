EMCC=emcc

main: main.c libruce
	$(EMCC) main.c js_code.c -lruce -L. -o main.html

libruce:
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/libruce.a .

clean:
	rm js_code.c main.html main.js main.wasm libruce.a
