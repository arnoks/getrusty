fn main() {
    let optional = Some(7);
    let optional_none: Option<i32> = None;

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
        }
        _ => {}
    }
    // Same as above
    if let Some(i) = optional_none {
        println!("Same with if let `{:?}`", i);
    } else {
        println!("No value");
    }
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't match a number. Let's go with a letter!");
    }
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }

    il2_main();
}

enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

fn il2_main() {
    println!("################## il2_main() starts here ####################");
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Baz = b {
        println!("b is foobaz");
    }
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
    if let Foo::Bar = b {
        println!("b is foobar");
    }
}

#[test]
fn il_challenge() {
    enum Foo {
        Bar,
        Baz,
    }
    let a = Foo::Bar;
    let b = Foo::Baz;
    if let Foo::Bar = a {
        println!("a is foobar");
    }
    if let Foo::Bar = b {
        println!("a is not foobar");
    }
}
