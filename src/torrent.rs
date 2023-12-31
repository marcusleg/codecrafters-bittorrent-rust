use serde_bencode;
use sha1::{Digest, Sha1};
use std::fs;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: TorrentInfo,
}

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug)]
pub struct TorrentInfo {
    pub name: String,
    pub length: usize,
    #[serde(rename = "piece length")]
    pub piece_length: usize,
    pub pieces: serde_bytes::ByteBuf,
}

pub fn parse(file_name: &str) -> Torrent {
    let file_contents = fs::read(file_name).unwrap();
    return serde_bencode::from_bytes(&file_contents).unwrap();
}

pub fn info(file_name: &str) {
    let decoded_file_contents = parse(file_name);

    let info_hash = calculate_info_hash(&decoded_file_contents.info);

    println!("Tracker URL: {}", decoded_file_contents.announce);
    println!("Length: {}", decoded_file_contents.info.length);
    println!("Info Hash: {}", hex::encode(info_hash));
    println!("Piece Length: {}", decoded_file_contents.info.piece_length);
    println!("Piece hashes:");
    decoded_file_contents
        .info
        .pieces
        .chunks_exact(20)
        .for_each(|hash| println!("{}", hex::encode(hash)));
}

pub fn calculate_info_hash(torrent_info: &TorrentInfo) -> [u8; 20] {
    let encoded_torrent_info = serde_bencode::to_bytes(torrent_info).unwrap();

    let mut hasher = Sha1::new();
    hasher.update(&encoded_torrent_info);
    return hasher.finalize().into();
}
