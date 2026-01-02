fn main() {
    /* The code below will print the words Hello World!
    to the screen, and it is amazing 🙃 */
    println!("Hello, world!"); // Print Hello, world!

    let name = "John";
    println!("My first name is: {}", name);

    let mut x = 5; // mut means mutable/changeable
    println!("Before: {}", x);
    x = 10;
    println!("After: {}", x);

    let my_num: i32 = 32; // integer
    let my_double: f64 = 64.1; // float
    let my_letter: char = 'A'; // character
    let my_bool: bool = true; // boolean
    let my_text: &str = "Hello"; // string

    println!("{}", my_num);
    println!("{}", my_double);
    println!("{}", my_letter);
    println!("{}", my_bool);
    println!("{}", my_text);

    const PI: f64 = 3.14;
    println!("{}", PI);

    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    x += 5;
    println!("After += 5: {}", x);

    x -= 2;
    println!("After -= 2: {}", x);

    x *= 2;
    println!("After *= 2: {}", x);

    x /= 3;
    println!("After /= 3: {}", x);

    x %= 4;
    println!("After %= 4: {}", x);

    let y = 10;

    if y > x {
        println!("y: {} is greater than x: {}", y, x);
    }

    let day = 5;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    for i in 1..6 {
        println!("i is: {}", i);
    }

    for i in 1..=6 {
        println!("i is: {}", i);
    }

    fn say_hello() {
        println!("Hello from a function!");
    }

    say_hello();

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }

    greet("John");

    fn add_num(a: i32, b: i32) -> i32 {
        return a + b;
    }

    let sum = add_num(3, 4);
    println!("Sum is: {}", sum);

    let a = String::from("Hello");
    let b = a;
    println!("{}", b);
}
