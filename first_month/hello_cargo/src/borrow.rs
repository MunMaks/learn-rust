
fn main() {
    first_function(5, 'h');
    println!("Main println!");
    // let mut y = {
    //     let x = 3;
    //     x + 1
    // };
    let y = plus_one(6);
    println!("The value of y is: {y}");


    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

    let mut s: String = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); // This will print `hello, world!`
    {
        let s = String::from("inside of parantheses"); // s is valid from this point forward
        println!("{s}");
        // do stuff with s
    }                                  // this scope is now over, and s is no longer valid

    let new_str: String = take_ownership(s);
    println!("new string: {new_str}");


    let s1: String = String::from("Bonjour Tout le monde\n");

    let (s2, len) = calculate_length_v1(s1);

    println!("The length of '{}' is {}.", s2, len);


    let s_good: String = String::from("Une chaine");
    let longueur: usize = calculate_length_v2(&s_good);
    
    println!("s good longueur : {longueur}");

}


fn calculate_length_v1(s: String) -> (String, usize) {
    let length: usize = s.len(); // len() returns the length of a String

    (s, length)
}


fn calculate_length_v2(s: &String) -> usize{
    s.len()
}


fn first_function(i: i32, h: char){
    println!("First output from function, variable: {i}, char: {h}\n");
}


// dont add ; to return value
fn plus_one(i: i32) -> i32 {
    i + 1
}

fn take_ownership(s: String) -> String{
    println!("I got ownership of the object {}", s);
    s
}
