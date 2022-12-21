fn main() {
    println!("{:?}", sum_u32(filter_u32(&even, fib_lessthan(4_000_000))));
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

fn filter_u32(func: &dyn Fn(u32) -> bool, arr: Vec<u32>) -> Vec<u32> {
    let mut new: Vec<u32> = Vec::new();
    for item in arr {
        if func(item) {
            new.push(item);
        }
    }
    return new;
}

fn sum_u32(arr: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;
    for i in arr { sum += i }
    return sum;
}