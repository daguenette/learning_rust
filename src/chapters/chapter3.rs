
fn variable() -> i32 {

    // By default variables are immutable 
    let x = 5;

    // You still have the option to make your variables mutable by
    // adding the mut keyword.
    let mut y = 3;

    // Adding the mut keyword also conveys intent to future readers
    // that other parts of the code will be changing this variable.
    y = x + y;

    return y;
}

fn constant() -> u32 {

    // Constant
    // - Like immutable variables, constant aren't allowed to change.
    // - They use the const keyword instead of let.
    // - They must be annotated (: u32).
    // - The convention is to write constant names using all caps.
    // - Can only be set with expression known at runtime.
    // - Useful when need to be declared as Global and reuse troughout the code.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    return THREE_HOURS_IN_SECONDS;
}

pub fn run() {
    
    // Variables
    println!("Variables");
    let y = variable();
    println!("{y}");

    // Constant
    println!("Constants");
    println!("{ }", constant());

}
