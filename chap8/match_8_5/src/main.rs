fn main() {
    match_literals();
    match_deconst_tuples();
    match_arrays_and_slices();
    match_enums();
    match_pointers_and_ref();
    let mypi_1 = approximate_pi();
    println!("pi: {}", mypi_1);
    let mypi_2 = approximate_pi_recursive();
    println!("pi: {}", mypi_2);
    // print pi as reference
    println!("pi: {}", std::f64::consts::PI);
}

fn match_literals() {
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),

        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let bolean = true;
    println!(
        "{}",
        match bolean {
            true => "True",
            false => "False",
        }
    );
}

fn match_deconst_tuples() {
    // let triple = (5, -2, 4);
    let triple = (1, -2, 4);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is `{:?}`, and `z` is `{:?}`", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("first is `3` and last is `4` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn match_arrays_and_slices() {
    let array = [43, -2, 7];
    match array {
        [0, second, third] => println!("array[0] = 0 array[1]: {}, array[2]: {}", second, third),
        [1, _, third] => println!("array[0]: 1, array[2] {}, the array[1] was ignored", third),
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all others ones were ignored",
            second
        ),
        // put the data into another array
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {}, and the rest {:?}",
            second, tail
        ),
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle ={:?}, array[2] = {}",
            first, middle, last
        ),
    }
}

#[derive(Debug)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn match_enums() {
    let color = Color::RGB(122, 17, 40);
    match color {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
        Color::RGB(r, g, b) => println!("r : {}, g : {}, b : {}", r, g, b),
        Color::HSV(h, s, v) => println!("h : {}, s : {}, v : {}", h, s, v),
        Color::HSL(h, s, l) => println!("h : {}, s : {}, l : {}", h, s, l),
        Color::CMY(c, m, y) => println!("c : {}, m : {}, y : {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("c : {}, m : {}, y : {}, k : {}", c, m, y, k),
    }
}

fn match_pointers_and_ref() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }
}

// add function to approximate pi value
fn approximate_pi() -> f64 {
    22.0 / 7.0
}

// add second function to approximate pi value using a recursive algorithm
fn approximate_pi_recursive() -> f64 {
    let mut pi = 0.0;
    let mut i = 1;
    let mut j = 1;
    while i < 1000000 {
        pi += j as f64 / i as f64;
        i += 2;
        j = -j;
    }
    pi * 4.0
}

// add test for approximate_pi function
#[test]
fn test_approximate_pi() {
    let mypi = approximate_pi();
    println!("pi: {}", mypi);
    assert_eq!(mypi, 3.142857142857143);
}

#[test]
fn test_approximate_pi_recursive() {
    let mypi = approximate_pi_recursive();
    println!("pi: {}", mypi);
    assert_eq!(mypi, 3.142857142857143);
}

fn destruct_structs() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {}", b, y),
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
    // Destructuring w/o matching
    let faa = Foo { x: (1, 2), y: 3 };
    let Foo { x: x0, y: y0 } = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");
    // Nested structs destructuring
    struct Bar {
        foo: Foo,
    }
    let bar = Bar { foo: faa };
    let Bar {
        foo: Foo {
            x: nested_x,
            y: nested_y,
        },
    } = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
