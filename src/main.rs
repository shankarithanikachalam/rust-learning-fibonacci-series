fn fibonacci(n: u32, mut i1: u32, mut i2: u32) -> Vec<u32> {
    let mut result = Vec::new();
    for _ in 0..n {
        result.push(i1);
        let temp = i1 + i2;
        i1 = i2;
        i2 = temp;
    }
    return result;
}
fn main() {
    let a = 0;
    let b = 1;

    let series = fibonacci(10, a, b);
    println!("Fibonacci Series: {:?}", series);
}
