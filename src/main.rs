use std::io;

fn main() {
    println!("Привет, русский текст!");

    //
    println!("Please input your guess:");
    let mut guess = String::new();
    //if
    io::stdin().read_line(&mut guess).expect("Error here!");

    println!("You guessed: {}", guess);

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    println!("You value guess + 10: {}", guess + 10);
    //cmp
    match guess.cmp(&100) {
            std::cmp::Ordering::Less => {
                println!("Too small!");
            },
            std::cmp::Ordering::Greater => println!("Too big!"),
            std::cmp::Ordering::Equal => println!("You win!"),
    }

    let guess = 10.25;
    println!("You float: {}", guess);

    //contatnts
    const MAX_POINTS: u32 = 100_000;
    println!("You MAX_POINTS: {}", MAX_POINTS);

    //var
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    let y: f32 = 3.08;
    println!("The value of x is: {} y: {}", x, y); //12

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of 0 tup: {}", tup.0); //500
    let (x, y, z) = tup;
    println!("The value of y of tup: {}", y);

    //arrays
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 20, 3, 4, 5];
    println!("The value of 2 of array: {}, {}", a[1], b[1]);

    //functions
    another_function();
    another_function1(52);
    println!("The value of fun: {}", five());
    println!("The value of fun: {}", plus_one(5));
    println!("The value of fun: {}", plus_two(10));

    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    if number == 3 {
        println!("number was three");
    }

    //let
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number);

    //loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    //while
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    for number in (1..4) {
        println!("{}!", number); //4 not include in range
    }
    println!("LIFTOFF!!!");

    //Ownership
    println!("Ownership");
}

fn another_function() {
    println!("Another function.");
}

fn another_function1(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn plus_two(x: i32) -> i32 {
    return (x + 2);
}
