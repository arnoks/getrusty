fn main() {
    match_literals();
    match_deconst_tuples();
}

fn match_literals(){
    let number = 13;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),

        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let bolean = true;
    println!("{}", match bolean {
        true => "True",
        false => "False"
    });
}

fn match_deconst_tuples(){
    let triple = (5, -2, 4);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is `{:?}`, and `z` is `{:?}`", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (..,2) => println!("last is `2` and the rest doesn't matter"),
        (3,..,4) => println!("first is `3` and last is `4` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
}

fn match_arrays_and_slices(){
    let x = 1;
    match x {
        [a, b] => println!("a: {}, b: {}", a, b),
        [a, ..] => println!("a: {}", a),
        _ => println!("It doesn't matter what they are"),
    }
}
