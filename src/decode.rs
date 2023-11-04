use serde_json;
use serde_json::Map;

pub fn decode_bencoded_value(encoded_value: &str) -> (serde_json::Value, usize) {
    let bencode_type = encoded_value.chars().next().unwrap();

    return match bencode_type {
        'd' => decode_dictionary(&encoded_value),
        'l' => decode_list(&encoded_value),
        'i' => decode_integer(&encoded_value),
        _ => decode_string(&encoded_value),
    };
}

fn decode_dictionary(encoded_value: &str) -> (serde_json::Value, usize) {
    // Example: "d3:foo3:bar5:helloi52ee" -> {"foo":"bar", "hello":52}
    assert_eq!(&encoded_value.chars().next().unwrap(), &'d');

    let mut items: Map<String, serde_json::Value> = Map::new();

    let mut index = 1;
    while encoded_value.chars().nth(index).unwrap() != 'e' {
        let (key, key_index_offset) = decode_string(&encoded_value[index..]);
        index += key_index_offset;

        let (value, value_index_offset) = decode_bencoded_value(&encoded_value[index..]);
        index += value_index_offset;

        items.insert(key.as_str().unwrap().to_string(), value);
    }

    return (serde_json::Value::Object(items), index + 1);
}

fn decode_integer(encoded_value: &str) -> (serde_json::Value, usize) {
    // Example: "i52e" -> ("52", 4)
    let index_end = encoded_value.find('e').unwrap();

    let number_string = &encoded_value[1..index_end];
    let number: i64 = number_string.parse().unwrap();

    return (
        serde_json::Value::Number(serde_json::Number::from(number)),
        index_end + 1,
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

fn decode_list(encoded_value: &str) -> (serde_json::Value, usize) {
    assert_eq!(&encoded_value.chars().next().unwrap(), &'l');

    let mut items: Vec<serde_json::Value> = Vec::new();
    let mut index = 1;

    while encoded_value.chars().nth(index).unwrap() != 'e' {
        let (item, length) = decode_bencoded_value(&encoded_value[index..]);
        items.push(item);
        index += length
    }

    return (serde_json::Value::Array(items), index + 1);
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
