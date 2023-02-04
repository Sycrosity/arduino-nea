`arduino-nea`
==================

## Usage
If you don't have them already, install [`cargo-generate`] and [`ravedude`]:

```bash
cargo install cargo-generate
cargo install ravedude
```

And change the ravedude port (the -P flag) in `.cargo/config.toml` to be the port of the arduino board plugged into your computer.

Then just run it as you would a normal rust program!

```bash
cargo run
```
-------

Built using:
[`cargo-generate`]: https://github.com/cargo-generate/cargo-generate
[`ravedude`]: https://github.com/Rahix/avr-hal/tree/next/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)
