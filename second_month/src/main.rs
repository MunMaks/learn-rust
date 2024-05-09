
/* enums */

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}


impl Message {
    fn call(&self) {
    }
}

enum Option<T> {
    None,
    Some(T),
}


fn main(){

    let m: Message = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    /*
    enum IpAddrKind {
        V4,
        V6,
    }
    
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    */


    // Continue
    /*
    enum IpAddr{
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home: IpAddr = IpAddr::V4(127, 0, 0, 1);

    let loopback: IpAddr = IpAddr::V6(String::from("::1"));
    */


    /*
    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
     */

}

