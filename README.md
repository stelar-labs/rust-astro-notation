# Rust Astro Notation
A library for transcoding between Astro Notation Format and Native Rust data types.

### Usage

In your `Cargo.toml`:

```

[dependencies]
astro-notation = "3.1.0"

```

In your Rust file:

```

use astro_notation::{encode, decode};

```

### Features
- Represent boolean, strings, unsigned integers, bytes & lists as strings.
- Transcode between Astro Notation Format and Native Rust data types.

### Support
| Type | Support |
|---|---|
| str | ✅ |
| u8 | ✅ |
| u16 | ✅ |
| u32 | ✅ |
| u64 | ✅ |
| u128 | ✅ |
| i8 | ✅ |
| i16 | ✅ |
| i32 | ✅ |
| i64 | ✅ |
| i128 | ✅ |
| f32 | 🚧 |
| f64 | 🚧 |
| bool | ✅ |
| list | ✅ |
| bytes | ✅ |

### API

`Boolean`

```

let astro_bool: String = encode::bool(&true);

let decoded_bool: bool = decode::as_bool(&astro_bool);

```

`String`

```

let str: String = "hello".to_string();

let astro_str: String = encode::str(&int);

let decoded_str: String = decode::as_u8(&astro_str);

```

`Unsigned Integers`

```

let astro_u8: String = encode::u8(&8_u8);

let decoded_u8: u8 = decode::as_u8(&astro_u8);

```

`Bytes`

```

let bytes: Vec<u8> = vec![1,2,3];

let astro_bytes: String = encode::bytes(&bytes);

let decoded_bytes: Vec<u8> = decode::as_bytes(&astro_bytes);

```

`List`

```

use astro_notation::list;

let list: Vec<Vec<u8>> = vec![
    vec![1,2,3],
    vec![4,5,6],
    vec![7,8,9]
];

let astro_list: String = list::from_bytes(&list);

let decoded_list: Vec<Vec<u8>> = list::as_bytes(&astro_list);

```


### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-18
