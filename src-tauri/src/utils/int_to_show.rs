pub fn int_to_size_str(size: u64) -> String {
    if size < 1024 {
        return format!("{}", size);
    } else if size < 1024 * 1024 {
        return format!("{}k", size / 1024);
    } else if size < 1024 * 1024 * 1024 {
        return format!("{}m", size / (1024 * 1024));
    } else if size < 1024 * 1024 * 1024 * 1024 {
        return format!("{}G", size / (1024 * 1024 * 1024));
    } else if size < 1024 * 1024 * 1024 * 1024 * 1024 {
        return format!("{}T", size / (1024 * 1024 * 1024 * 1024));
    }
    return format!(">{}T", 1024);
}