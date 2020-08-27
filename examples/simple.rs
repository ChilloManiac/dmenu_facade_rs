use dmenu_facade::*;

fn main() {
    let items = vec!["Hello", "World", ":D"];

    println!("{}", DMenu::default().execute(&items).unwrap());
}
