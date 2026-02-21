#[derive(Debug)]
enum Err {
    EmptyString,
    InvalidChar { c: char, idx: usize },
    NegativeNum,
}

fn ss_to_ui(s: &str) -> Result<u32, Err> {
    if s.is_empty() {
        return Err(Err::EmptyString);
    }

    if s.starts_with('-') {
        return Err(Err::NegativeNum);
    }

    let mut val: u32 = 0;

    for (i, ch) in s.chars().enumerate() {
        if !ch.is_ascii_digit() {
            return Err(Err::InvalidChar { c: ch, idx: i as usize });
        }

        let dig = ch as u32 - '0' as u32;
        val = val * 10 + dig;
    }

    Ok(val)
}

fn main() {
    let s = "4u4";
    
    let num = ss_to_ui(s);

    match num {
        Ok(val) => println!("Parsed {}", val),
        Err(e) => println!("Error {:?}", e)
    }
}
