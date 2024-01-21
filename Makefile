EMCC=emcc

main: main.c libruce.a
	$(EMCC) main.c ruce_out/*.c -lruce -L. -o main.js

libruce.a:
	cargo build --target wasm32-unknown-emscripten
	cp target/wasm32-unknown-emscripten/debug/libruce.a .

clean:
	rm ruce_out -rf
	rm main.js main.wasm libruce.a
