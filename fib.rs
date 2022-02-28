fn fib_recur(n: usize) -> [usize; 2] {
    if n == 1 {
        println!("{}: {}", n, 1);
        return [1, 0];
    }
    let [a, b] = fib_recur(n - 1);
    println!("{}: {}", n, a + b);
    [a + b, a]
}

fn main() {
    fib_recur(12);
}
