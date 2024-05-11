#![allow(unused)]
#[derive(Debug)] // so we can inspect the state in a minute


/* enums */

enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }

        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }

    }
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


fn main(){

    let my_coin : Coin = Coin::Quarter(UsState::Alabama);
    let val: u8 = value_in_cents(my_coin);
    // println!("first coin: {}", val);
    dbg!(val);

    let five: Option<i32> = Some(5);
    let six: Option<i32> = plus_one(five);
    let none: Option<i32> = plus_one(None);

    dbg!(five);
    dbg!(six);
    dbg!(none);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}


    let mut count = 0;
    if let Coin::Quarter(state) = my_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

}

