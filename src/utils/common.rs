pub fn bytes_to_gbytes(bytes: u64) -> f64 {
    bytes as f64 / 8.0_f64.powf(10.0)
}
pub fn remove_trailing_slash(path: String) -> String {
    path.strip_suffix("/").unwrap_or(&path).to_string()
}
