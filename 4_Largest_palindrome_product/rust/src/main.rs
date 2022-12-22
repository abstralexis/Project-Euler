fn main() {
    println!("{:?}", lpp3());
}

fn lpp3() -> u32 {
    let mut products: Vec<u32> = Vec::new();
    for i in 100..999 {
        for j in 100..999 {
            let product: u32 = i * j;
            let strproduct: &str = &product.to_string().to_owned();
            let reversed: String = strproduct.chars().rev().collect::<String>();
            if strproduct == reversed {
                products.push(product);
            }
        }
    }
    return *products.iter().max().unwrap();
}

