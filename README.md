# u8-base-converter
`u8-base-converter` is a minimalistic u8 base coverter, capable of converting arbitrary-length <sub>(up to 1024)</sub> numerals of `u8` bases to other `u8` bases.

`u8-base-converter` features *zero* dependencies and is suitable for `no_std` environments.

## Functionality


```rust
// Example Usage

let mut user_id: Numeral = Numeral::new_dec_from_u128(2173619849);
user_id.convert(ALPHANUMERIC);

let hex_key:Numeral = Numeral::new_hex(b"2A3BC512FF")
let binary_key = hex_key.convert(Base::from_radix(2))
```

## Installation
Add `u8-base-converter` to your `Cargo.toml` dependencies:

```toml
[dependencies]
u8-base-converter = "0.1.1"
```

<sub>This project is licensed under the BSD 3-Clause License.</sub>