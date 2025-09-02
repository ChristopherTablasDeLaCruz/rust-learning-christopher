fn fibonacci_rec(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec(n - 2)
    }
}

fn fibonacci_iter(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut prev = 0;      
    let mut curr = 1;      

    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}


fn main() {
    let n = 10;
    println!("The {}th Fibonacci number (recursive) is: {}", n, fibonacci_rec(n));
    println!("The {}th Fibonacci number (iterative) is: {}", n, fibonacci_iter(n));
}