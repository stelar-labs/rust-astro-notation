use std::fmt::Write;

pub fn bool(input: &bool) -> String {
    match input {
        true => "0x01".to_string(),
        false => "0x00".to_string()
    }
}

pub fn str(input: &str) -> String {
    hex(&input.to_string().into_bytes())
}

pub fn u8(input: &u8) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn u16(input: &u16) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn u32(arg: &u32) -> String {
    hex(&arg.to_be_bytes().to_vec())
}

pub fn u64(input: &u64) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn u128(input: &u128) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn i8(input: &i8) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn i16(input: &i16) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn i32(input: &i32) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn i64(input: &i64) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn i128(input: &i128) -> String {
    hex(&input.to_be_bytes().to_vec())
}

pub fn bytes(input: &Vec<u8>) -> String {
    hex(input)
}

fn hex(input: &Vec<u8>) -> String {
    
    let mut output = String::new();

    output.push_str("0x");

    for &byte in input {
        write!(&mut output, "{:02X}", byte).unwrap();
    }

    output

}