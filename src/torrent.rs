use serde_bencode;
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    announce: String,
    info: TorrentInfo,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
struct TorrentInfo {
    length: i64,
}

pub fn info(file_name: &str) {
    let file_contents = fs::read(file_name).unwrap();

    let decoded_file_contents: Torrent = serde_bencode::from_bytes(&file_contents).unwrap();

    println!("Tracker URL: {}", decoded_file_contents.announce);
    println!("Length: {}", decoded_file_contents.info.length);
}
