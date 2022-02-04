
use std::str;
use std::convert::TryInto;
use std::collections::HashMap;

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

    let buf = hex(arg);

    match buf[0] {
        
        1 => {

            let mut res: Vec<String> = Vec::new();

            let mut i: usize = 1;
            
            while i < buf.len() {

                let item_size_buf: Vec<u8> = buf[i..(i + 4)].to_vec();

                i += 4;

                let item_size: usize = u32::from_be_bytes(item_size_buf.try_into().unwrap()) as usize;

                let item_buf: Vec<u8> = buf[i..(i + item_size)].to_vec();

                i += item_size;

                let item: String = str::from_utf8(&item_buf).unwrap().to_string();

                res.push(item)

            }

            res

        },

        _ => panic!("Unsupported Astro Notation Version")

    }

}

pub fn as_hashmap(arg: &str) -> HashMap<String, String> {

    let buf = hex(arg);

    match buf[0] {
        
        1 => {

            let mut res: HashMap<String, String> = HashMap::new();

            let mut i: usize = 1;
            
            while i < buf.len() {

                let map_size_buf: Vec<u8> = buf[i..(i + 4)].to_vec();

                i += 4;

                let map_size: usize = u32::from_be_bytes(map_size_buf.try_into().unwrap()) as usize;

                let map_end_index: usize = i + map_size;

                let key_size_buf: Vec<u8> = buf[i..(i + 4)].to_vec();

                i += 4;

                let key_size: usize = u32::from_be_bytes(key_size_buf.try_into().unwrap()) as usize;

                let key_buf: Vec<u8> = buf[i..(i + key_size)].to_vec();

                i += key_size;

                let val_buf: Vec<u8> = buf[i..map_end_index].to_vec();

                i = map_end_index;

                let key: String = str::from_utf8(&key_buf).unwrap().to_string();

                let val: String = str::from_utf8(&val_buf).unwrap().to_string();

                res.insert(key, val);

            }

            res

        },

        _ => panic!("Unsupported Astro Notation Version")

    }

}

fn hex(arg: &str) -> Vec<u8> {
    
    (2..arg.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&arg[i..i + 2], 16).unwrap())
        .collect()

}
