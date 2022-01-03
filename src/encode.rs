
use std::fmt::Write;

pub fn u8(val: &u8) -> String {
    hex(&val.to_le_bytes().to_vec())
}

pub fn u16(val: &u16) -> String {
    hex(&val.to_le_bytes().to_vec())
}

pub fn u32(val: &u32) -> String {
    hex(&val.to_le_bytes().to_vec())
}

pub fn u64(val: &u64) -> String {
    hex(&val.to_le_bytes().to_vec())
}

pub fn u128(val: &u128) -> String {
    hex(&val.to_le_bytes().to_vec())
}

pub fn bytes(val: &Vec<u8>) -> String {
    hex(val)
}

pub fn list(input: &Vec<String>) -> String {

    let mut buffer: Vec<u8> = Vec::new();

    input
        .iter()
        .for_each(|x| {

            let str: Vec<u8> = x.to_string().into_bytes();

            let str_length: usize = str.len();

            let mut str_length_size_bytes: Vec<u8> = Vec::new();

            let mut str_length_bytes: Vec<u8> = Vec::new();

            if str_length < 256 {

                str_length_size_bytes = 1_u8.to_le_bytes().to_vec();

                let str_length_as_u8: u8 = str_length as u8;

                str_length_bytes = str_length_as_u8.to_le_bytes().to_vec();

            } else if str_length < 65536 {

                str_length_size_bytes = 2_u8.to_le_bytes().to_vec();

                let str_length_as_u16: u16 = str_length as u16;

                str_length_bytes = str_length_as_u16.to_le_bytes().to_vec();

            } else if str_length < 4294967296 {

                str_length_size_bytes = 4_u8.to_le_bytes().to_vec();

                let str_length_as_u32: u32 = str_length as u32;

                str_length_bytes = str_length_as_u32.to_le_bytes().to_vec();

            } else {

                str_length_size_bytes = 8_u8.to_le_bytes().to_vec();

                let str_length_as_u64: u64 = str_length as u64;

                str_length_bytes = str_length_as_u64.to_le_bytes().to_vec();

            }

            buffer = [buffer.clone(), str_length_size_bytes, str_length_bytes, str].concat();
        
        });

    hex(&buffer)

}

fn hex(bytes: &Vec<u8>) -> String {

    let mut res = String::with_capacity((bytes.len() * 2) + 2);

    res.push_str("0x");

    for &byte in bytes {
        write!(&mut res, "{:02X}", byte).unwrap();
    }

    res

}
