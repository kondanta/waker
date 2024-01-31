use serde::Deserialize;

#[derive(Deserialize)]
struct MacAddress {
    mac_address: String,
}

// Parse the MAC address from the config file
pub(crate) fn read_mac_address_from_config() -> String {
    let config_file = std::fs::read_to_string("config.json").expect("Unable to read config file");
    let mac_address: MacAddress = serde_json::from_str(&config_file).expect("Unable to parse config file");
    mac_address.mac_address
}

pub(crate) fn parse_mac_address(mac_address: &str) -> Result<[u8; 6], &'static str> {
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