use std::io;

use felys::{Context, Object, Output};

pub fn print(cx: &mut Context) -> Output {
    let out = cx.args.iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
    Object::None.into()
}


pub fn input(cx: &mut Context) -> Output {
    if cx.args.len() > 1 {
        return Output::error("expect no more than one arg".into());
    }
    if let Some(msg) = cx.args.first() {
        println!("{}", msg)
    }
    let mut buf = String::new();
    if io::stdin().read_line(&mut buf).is_err() {
        return Output::error("failed to read input".into());
    };
    buf.pop();
    Object::String { value: buf }.into()
}
