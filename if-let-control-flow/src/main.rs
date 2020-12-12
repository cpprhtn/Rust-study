

fn main() {
    // ver1 - match
    let some_u8_value = Some(3u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // ver1 - if let
    if let Some(3) = some_u8_value {
        println!("three");
    }

    
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    let coin = Coin::Penny;
    let mut count = 0;
    // ver2 - match
    /*
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
    */

    // ver2 - if let
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } 
    else {
        count += 1;
    }
}