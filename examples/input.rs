use dmenu_facade::*;

fn main() {
    println!("{}", DMenu::default().execute_as_input().unwrap());
}
