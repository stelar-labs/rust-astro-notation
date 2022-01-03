
use std::str;
use std::convert::TryInto;
use std::error::Error;
use std::num::ParseIntError;

pub fn as_str(arg: &str) -> Result<String, Box<dyn Error>> {
    let buf: Vec<u8> = hex(arg)?;
    let res: String = str::from_utf8(&buf)?.to_string();
    Ok(res)
}

pub fn as_u8(i: &str) -> Result<u8, Box<dyn Error>> {
    let buffer = hex(i)?;
    let o = u8::from_le_bytes(buffer.try_into().unwrap());
    Ok(o)
}

pub fn as_u16(i: &str) -> Result<u16, Box<dyn Error>> {
    let buffer = hex(i)?;
    let o = u16::from_le_bytes(buffer.try_into().unwrap());
    Ok(o)
}

pub fn as_u32(i: &str) -> Result<u32, Box<dyn Error>> {
    let buffer = hex(i)?;
    let o = u32::from_le_bytes(buffer.try_into().unwrap());
    Ok(o)
}

pub fn as_u64(i: &str) -> Result<u64, Box<dyn Error>> {
    let buffer = hex(i)?;
    let o = u64::from_le_bytes(buffer.try_into().unwrap());
    Ok(o)
}

pub fn as_u128(i: &str) -> Result<u128, Box<dyn Error>> {
    let buffer = hex(i)?;
    let o = u128::from_le_bytes(buffer.try_into().unwrap());
    Ok(o)
}

pub fn as_bytes(i: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let o = hex(i)?; Ok(o)
}

pub fn as_list(i: &str) -> Result<Vec<String>, Box<dyn Error>> {

    let buffer = hex(i)?;

    let mut o: Vec<String> = Vec::new();

    let mut p: usize = 0;

    while p < buffer.len() {

        let str_length_size: u8 = u8::from_le_bytes([buffer[p]]); p += 1;

        match str_length_size {

            1 => {

                let str_length: usize = u8::from_le_bytes([buffer[p]]) as usize; p += 1;

                let str_bytes: Vec<u8> = buffer[p..p + str_length].to_vec(); p += str_length;

                let str: String = str::from_utf8(&str_bytes).unwrap().to_string();

                o.push(str)

            },

            2 => {

                let str_length: usize = u16::from_le_bytes([buffer[p], buffer[p + 1]]) as usize; p += 2;

                let str_bytes: Vec<u8> = buffer[p..p + str_length].to_vec(); p += str_length;

                let str: String = str::from_utf8(&str_bytes).unwrap().to_string();

                o.push(str)

            },

            4 => {

                let str_length: usize = u32::from_le_bytes([buffer[p], buffer[p + 1], buffer[p + 2], buffer[p + 3]]) as usize;

                p += 4;

                let str_bytes: Vec<u8> = buffer[p..p + str_length].to_vec(); p += str_length;

                let str: String = str::from_utf8(&str_bytes).unwrap().to_string();

                o.push(str)
                
            },

            8 => {

                let str_length: usize = u64::from_le_bytes([buffer[p], buffer[p + 1], buffer[p + 2], buffer[p + 3], buffer[p + 4], buffer[p + 5], buffer[p + 6], buffer[p + 7]]) as usize;

                p += 8;

                let str_bytes: Vec<u8> = buffer[p..p + str_length].to_vec(); p += str_length;

                let str: String = str::from_utf8(&str_bytes).unwrap().to_string();

                o.push(str)
                
            },

            _ => ()

        }

    }

    Ok(o)

}

fn hex(srt: &str) -> Result<Vec<u8>, ParseIntError> {
    
    (2..srt.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&srt[i..i + 2], 16))
        .collect()

}
