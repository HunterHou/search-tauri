use std::{borrow::BorrowMut, vec};

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

pub fn code_from_name(name: &str) -> String {
    let res = String::from(name);
    let vecs: Vec<&str> = res.split("[").collect();
    let mut code: String = String::new();
    for str in vecs {
        let item1 = str;
        match item1.find("-") {
            Some(v) => v,
            None => continue,
        };
        code = String::from(str);
    }
    let codes: Vec<&str> = code.split("]").collect();
    let code = match codes.get(0) {
        Some(v) => String::from(*v),
        None => String::from(""),
    };
    return code;
}

pub fn actress_from_name(name: &str) -> String {
    return String::from(name);
}

pub fn movie_type_from_name(name: &str) -> String {
    let start = name.find("《").unwrap();
    let mut end = name.find("》").unwrap();
    if end >= name.len() {
        end = name.len()
    }
    return String::from(&name[start..end]);
}

pub fn title_from_name(name: &str) -> String {
    return String::from(name);
}

pub fn vm_png_from_name(name: &str) -> String {
    let mut res = String::from(name);
    res.push_str(".png");
    return res;
}

pub fn vm_jpg_from_name(name: &str) -> String {
    let mut res = String::from(name);
    res.push_str(".jpg");
    return res;
}

pub fn tags_from_name(name: &str) -> Vec<String> {
    let start = name.find("{{").unwrap();
    let mut end = name.find("}}").unwrap();
    if end >= name.len() {
        end = name.len()
    }
    let tag_str = String::from(&name[start..end]);
    let mut tags = Vec::new();
    let location: Vec<&str> = tag_str.split(",").collect();
    for str in location {
        tags.push(String::from(str))
    }
    return tags;
}
