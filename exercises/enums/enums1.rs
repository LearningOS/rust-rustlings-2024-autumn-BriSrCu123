// enums1.rs
//
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

fn main() {
    // 下面是使用这些变体的示例
    let msg1 = Message::Move { x: 10, y: 30 };
    let msg2 = Message::Echo(String::from("hello world"));
    let msg3 = Message::ChangeColor(200, 255, 255);
    let msg4 = Message::Quit;

    println!("{:?}", msg1);
    println!("{:?}", msg2);
    println!("{:?}", msg3);
    println!("{:?}", msg4);
}
