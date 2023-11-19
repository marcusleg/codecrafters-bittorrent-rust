use serde_bencode;
use sha1::{Digest, Sha1};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
struct Torrent {
    announce: String,
    info: TorrentInfo,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
struct TorrentInfo {
    name: String,
    length: usize,
    #[serde(rename = "piece length")]
    piece_length: usize,
    pieces: serde_bytes::ByteBuf,
}

pub fn info(file_name: &str) {
    let file_contents = fs::read(file_name).unwrap();
    let decoded_file_contents: Torrent = serde_bencode::from_bytes(&file_contents).unwrap();

    let info_hash = calculate_info_hash(&decoded_file_contents.info);

    println!("Tracker URL: {}", decoded_file_contents.announce);
    println!("Length: {}", decoded_file_contents.info.length);
    println!("Info Hash: {}", hex::encode(info_hash));
}

fn calculate_info_hash(torrent_info: &TorrentInfo) -> Vec<u8> {
    let encoded_torrent_info = serde_bencode::to_bytes(torrent_info).unwrap();

    let mut hasher = Sha1::new();
    hasher.update(&encoded_torrent_info);
    return hasher.finalize().to_ascii_lowercase();
}
