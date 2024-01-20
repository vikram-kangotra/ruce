EMCC=emcc

main: main.c libruce
	$(EMCC) main.c output.c -lruce -L. -o main.html

libruce:
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/libruce.a .

clean:
	rm output.c main.html main.js main.wasm libruce.a
