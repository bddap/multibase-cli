# Multibase converter

Command line tool for converting {to,from,between}
[multibase](https://tools.ietf.org/html/draft-multiformats-multibase-00) encodings.

Uses the [multibase crate](https://crates.io/crates/multibase).

## Usage

```bash
echo -n "hey" | multibase base2
# 011010000110010101111001

multibase decode 011010000110010101111001
# hey

echo -n 011010000110010101111001 | multibase decode
# hey
```

Multibase **does** encode newlines.

```bash
echo hey | multibase base58btc
# z3fmoA9

echo -n hey | multibase base58btc
# zc4oi
```

Multibase is strict about the input it accepts. It won't decode newlines unless they are valid
parts of the input encoding.

```bash
# this will fail because echo appends a '\n' character to its output
echo 011010000110010101111001 | multibase decode

# try one of these instead
echo -n 011010000110010101111001 | multibase decode
printf "%s" 011010000110010101111001 | multibase decode
multibase decode 011010000110010101111001
```

## Known issues

Multibase encodings with padding are not yet supported. To make them supported,
[rust-multibase](https://github.com/multiformats/rust-multibase) must push recent changes to
crates.io. https://github.com/multiformats/rust-multibase/issues/12

The `identity` encoding is not yet supported.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
