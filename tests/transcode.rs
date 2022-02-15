// use std::collections::HashMap;
use astro_notation::{encode, decode};

#[test]
fn bool() {
    assert_eq!(true, decode::as_bool(&encode::bool(&true)))
}

#[test]
fn u8() {
    assert_eq!(8_u8, decode::as_u8(&encode::u8(&8_u8)))
}

#[test]
fn str() {
    assert_eq!("hello".to_string(), decode::as_str(&encode::str("hello")))
}

#[test]
fn bytes() {
    assert_eq!(vec![1_u8, 2_u8, 3_u8], decode::as_bytes(&encode::bytes(&vec![1_u8, 2_u8, 3_u8])))
}

#[test]
fn list() {
    let list: Vec<String> = vec!["one".to_string(), "two".to_string(), "three".to_string()];
    assert_eq!(list, decode::as_list(&encode::list(&list)))
}

// #[test]
// fn hashmap() {
//     let hmap: HashMap<String, String> = HashMap::from([
//         ("key_1".to_string(), "val_1".to_string()),
//         ("key_2".to_string(), "val_2".to_string()),
//         ("key_3".to_string(), "val_3".to_string()),
//         ("key_4".to_string(), "val_4".to_string())
//     ]);
//     assert_eq!(hmap, decode::as_hashmap(&encode::hashmap(&hmap)))
// }