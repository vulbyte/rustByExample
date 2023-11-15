//when data is bound by the same name immutably, it also freezes. frozen data can't be modified
//until the immutable binding goes out of scope:

fn main() {
    let mut mut_int = 7i32;

    {
        //shadowing by immutable
        let mut_int = mut_int;

        //below will error
        //mut_int = 50;
    }

    //this will not error

    println("{}", mut_int);
}
