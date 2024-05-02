#[allow(dead_code)]

enum Temperature {
    Fahrenheit(f32),
    Celsius(f32),
}
fn main() {
    let temperature = Temperature::Fahrenheit(30.0);
    match temperature {
        // match guard to filter temperature above 30 degree
        Temperature::Celsius(c) if c > 30.0_f32 => {
            println!("Temperature in Celsius is above 30 degree: {}", c);
        }
        Temperature::Celsius(c) => {
            println!("Temperature is equal or below 30 degree: {}", c);
        }
        Temperature::Fahrenheit(f) if f > 86_f32 => {
            println!("Temperature in above 86 Fahrenheit: {}", f);
        }
        Temperature::Fahrenheit(f) => {
            println!("Temperature is below or equals 86 Fahrenheit: {}", f);
        }
    }

    let num: u8 = 4;
    match num {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero "),
        _ => println!("Other number"), // can not happen but required
                                       // the compiler can not determine that the match is exhaustive
    }

    bind_main();
    destructure_some();
}

fn age() -> u32 {
    20
}

fn bind_main() {
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I have not celebrated by 1st birthday yet"),
        n @ 1..=12 => println!("Child of age: {}", n),
        n @ 13..=19 => println!("Teen of age: {}", n),
        n => println!("Old person of age: {}", n),
    }
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn destructure_some() {
    match some_number() {
        Some(n @ 42) => println!("Got a value: {}", n),
        Some(n) => println!("Dont worry about {}", n),
        _ => (),
    }
}
