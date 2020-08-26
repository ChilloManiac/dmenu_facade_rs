use dmenu_facade::*;
use std::error::Error;
use std::fmt::Display;

pub fn main() -> Result<(), Box<dyn Error>> {
    let items = vec![
        TestStruct {
            id: 0,
            text: "Hello".to_string(),
        },
        TestStruct {
            id: 1,
            text: "World".to_string(),
        },
        TestStruct {
            id: 2,
            text: "!".to_string(),
        },
    ];

    println!(
        "id: {}",
        DMenu::default().execute_consume(items).unwrap().id
    );

    Ok(())
}

struct TestStruct {
    id: i32,
    text: String,
}

impl Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text.fmt(f)
    }
}
