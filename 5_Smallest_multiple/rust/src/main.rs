/*
 *  I have heard that there is some numeric algorithm for finding 
 *  the LCD and such, but I'm simply too silly for it.
 */

fn main() {
    println!("{:?}", lowest_multiple(20));
}

fn lowest_multiple(n: i32) -> i32{
    let mut i: i32 = 0;
    loop {
        i += 1;
        let results: Vec<i32> = filter(&|x| i % x == 0, (1..n+1).collect());
        if results.len() == n.try_into().unwrap() {
            return i;
        }
    }
}

fn filter<T: Copy>(func: &dyn Fn(T) -> bool, arr: Vec<T>) -> Vec<T> {
    let mut new: Vec<T> = Vec::new();
    for item in arr {
        if func(item) {
            new.push(item);
        }
    }
    return new;
}