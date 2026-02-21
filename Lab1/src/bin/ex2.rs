fn n_odd(n: u32) {
    for i in 0..n {
        if i % 2 == 1 {
            println!("{}", i);
        }
    }
}

fn main() {
    n_odd(10);
}
