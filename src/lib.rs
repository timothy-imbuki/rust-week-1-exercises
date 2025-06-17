pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode the hex string to bytes
    let tx_bytes = hex::decode(raw_tx_hex).map_err(|_| "Hex decode error".to_string())?;

    // Ensure there are at least 4 bytes for the version
    if tx_bytes.len() < 4 {
        return Err("Transaction data too short to contain a version".to_string());
    }

    // Extract and convert the first 4 bytes into a u32 (little-endian)
    Ok(u32::from_le_bytes([
        tx_bytes[0],
        tx_bytes[1],
        tx_bytes[2],
        tx_bytes[3],
    ]))
}
