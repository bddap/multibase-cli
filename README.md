# Multibase converter

## Usage

```bash
echo -n "hey" | multibase base2
>> 011010000110010101111001

multibase decode 011010000110010101111001
>> hey

echo -n 011010000110010101111001 | multibase decode
>> hey
```

Multibase **does** encode newlines.

```bash
echo hey | multibase base58btc
>> z3fmoA9

echo -n hey | multibase base58btc
>> zc4oi
```

## Known issues

Multibase encodings with padding are not yet supported. To make them supported,
[rust-multibase](https://github.com/multiformats/rust-multibase) must push recent changes to
crates.io. https://github.com/multiformats/rust-multibase/issues/12
