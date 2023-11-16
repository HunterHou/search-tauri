use chrono::offset::Utc;
use chrono::DateTime;
use std::time::SystemTime;

pub fn int_to_size_str(size: i64) -> String {
    if size < 1024 {
        return format!("{}", size);
    } else if size < 1024 * 1024 {
        return format!("{:.2}k", (size as f64) / 1024.00);
    } else if size < 1024 * 1024 * 1024 {
        return format!("{:.2}m", (size as f64) / (1024.00 * 1024.00));
    } else if size < 1024 * 1024 * 1024 * 1024 {
        return format!("{:.2}G", (size as f64) / (1024.00 * 1024.00 * 1024.00));
    } else if size < 1024 * 1024 * 1024 * 1024 * 1024 {
        return format!(
            "{:.2}T",
            (size as f64) / (1024.00 * 1024.00 * 1024.00 * 1024.00)
        );
    }
    return format!(">{:.2}T", 1024);
}

pub fn system_time_to_string(system_time: &SystemTime) -> String {
    let datetime: DateTime<Utc> = (*system_time).into();
    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
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
    let res = String::from(name);
    let vecs: Vec<&str> = res.split("[").collect();
    for str in vecs {
        let item1 = str;
        let index = item1.find("-");
        if index.is_none() && str.len() > 0 {
            let right = String::from(str);
            let rigths: Vec<&str> = right.split("]").collect();
            if rigths.len() > 0 {
                return String::from(*rigths.get(0).unwrap());
            } else {
                return String::from(str);
            }
        }
        continue;
    }
    return String::from("");
}

pub fn movie_type_from_name(name: &str) -> String {
    let v1 = name.find("{{");
    if v1.is_none() {
        return String::from("");
    }
    let start = v1.unwrap() + "{{".len();
    let mut end = name.find("}}").unwrap();
    if end >= name.len() {
        end = name.len()
    }
    if start > end {
        return String::from("");
    }
    return String::from(&name[start..end]);
}

pub fn title_from_name(name: &str) -> String {
    let str = String::from(name);
    let res: Vec<&str> = str.split("]").collect();

    if res.len() > 0 {
        let title = res.get(res.len() - 1);
        if title.is_some() {
            let result = title.unwrap().to_string();
            // println!("{}",&result);
            return result;
        }
    }
    return String::from(name);
}

pub fn vm_png_from_name(name: &str) -> String {
    let mut res = String::from(name);
    res.push_str(".png");
    return res;
}
pub fn vm_git_from_name(name: &str) -> String {
    let mut res = String::from(name);
    res.push_str(".gif");
    return res;
}

pub fn vm_jpg_from_name(name: &str) -> String {
    let mut res = String::from(name);
    res.push_str(".jpg");
    return res;
}

pub fn tagstr_from_name(name: &str) -> String {
    let v1 = name.find("《");
    if v1.is_none() {
        return String::new();
    }
    let start = v1.unwrap() + "《".len();
    let end = match name.find("》") {
        Some(v) => v,
        None => name.len(),
    };
    if start > end {
        return String::new();
    }
    let tag_str = String::from(&name[start..end]);
    return tag_str;
}

pub fn tags_from_name(name: &str) -> Vec<String> {
    let tag_str = tagstr_from_name(name);
    let mut tags = Vec::new();
    let location: Vec<&str> = tag_str.split(",").collect();

    for str in location {
        tags.push(String::from(str))
    }
    tags.retain(|e| e.len() > 0);
    return tags;
}
