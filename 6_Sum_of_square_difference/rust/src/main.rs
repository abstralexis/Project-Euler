fn main() {
    let n: i32 = 100;
    println!("{:?}", square_sum_upto(n) - sum_square_upto(n));
}

fn sum_square_upto(n: i32) -> i32 {
    (1..n + 1)
        .into_iter()
        .map(|x| x.pow(2))
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

fn square_sum_upto(n: i32) -> i32 {
    (1..n + 1).into_iter().sum::<i32>().pow(2)
}
