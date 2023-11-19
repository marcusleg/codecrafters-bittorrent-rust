use crate::torrent;

#[derive(serde::Serialize, Debug)]
struct TrackerRequest {
    peer_id: String,
    port: u16,
    uploaded: usize,
    downloaded: usize,
    left: usize,
    compact: u8,
}

#[derive(serde::Deserialize, Debug)]
struct TrackerResponse {
    interval: usize,
    peers: Peers,
}

#[derive(Debug)]
struct Peers(Vec<std::net::SocketAddrV4>);

struct PeersVisitor;

impl<'de> serde::de::Visitor<'de> for PeersVisitor {
    type Value = Peers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a byte array of size 6, the first 4 bytes are a IPv4 address and the last 2 bytes are a port number")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() % 6 != 0 {
            return Err(E::invalid_length(v.len(), &"length must be divisible by 6"));
        }

        let peers: Vec<std::net::SocketAddrV4> = v
            .chunks_exact(6)
            .map(|bytes| {
                std::net::SocketAddrV4::new(
                    std::net::Ipv4Addr::from([bytes[0], bytes[1], bytes[2], bytes[3]]),
                    u16::from_be_bytes([bytes[4], bytes[5]]),
                )
            })
            .collect();
        return Ok(Peers(peers));
    }
}

impl<'de> serde::Deserialize<'de> for Peers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(PeersVisitor)
    }
}

pub fn peers(file_name: &str) {
    let torrent = torrent::parse(file_name);
    let info_hash = torrent::calculate_info_hash(&torrent.info);

    let tracker_request = TrackerRequest {
        peer_id: String::from("00112233445566778899"),
        port: 6881,
        uploaded: 0,
        downloaded: 0,
        left: torrent.info.length,
        compact: 1,
    };

    let mut url = reqwest::Url::parse(&torrent.announce).unwrap();
    let url_parameters = serde_urlencoded::to_string(&tracker_request).unwrap();
    url.set_query(Some(&url_parameters));
    let url = format!(
        "{}&info_hash={}",
        url.as_str(),
        urlencode_info_hash(&info_hash),
    );

    let response = reqwest::blocking::get(url).unwrap();
    let response = response.bytes().unwrap();
    let response: TrackerResponse = serde_bencode::from_bytes(&response).unwrap();

    for peer in response.peers.0 {
        println!("{}:{}", peer.ip(), peer.port())
    }
}

fn urlencode_info_hash(info_hash: &[u8; 20]) -> String {
    let mut encoded = String::with_capacity(3 * info_hash.len());

    for &byte in info_hash {
        encoded.push('%');
        encoded.push_str(&hex::encode(&[byte]));
    }

    return encoded;
}
