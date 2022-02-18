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