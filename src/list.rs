use crate::encode;
use crate::decode;

pub fn to_str(input: Vec<Vec<u8>>) -> String {

    let mut output: String = String::new();

    for i in input {

        if output != String::new() {
            output.push(' ')
        }

        output.push_str(&encode::bytes(&i))

    }

    output

}

pub fn from_str(input: &str) -> Vec<Vec<u8>> {

    let mut output: Vec<Vec<u8>> = Vec::new();

    let split_arg: Vec<&str> = input.split(' ').collect();

    for i in split_arg {
        output.push(decode::as_bytes(i))
    }

    output

}