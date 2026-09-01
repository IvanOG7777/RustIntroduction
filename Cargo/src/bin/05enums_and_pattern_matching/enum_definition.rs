enum IpAddrKind {
    V4,
    V6,
}

enum IpAdderKindString {
    V4(String),
    V6(String),
}

enum IpAddrKindIntString {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

struct Ipv4Addr {

}

struct Ipv6Addr {

}
// You can put structs directly into enums
enum IpAddrStruct {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

enum Message {
    Quit, // No data
    Move {x: i32, y: i32}, // Has named fields x,y like a struct
    Write(String), // Includes a string
    ChangeColor(i32, i32, i32), // Includes a tuple
}

struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

impl Message {
    fn call(&self) {
        println!("call() has been called");
    }
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loop_back = IpAddr {
        kind:IpAddrKind::V6,
        address: String::from("::1"),
    };

    // Using enum type specification
    let home2 = IpAdderKindString::V4(String::from("127.0.0.1"));
    let loop_back_2 = IpAdderKindString::V6(String::from("::1"));

    let home3 = IpAddrKindIntString::V4(127, 0, 0, 1);
    let loop_back_3 = IpAddrKindIntString::V6(String::from("::1"));

    {
        let m = Message::Write(String::from("Hello"));
        m.call();
    }

}