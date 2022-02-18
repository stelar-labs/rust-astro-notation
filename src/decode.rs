use std::convert::TryInto;
use std::str;

pub fn as_bool(input: &str) -> bool {
    match input {
        "0x00" => false,
        "0x01" => true,
        _ => panic!("{} is an unsupported string as bool.", input)
    }
}

pub fn as_str(input: &str) -> String {
    str::from_utf8(&hex(input)).unwrap().to_string()
}

pub fn as_u8(input: &str) -> u8 {
    u8::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_u16(input: &str) -> u16 {
    u16::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_u32(input: &str) -> u32 {
    u32::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_u64(input: &str) -> u64 {
    u64::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_u128(input: &str) -> u128 {
    u128::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_i8(input: &str) -> i8 {
    i8::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_i16(input: &str) -> i16 {
    i16::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_i32(input: &str) -> i32 {
    i32::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_i64(input: &str) -> i64 {
    i64::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_i128(input: &str) -> i128 {
    i128::from_be_bytes(hex(input).try_into().unwrap())
}

pub fn as_bytes(input: &str) -> Vec<u8> {
    hex(input)
}

fn hex(input: &str) -> Vec<u8> {

    let mut output: Vec<u8> = Vec::new();
    
    (2..input.len())
        .step_by(2)
        .for_each(|x| output.push(u8::from_str_radix(&input[x..x + 2], 16).unwrap()));

    output

}