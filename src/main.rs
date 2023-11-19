use std::env;

mod client;
mod decode;
mod torrent;

// Available if you need it!
// use serde_bencode

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode::decode_bencoded_value(encoded_value).0;
        println!("{}", decoded_value.to_string());
    } else if command == "peers" {
        client::peers(&args[2]);
    } else if command == "info" {
        torrent::info(&args[2]);
    } else {
        println!("unknown command: {}", args[1])
    }
}
