use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    code: u32,
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Res2 {
    code: u32,
}

fn main() {
    let res = Response {
        code: 412,
        message: "I'm early!!!".into(),
    };
    dbg!(&res);

    let encoded = serde_json::to_string(&res).unwrap();
    println!("{encoded}");

    let data = serde_json::from_str::<Res2>(&encoded).unwrap();
    dbg!(data);
}
