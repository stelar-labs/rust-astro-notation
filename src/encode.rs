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

    let mut res: String = String::new();

    for item in arg {

        if res != String::new() {
            res.push_str(" ")
        }

        let item_bytes: Vec<u8> = item.to_string().into_bytes();

        res.push_str(&hex(&item_bytes))

    }

    res

}

// pub fn hashmap(arg: &HashMap<String, String>) -> String {

//     let mut res: String = String::new();

//     for (key, val) in arg {

//         if res != String::new() {
//             res.push_str(" ")
//         }

//         let key_str: String = hex(&key.to_string().into_bytes());

//         let val_str: String = hex(&val.to_string().into_bytes());

//         res.push_str(&format!("{}:{}", key_str, val_str))

//     }

//     res
    
// }

fn hex(arg: &Vec<u8>) -> String {
    
    let mut res = String::new();

    res.push_str("0x");

    for &byte in arg {
        write!(&mut res, "{:02X}", byte).unwrap();
    }

    res

}