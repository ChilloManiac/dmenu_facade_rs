use dmenu_facade::*;
use std::fmt::Display;

fn main() {
    let items = vec![
        Complex {
            id: 1,
            name: "Complex thingy".to_string(),
            active: false,
            list: Vec::new(),
        },
        Complex {
            id: 2,
            name: "Somewhat Complex".to_string(),
            active: true,
            list: Vec::new(),
        },
    ];

    println!(
        "{}",
        DMenu::default()
            .vertical_with_lines(5)
            .with_font("FiraCodeNerdFont:size=13")
            .execute(&items)
            .unwrap()
    );
}

#[allow(dead_code)]
struct Complex {
    id: i32,
    name: String,
    active: bool,
    list: Vec<i32>,
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}
