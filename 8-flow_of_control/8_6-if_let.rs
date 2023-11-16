enum Foo {
    Bar,
    Baz,
    Qux(u32),
}

enum Faz {
    Bar,
}

/*
 *  for some use cases, when matching enims, match is awkward, for exmaple:
 */

fn main() {
    // Make `optional` of type `Option<i32>`
    let optional = Some(7);

    match optional {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);
            // ^ Needed 2 indentations just so we could destructure
            // `i` from the option.
        }
        _ => {} // ^ Required because `match` is exhaustive. Doesn't it seem
                // like wasted space?
    };

    /*
     *  if let is cleaner for this use case and in addition allows various falue options to be
     *  specified
     */

    // all have type 'option<i32>'
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    //the if let construct reads: "if 'let' destructures 'number' into 'Some(i)', evaluate the
    //block
    if let Some(i) = number {
        println!("matched {:?}", i);
    }

    // if you need to specify a falue, then use an else:
    if let Some(i) = letter {
        println!("matched {:?}", i);
    } else {
        //destructure failed, change to the failure case
        println!("didn't match a number, let's go witha  letter!");
    }

    //provide an altered failing condition
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("matched {:?}", i);
        //destructure failed.
        //evaluate an 'else if' condition to see if the alternate failure branch should be taken
    } else if i_like_letters {
        println!("didn't match a number, let's go with a letter!");
    } else {
        //the condition evaluated false, this branch is the default:
        println!("i don't like letters, let's go with an emoticon :)");
    }

    /*
     *  in the same way, if let can be used to match any enum value:
     */
    //create expression varibales
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    //variable a matches Foo::Bar
    if let Foo::Bar = a {
        println!("a is foobar");
    }

    // variable b does not match Foo::Bar
    if let Foo::Bar = b {
        println!("b is foobar");
    }

    //variable c matches Foo::qux which has a value,
    //similar to Some() in the precious example
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    //binding also works with if let
    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }

    /*
     *  another benefit is that 'if let' allows us to match non-parameterized enum variants.
     *  this is true even in caes where the enum doesn't implement or derive PartialEq. in such
     *  cases if Foo::Bar == a could fail to compile, because instances of the enum cannot be
     *  equated, however if let will continue to work.
     */

    /*
     *  CHALLANGE
     *  fix the following example to use if let
     */

    //enum Faz {Bar}
    let x = Faz::Bar;

    // variable matches Foo::Bar
    if let Faz::Bar = x {
        // ^ this causes a compile time error, use if let instead
        println!("a is fazbar");
    }
}
