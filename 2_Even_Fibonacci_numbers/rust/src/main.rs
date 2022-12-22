use std::ops::AddAssign;

fn main() {
    println!("{:?}", sum(filter(&even, fib_lessthan(4_000_000))));
}

fn fib_lessthan(n: u32) -> Vec<u32> {
    let mut arr: Vec<u32> = vec![1, 2];
    let mut i: usize = 1;
    loop {
        if arr[i] + arr[i - 1] > n {
            return arr;
        }
        arr.push(arr[i] + arr[i - 1]);
        i += 1;
    }
}

fn even(item: u32) -> bool { item % 2 == 0 }

fn filter<T: Copy>(func: &dyn Fn(T) -> bool, arr: Vec<T>) -> Vec<T> {
    let mut new: Vec<T> = Vec::new();
    for item in arr {
        if func(item) {
            new.push(item);
        }
    }
    return new;
}

fn sum<T: AddAssign + Default>(arr: Vec<T>) -> T {
    let mut sum: T = T::default();
    for i in arr { sum += i }
    return sum;
}