fn first_even(a: &[u32]) -> Option<&u32> {
    a.iter().find(|&&x| x % 2 == 0)
}

fn main() {
    let arr = [1, 2, 3, 4, 3, 3, 3, 3];

    let res = first_even(&arr[3..]);
    match res {
        Some(res) => println!("{}", res),
        None => println!("No even num!")
    }
}
