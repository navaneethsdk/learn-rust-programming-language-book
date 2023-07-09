fn main() {
    let number = 5;

    if number > 5 {
        println!("greater than 5")
    } else if number < 5 {
        println!("smaller than 5")
    } else {
        println!("number is 5")
    }
    inline_if();
    looping_using_loop(number);
    loop_labeling();
    looping_with_while();
    looping_with_for();
}

fn inline_if() {
    let condition = true;
    // the values that have the potential to be results from each arm of the if must be the same type;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn looping_using_loop(number: i32) {
    let mut counter = 0;
    loop {
        if counter == number {
            break;
        }
        counter += 1;
        println!("**{counter}**");
    }
}

fn loop_labeling() {
    let mut count = 0;
    // Loop labels must begin with a single quote.
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn looping_with_while() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn looping_with_for() {
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
