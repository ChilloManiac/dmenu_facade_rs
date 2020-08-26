use dmenu_facade::*;
use std::error::Error;
use std::fmt::Display;

fn main() -> Result<(), Box<dyn Error>> {
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

    if let Ok(item) = DMenu::default()
        .vertical_with_lines(2)
        .execute_consume(items)
    {
        println!("{}", item.id);
    }
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
