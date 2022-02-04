# Rust Astro Notation
A library for transcoding between hexadecimal strings in Astro Notation Format and Native Rust data types.

### Usage

In your `Cargo.toml`:

```

[dependencies]
astro-notation = "2.0.0"

```

In your Rust file:

```

use astro_notation::{encode, decode};

```

### Features
- Represent boolean, strings, integers, bytes, lists & hashmap as hexadecimal strings.
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
| i8 | 🚧 |
| i16 | 🚧 |
| i32 | 🚧 |
| i64 | 🚧 |
| i128 | 🚧 |
| f32 | 🚧 |
| f64 | 🚧 |
| bool | ✅ |
| list | ✅ |
| bytes | ✅ |

### API

`Boolean`

```

let astro_bool: String = encode::bool(true);

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

let list: Vec<String> = vec![
    "one".to_string(),
    "two".to_string(),
    "three".to_string()
];

let astro_list: String = encode::list(&list);

let decoded_list: Vec<String> = decode::as_list(&astro_list);

```

`HashMap`

```

let hmap: HashMap<String, String> = HashMap::from([
    ("key_1".to_string(), "val_1".to_string()),
    ("key_2".to_string(), "val_2".to_string()),
    ("key_3".to_string(), "val_3".to_string()),
    ("key_4".to_string(), "val_4".to_string())
]);

let astro_hmap: String = encode::hashmap(&hmap);

let decoded_hmap: HashMap<String, String> = decode::as_hashmap(&astro_hmap);

```


### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-02-04
