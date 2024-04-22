
fn operations() {
    let first: i32 = 10001;
    let second: i32 = 241;
    println!("sum {} and {}: {}\n", first, second, first + second);
    println!("substrusction {} and {}: {}\n", first, second, first - second);
    println!("multiplication {} and {}: {}\n", first, second, first * second);
    println!("division {} and {}: {}\n", first, second, first / second);
    println!("modulo {} and {}: {}\n", first, second, first % second);
}

/*
Let’s recap what we’ve learned so far about Cargo:

We can create a project using ```cargo new```.
We can build a project using ```cargo build```.
We can build and run a project in one step using ```cargo run```.
We can build a project without producing a binary to check for errors using ```cargo check```.
Instead of saving the result of the build in the same directory as our code,
Cargo stores it in the ```target/debug``` directory.
*/

fn main() {
    /* adding a comment */
    println!("Hello, world!\n");

    /* use functions */
    operations();
}



// use std::{any::type_name, mem::transmute_copy};

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }


// fn main(){
//     let mut x: i32 = 5;
//     println!("x: {}", x);
//     x = 6;
//     println!("x: {}", x);
//     let guess: u32 = "42"
//                         .parse()
//                         .expect("Not a number!");
//     println!("guess {}", guess);
//     let a: u8 = 17;
//     let b: u8 = 3;
//     let result = a.checked_rem(b); // 250 + 10 = 260, which is an overflow for u8
//     match result {
//         Some(value) => println!("checked_add result: {}", value),
//         None => println!("checked_add result: Overflow occurred"),
//     }

//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // Results in -1
//     println!("first: {} second :{}", quotient, truncated);
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
//     println!("tup : {}, {}, {}", tup.0, tup.1, tup.2);

//     let months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

//     println!("months: {:?}", months);
//     //let a = [0; 5];

// }
