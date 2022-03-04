fn main() {
    let a = 1;
    let b = 2;
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    if !inspect(&arg) {
        change(&mut arg);
    }

    println!("Compare that with: {}", arg);

    if eat(arg) {
        println!("Might be bananas");
     } else {
        println!("Not bananas");
     }

    // Challenge: Write a function "add" that takes *references* to two integer arguments,
    // dereferences them and adds them together, and returns the result.

    println!("1 + 2 = {}", add(a, b));
    println!("1 + 2 = {}", add_ref(&a, &b));
}

/*
    Dereferencing a pointer...
    A pointer by itself only points at an object placed somewhere in memory.
    There’s not much one can do with such pointer by itself since it’s just
    an opaque address in memory (Simplifying a bit).
    To get to the value that a pointer points at one needs to dereference
    the pointer.
 */
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn add_ref(x: &i32, y: &i32) -> i32 {
    x + y
}

fn change(expr: &mut String) {
    expr.push_str("s");
}

// Since we don't pass reference, it consumes the value in "expr"
fn eat(expr: String) -> bool {
    expr.starts_with("b") && expr.contains("a")
}

fn inspect(expr: &String) -> bool {
    if expr.ends_with("s") {
        println!("{} is originally plural.", expr);
        true
    } else {
        println!("{} is originally singular.", expr);
        false
    }
}
