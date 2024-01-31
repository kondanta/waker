use axum::{
    routing::get,
    http::StatusCode,
    Json, Router,
};

use std::net::{UdpSocket, SocketAddr};

use serde::{Deserialize, Serialize};

// Struct that holds the string value of MAC address
#[derive(Deserialize)]
struct MacAddress {
    mac_address: String,
}

#[derive(Serialize)]
struct Response<'a> {
    message: &'a str,
}

// Parse the MAC address from the config file
fn parse_mac_address_from_config() -> String {
    let config_file = std::fs::read_to_string("config.json").expect("Unable to read config file");
    let mac_address: MacAddress = serde_json::from_str(&config_file).expect("Unable to parse config file");
    mac_address.mac_address
}

#[tokio::main]
async fn main() {
    // tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/wol", get(wol));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:9002").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Root endpoint
async fn root() -> &'static str {
    "Root page!"
}

// WoL endpoint
async fn wol() -> (StatusCode, Json<Response<'static>>) {
    create_wol_message().unwrap_or_else(|err| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(Response { message: "Failed to send magic packet!" }));
    });
    (StatusCode::OK, Json(Response { message: "Magic packet sent!" }))
}


fn create_wol_message() -> std::io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Set the socket to broadcast mode
    socket.set_broadcast(true)?;

     let mac_address_bytes = parse_mac_address(parse_mac_address_from_config().as_ref()).unwrap_or_else(|err| {
        panic!("Failed to parse MAC address: {}", err);
    });

    // Construct the magic packet
    let magic_packet = construct_magic_packet(mac_address_bytes);

    // Send the magic packet to the broadcast address
    // Port is 40000, I don't know why?
    let broadcast_addr = "255.255.255.255:40000".parse::<SocketAddr>().unwrap();
    socket.send_to(&magic_packet, broadcast_addr)?;

    Ok(())
}


fn construct_magic_packet(mac_address: [u8; 6]) -> [u8; 102] {
    let mut magic_packet = [0xFFu8; 102]; // Initialize with 0xFF bytes

    // Repeat the MAC address 16 times starting from the 7th byte
    for i in 6..102 {
        magic_packet[i] = mac_address[(i - 6) % 6];
    }
    magic_packet
}

fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], &'static str> {
    let mut bytes = [0u8; 6];

    let parts: Vec<&str> = mac_address.split(':').collect();
    if parts.len() != 6 {
        return Err("Invalid MAC address format");
    }

    for (index, part) in parts.iter().enumerate() {
        let byte_value = match u8::from_str_radix(part, 16) {
            Ok(value) => value,
            Err(_) => return Err("Invalid hexadecimal character in MAC address"),
        };

        bytes[index] = byte_value;
    }

    Ok(bytes)
}
