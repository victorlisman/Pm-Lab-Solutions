enum Currency {
    Ron(f32),
    Dollar(f32),
    Euro(f32),
    Pound(f32),
    Bitcoin(f32)
}

impl Currency {
    fn to_ron(&self) -> f32 {
        match self {
            Currency::Ron(amm) => * amm,
            Currency::Dollar(amm) => amm * 4.5,
            Currency::Euro(amm) => amm * 5.0,
            Currency::Pound(amm) => amm * 6.0,
            Currency::Bitcoin(amm) => amm * 100_000.0,
        }
    }

    fn total_in_ron(curr: &[Currency]) -> f32 {
        curr.iter().map(|x| x.to_ron()).sum()
    }
}


fn main() {
   let v = vec![Currency::Ron(100.0), Currency::Dollar(550.0), Currency::Euro(1.0), Currency::Pound(0.0), Currency::Bitcoin(0.0001)]; 
   let total = Currency::total_in_ron(&v);
   println!("Total in ron {}", total);
}
