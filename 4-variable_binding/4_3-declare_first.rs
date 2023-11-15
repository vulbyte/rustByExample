fn main() {
    let text = r#"
        this entire thing was just saying you can initialize then decalre vars, tho it is discouraged.
        the compiler forbids the use of uninitalized variabled, as this would lead to undefined behavior.

        this is also how you do a literal printing in rust via: r\"\"\#\; (without the \'s)
    "#;

    println!("{}", text);
}
