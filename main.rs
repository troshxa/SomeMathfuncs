fn main() {
    println!("Some fib numbers:");
    println!("{}", fib(0));
    println!("{}", fib(1));
    println!("{}", fib(2));
    println!("{}", fib(3));
    println!("{}", fib(4));
    println!("{}", fib(5));

    println!("Some luc numbers:");
    println!("{}", luc(0));
    println!("{}", luc(1));
    println!("{}", luc(2));
    println!("{}", luc(3));
    println!("{}", luc(4));
    println!("{}", luc(5));
}

fn fib(n: i32) -> i32 {
    if n == 0{
        return 0;
    } else if n== 1{
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

fn luc(n: i32) -> i32 {
    if n == 0{
        return 2;
    } else if n== 1{
        return 1;
    }
    luc(n - 1) + luc(n - 2)
}

