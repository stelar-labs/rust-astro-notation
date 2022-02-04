
use std::collections::HashMap;
use std::fmt::Write;

pub fn bool(arg: &bool) -> String {
    match arg {
        true => "0x01".to_string(),
        false => "0x00".to_string()
    }
}

pub fn str(arg: &str) -> String {
    hex(&arg.to_string().into_bytes())
}

pub fn u8(arg: &u8) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn u16(arg: &u16) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn u32(arg: &u32) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn u64(arg: &u64) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn u128(arg: &u128) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn bytes(arg: &Vec<u8>) -> String {
    hex(arg)
}

pub fn list(arg: &Vec<String>) -> String {

    let mut buf: Vec<u8> = vec![1];

    for item in arg {

        let item_buf: Vec<u8> = item.clone().into_bytes();

        let item_len: u32 = item_buf.len() as u32;

        buf = [buf, item_len.to_be_bytes().to_vec(), item_buf].concat();

    }

    hex(&buf)

}

pub fn hashmap(arg: &HashMap<String, String>) -> String {

    let mut buf: Vec<u8> = vec![1];

    for (key, val) in arg {

        let key_buf: Vec<u8> = key.clone().into_bytes();

        let key_len: u32 = key_buf.len() as u32;

        let mut key_val_buf: Vec<u8> = key_len.to_be_bytes().to_vec();

        key_val_buf = [key_val_buf, key_buf].concat();

        let val_buf: Vec<u8> = val.clone().into_bytes();

        key_val_buf = [key_val_buf, val_buf].concat();

        let key_val_len: u32 = key_val_buf.len() as u32;

        buf = [buf, key_val_len.to_be_bytes().to_vec(), key_val_buf].concat()

    }

    hex(&buf)
    
}

fn hex(arg: &Vec<u8>) -> String {
    
    let mut res = String::new();

    res.push_str("0x");

    for &byte in arg {
        write!(&mut res, "{:02X}", byte).unwrap();
    }

    res

}
