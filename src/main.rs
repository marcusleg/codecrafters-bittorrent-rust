use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    // If encoded_value starts with a digit, it's a number
    let Some(bencode_type) = encoded_value.chars().next() else {
        panic!("Bencode string is empty")
    };

    if encoded_value.chars().next().unwrap().is_digit(10) {
        return decode_string(&encoded_value);
    } else if bencode_type.eq(&'i') {
        return decode_integer(&encoded_value);
    } else {
        panic!("Unknown encoded value {}", encoded_value)
    }
}

fn decode_integer(encoded_value: &str) -> serde_json::Value {
    // Example: "i52e" -> "52"
    let number: i64 = encoded_value
        .strip_prefix("i")
        .unwrap()
        .strip_suffix("e")
        .unwrap()
        .parse()
        .unwrap();
    return serde_json::Value::Number(serde_json::Number::from(number));
}

fn decode_string(encoded_value: &str) -> serde_json::Value {
    // Example: "5:hello" -> "hello"
    let colon_index = encoded_value.find(':').unwrap();
    let number_string = &encoded_value[..colon_index];
    let number = number_string.parse::<i64>().unwrap();
    let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
    return serde_json::Value::String(string.to_string());
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
