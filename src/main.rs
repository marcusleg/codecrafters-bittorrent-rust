use serde_json;
use std::env;

// Available if you need it!
// use serde_bencode

fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let bencode_type = encoded_value.chars().next().unwrap();

    return match bencode_type {
        'l' => decode_list(&encoded_value),
        'i' => decode_integer(&encoded_value),
        _ => decode_string(&encoded_value),
    };
}

fn decode_list(encoded_value: &str) -> serde_json::Value {
    let mut items: Vec<serde_json::Value> = Vec::new();

    let mut index_start = 1;
    let mut index_end;

    let mut done = false;

    while !done {
        let symbol = encoded_value.chars().nth(index_start).unwrap();

        match symbol {
            'e' => done = true,
            'i' => {
                let index_end = index_start + encoded_value[index_start..].find('e').unwrap();

                let number_string = &encoded_value[index_start + 1..index_end];
                let number: usize = number_string.parse().unwrap();

                items.push(serde_json::Value::Number(serde_json::Number::from(number)));

                index_start = index_end + 1;
            }
            _ => {
                let index_colon = index_start + encoded_value[index_start..].find(':').unwrap();
                let length_string = &encoded_value[index_start..index_colon];
                let length: usize = length_string.parse().unwrap();

                index_end = index_colon + 1 + length;

                let string = &encoded_value[index_colon + 1..index_end];

                items.push(serde_json::Value::String(string.to_string()));

                index_start = index_end;
            }
        }
    }
    return serde_json::Value::Array(items);
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
