# Rust Astro Notation
A library for transcoding between hexadecimal strings in Astro Notation Format and Native Rust data types.

### Usage

In your `Cargo.toml`:

```

[dependencies]
astro-notation = "1.0.0"

```

In your Rust file:

```

use astro-notation::{encode, decode};

```

### Features
- Represent integers, bytes & lists as hexadecimal strings.
- Transcode between Astro Notation Format and Native Rust data types.

### Support
| Type | Support |
|---|---|
| u8 | ✅ |
| u16 | ✅ |
| u32 | ✅ |
| u64 | ✅ |
| u128 | ✅ |
| i8 | 🚧 |
| i16 | 🚧 |
| i32 | 🚧 |
| i64 | 🚧 |
| i128 | 🚧 |
| f32 | 🚧 |
| f64 | 🚧 |
| bool | 🚧 |
| list | ✅ |
| bytes | ✅ |

### API

`Unsigned Integers`

```

let int: u8 = 1;

let astro_notation_u8: String = encode::u8(&int);

let decoded_u8: u8 = decode::as_u8(&astro_notation_u8)?;

```

`Bytes`

```

let bytes: Vec<u8> = vec![1,2,3];

let astro_notation_bytes: String = encode::bytes(&bytes);

let decoded_bytes: Vec<u8> = decode::as_bytes(&astro_notation_bytes)?;

```

`List`

```

let list: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];

let astro_notation_list: String = encode::list(&list);

let decoded_list: Vec<String> = decode::as_list(&astro_notation_list)?;

```

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-01-03
