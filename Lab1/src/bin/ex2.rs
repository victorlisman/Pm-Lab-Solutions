fn n_odd(n: u32) {
    for i in 0..n {
        let odd = 2 * i + 1;
        println!("{}", odd);
    } 
}

fn main() {
    n_odd(10);
}
