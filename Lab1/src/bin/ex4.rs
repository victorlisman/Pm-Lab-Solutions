fn first_4_chars(v: &Vec<String>) -> Option<String> {
    for s in v {
        if s.len() > 4 {
            return Some(s.to_string());
        }
    }

    return None
}

fn main() {
    let v = vec!["ab".to_string(), "abcd".to_string(), "abbb".to_string(), "aaaaa".to_string()];

    let res = first_4_chars(&v);

    match res {
        Some(res) => println!("{}", res),
        None => println!("No element with at least 4 chars!")
    }
}
