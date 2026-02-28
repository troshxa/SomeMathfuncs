fn main() {
    println!("{}", fib(0));
    println!("{}", fib(1));
    println!("{}", fib(2));
    println!("{}", fib(3));
    println!("{}", fib(4));
    println!("{}", fib(5));
}

fn fib(n: i32) -> i32 {
    if n == 0{
        return 0;
    } else if n== 1{
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn fibe(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}