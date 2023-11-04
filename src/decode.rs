use serde_json;

pub fn decode_bencoded_value(encoded_value: &str) -> serde_json::Value {
    let bencode_type = encoded_value.chars().next().unwrap();

    return match bencode_type {
        'l' => decode_list(&encoded_value).0,
        'i' => decode_integer(&encoded_value).0,
        _ => decode_string(&encoded_value).0,
    };
}

fn decode_list(encoded_value: &str) -> (serde_json::Value, usize) {
    let mut items: Vec<serde_json::Value> = Vec::new();
    let mut index_start = 1;
    let mut done = false;

    while !done {
        let symbol = encoded_value.chars().nth(index_start).unwrap();

        match symbol {
            'e' => done = true,
            'l' => {
                let (list, length) = decode_list(&encoded_value[index_start..]);
                items.push(list);
                index_start += length + 1;
            }
            'i' => {
                let (number, length) = decode_integer(&encoded_value[index_start..]);
                items.push(number);
                index_start += length + 1;
            }
            _ => {
                let (string, length) = decode_string(&encoded_value[index_start..]);
                items.push(string);
                index_start += length;
            }
        }
    }

    return (serde_json::Value::Array(items), index_start);
}

pub fn decode_integer(encoded_value: &str) -> (serde_json::Value, usize) {
    // Example: "i52e" -> ("52", 4)
    let index_end = encoded_value.find('e').unwrap();

    let number_string = &encoded_value[1..index_end];
    let number: i64 = number_string.parse().unwrap();

    return (
        serde_json::Value::Number(serde_json::Number::from(number)),
        index_end,
    );
}

#[cfg(test)]
mod tests {
    use crate::decode::decode_integer;

    #[test]
    fn test_decode_integer() {
        let test_cases = [("i52e", 52), ("i-52e", -52), ("i606e", 606)];

        for test_case in test_cases {
            let input = test_case.0;
            let expected_value = serde_json::Value::Number(serde_json::Number::from(test_case.1));
            let expected_index_offset = test_case.0.len() - 1;

            assert_eq!(
                decode_integer(input),
                (expected_value, expected_index_offset)
            )
        }
    }
}

fn decode_string(encoded_value: &str) -> (serde_json::Value, usize) {
    // Example: "5:hello" -> ("hello", 7)
    let index_colon = encoded_value.find(':').unwrap();

    let length_string = &encoded_value[..index_colon];
    let length: usize = length_string.parse().unwrap();

    let index_end = index_colon + 1 + length;

    let string = &encoded_value[index_colon + 1..index_end];

    return (serde_json::Value::String(string.to_string()), index_end);
}
