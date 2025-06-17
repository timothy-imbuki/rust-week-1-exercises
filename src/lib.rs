pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
    // Decode hex string, return exact error message expected by test
    let tx_bytes = hex::decode(raw_tx_hex)
        .map_err(|_| "Hex decode error".to_string())?;

    if tx_bytes.len() < 4 {
        return Err("Transaction data too short to contain a version".to_string());
    }

    let version_bytes: [u8; 4] = tx_bytes[0..4]
        .try_into()
        .map_err(|_| "Failed to extract version bytes".to_string())?;

    let version = u32::from_le_bytes(version_bytes);

    Ok(version)
}
