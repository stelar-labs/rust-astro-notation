use std::collections::HashMap;
use std::convert::TryInto;
use std::str;

pub fn as_bool(arg: &str) -> bool {
    match arg {
        "0x00" => false,
        "0x01" => true,
        _ => panic!("unsupported string as bool.")
    }
}

pub fn as_str(arg: &str) -> String {
    str::from_utf8(&hex(arg)).unwrap().to_string()
}

pub fn as_u8(i: &str) -> u8 {
    u8::from_be_bytes(hex(i).try_into().unwrap())
}

pub fn as_u16(i: &str) -> u16 {
    u16::from_be_bytes(hex(i).try_into().unwrap())
}

pub fn as_u32(i: &str) -> u32 {
    u32::from_be_bytes(hex(i).try_into().unwrap())
}

pub fn as_u64(i: &str) -> u64 {
    u64::from_be_bytes(hex(i).try_into().unwrap())
}

pub fn as_u128(i: &str) -> u128 {
    u128::from_be_bytes(hex(i).try_into().unwrap())
}

pub fn as_bytes(i: &str) -> Vec<u8> {
    hex(i)
}

pub fn as_list(arg: &str) -> Vec<String> {

    let mut res: Vec<String> = Vec::new();

    let split_arg: Vec<&str> = arg.split(' ').collect();

    for item in split_arg {
        res.push(str::from_utf8(&hex(item)).unwrap().to_string())
    }

    res

}

pub fn as_hashmap(arg: &str) -> HashMap<String, String> {

    let mut res: HashMap<String, String> = HashMap::new();

    let split_arg: Vec<&str> = arg.split(' ').collect();

    for item in split_arg {

        let split_item: Vec<&str> = item.split(':').collect();

        let key: String = str::from_utf8(&hex(split_item[0])).unwrap().to_string();

        let val: String = str::from_utf8(&hex(split_item[1])).unwrap().to_string();

        res.insert(key, val);

    }

    res

}

fn hex(arg: &str) -> Vec<u8> {

    let mut res: Vec<u8> = Vec::new();
    
    (2..arg.len())
        .step_by(2)
        .for_each(|x| res.push(u8::from_str_radix(&arg[x..x + 2], 16).unwrap()));

    res

}
