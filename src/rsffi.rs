use std::collections::HashMap;
use std::io;

use felys::{Context, Language, Object, Output};

pub fn register(lang: &Language) -> HashMap<String, Object> {
    match lang {
        Language::ZH => HashMap::from([
            ("打印".into(), Object::Rust(print)),
            ("输入".into(), Object::Rust(input))
        ]),
        Language::EN => HashMap::from([
            ("print".into(), Object::Rust(print)),
            ("input".into(), Object::Rust(input)),
        ])
    }
}


fn print(cx: &mut Context) -> Output {
    let out = cx.args.iter()
        .map(|o| o.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", out);
    Object::None.into()
}


fn input(cx: &mut Context) -> Output {
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
    Object::String(buf.trim().into()).into()
}
