Adventures in Rust-JavaScript interop land!
===========================================

Check out
[procrat.github.io/rust-asmjs-fiddlings](https://procrat.github.io/rust-asmjs-fiddlings)
for a working version of this code.

If you want to build it yourself:
1. install [nightly Rust with rustup](https://www.rustup.rs),
2. install [emscripten](http://emscripten.org),
3. add the asm.js target with `rustup target add asmjs-unknown-emscripten`,
4. run `make`, and
5. open `docs/index.html` in your browser.
