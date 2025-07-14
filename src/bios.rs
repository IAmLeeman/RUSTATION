use anyhow::Result;

pub fn load_bios(path: &str) -> Result<Vec<u8>>{
    let data = std::fs::read(path)
        .map_err(|e| anyhow::anyhow!("Failed to read BIOS file: {}", e))?;
    Ok(data)
}