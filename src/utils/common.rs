pub fn bytes_to_gbytes(bytes: u64) -> f64 {
    bytes as f64 / 8.0_f64.powf(10.0)
}
