//
// This file contains the implementation of the magic packet
//

use std::net::{UdpSocket, SocketAddr};

use crate::util;

pub(crate) fn construct_magic_packet(mac_address: [u8; 6]) -> [u8; 102] {
    let mut magic_packet = [0xFFu8; 102]; // Initialize with 0xFF bytes

    // Repeat the MAC address 16 times starting from the 7th byte
    for i in 6..102 {
        magic_packet[i] = mac_address[(i - 6) % 6];
    }
    magic_packet
}

pub(crate) fn create_wol_message() -> std::io::Result<()> {
    // Create a UDP socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // Set the socket to broadcast mode
    socket.set_broadcast(true)?;

     let mac_address_bytes = util::parse_mac_address(util::read_mac_address_from_config().as_ref()).unwrap_or_else(|err| {
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
