//rust provides type safety via static typic. variable bindings can be annotated when declared.
//however, in most cases the compiler wil leb able to infer the type of the variable from teh
//context. heavily reducing the annotation burden.
//
//values (like literals) can eb bound to variables using the let binding

fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // copy an_integer into copied_integer
    let copied_integer = an_integer;

    println!("an integer: {:?}", copied_integer);
    println!("a boolean: {:?}", a_boolean);
    println!("meet the unit value: {:?}", unit);

    //the compiler warns about unused variable bindings. these can be scilenced by prefixing with
    //an underscore ['_']
    let _unused_variable = 2u32;

    //let noisy_unused_variable = 2u32;
    //
}

/*


              ┌──────────────┐
              │              │           ┌┐
          ┌─┐ │              ├───────────┴┤
          │ └─┘             ─┘            │
     ┌┬───┘                               └┐
     └┤                                    ├─┬───┐
      │  ┌──────────────┐ ┌─┐     ┌────────┴─┤   │
   ┌─┬┴┬─┘              └─┤ └──┐  │          │ ──┘
   └─┤ │                  │    └──┘          │
     │ │                                     │
  │  │ │    │                            │   │
  ├──┘ │    └┬────────┐      ──┬────────┬┘   │
  │    │     ├─────┼┼─┘        └────┼┼──┤    │
  │    │     │     └┘        │      └┘  │    │
  └┬───┤                     └───┐           ├──┐
   │   │                         │           │  │
   │   │                    ─────┘           │  │
   │   │                                     ├──┘
   └───┤       ┌───┬───────────────┐         │
       │       │   │               │         │    < i don't care who corperate sends, i'm not compiling this code
       │       │   │         ┌─────┘         │
       └────┐  └───┴─────────┘        ┌──────┘
            │                         │
            │                         │
            │                         │
            └─────────────────────────┘

*/
