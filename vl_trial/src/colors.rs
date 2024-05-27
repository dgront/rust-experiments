/// Converts RGB components to a hexadecimal color string.
pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String {
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

/// Converts a hexadecimal color string to RGB components.
pub fn hex_to_rgb(hex: &str) -> Result<(u8, u8, u8), &'static str> {
    if hex.len() != 7 || !hex.starts_with('#') {
        return Err("Invalid hex color format");
    }

    let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
    let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
    let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

    Ok((r, g, b))
}

/// Darkens a hexadecimal color by a given fraction.
pub fn darker(color_hex: &str, fraction: f32) -> Result<String, &'static str> {
    if fraction < 0.0 || fraction > 1.0 {
        return Err("Fraction must be between 0.0 and 1.0");
    }

    let (r, g, b) = hex_to_rgb(color_hex)?;

    let new_r = (r as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;
    let new_g = (g as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;
    let new_b = (b as f32 * (1.0 - fraction)).clamp(0.0, 255.0) as u8;

    Ok(rgb_to_hex(new_r, new_g, new_b))
}