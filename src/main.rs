// Functions
fn hello() {
    println!("Hello everyone!");
}

fn hello_world() {
    println!("Hello world!");
}

// Functions Parameters
fn user(age: u8, name: String) {
    println!("I'm {} years old. My name is {}", age, name);
}

// Return Values
fn sum_two_values(number1: i32, number2: i32) -> i32 {
    return number1 + number2;
}

// Return a Value without Return Keyword
fn sum_two_values_without_return_keyword(number1: i32, number2: i32) -> i32 {
    number1 + number2
}

fn main() {
    // Functions
    hello();
    hello_world();

    // Functions Parameters
    user(27, "Ali".to_string()); // I'm 27 years old. My name is Ali

    // Function Bodies
    
    // let age = (let my_age = 12);

    let born_year = 1993;
    println!("{}", born_year);
    let year = {
        let born_year = 1990;
        born_year + 1
    };
    println!("Born Year {}", year);

    let born_year = 1993; // IDENTITY NUMBER: 11131211113
    println!("Born Year {}", born_year);

    // let year = {
    //     let born_year = 1990; // IDENTITY NUMBER: 0192585111
    //     println!("Born Year {}", born_year);
    //     // born_year + 1
    // };

    // Return Values
    println!("{}", sum_two_values(1, 2));
    
    // Return a Value without Return Keyword
    println!("{}", sum_two_values_without_return_keyword(1, 2));
}