
pub fn bytes_to_gbytes(bytes: u64) -> f64 {
    bytes as f64 / 8.0_f64.powf(10.0)
}

pub fn slice_string_by_max_value(string: String, max_value: usize) -> String {

    if string.len() >= max_value {
        string[0..max_value].to_string()
    } else {
        string
    }
}

