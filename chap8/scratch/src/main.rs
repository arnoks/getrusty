fn example1() {
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n
        } else {
            println!(", and is a big number, reduce by two");
            n / 2
        };
    println!("{} -> {}", n, big_n);
}
#[allow(unused)]
fn example2() {
    // adding labels to loops and exit by label allow to break outer loops
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the outer loop");
}

fn example3()  {
    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter == 10 {
            // break return the value of the loop
            break counter * 2;
        }
    }; // semicol is required to close the asiignment statement
    println!("result is {}", result);
    assert_eq!(result, 20 );
}

// while loop fizzbuzz example
fn example4(){
    let mut n = 1;
    // loop while n is less than 101
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
        n += 1;
    }

} 
// for and range example
fn example5(){
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// for iterator using = operator
fn example6(){
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
}

// iter vs into_iter vs iter_mut
fn example7(){
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("names: {:?}", names);
    let names = vec!["Bob", "Frank", "Ferris"];
    // into_iter consumes the vector, so it is not available after the looplo {}", name),                                                                                          │  i_CTRL-@        CTRL-@            insert previously inserted text and stop                                                                 
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // println!("names: {:?}", names); // error: value borrowed here after move

    let mut names = vec!["Bob", "Frank", "Ferris"];
    // iter_mut allows to modify the vector
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names: {:?}", names);
}


fn main(){
    example1();
    example2();
    example3();
    example4();
    example5();
    example6();
    example7();
}
