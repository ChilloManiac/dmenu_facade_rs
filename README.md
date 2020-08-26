# dmenu_facade

This is a small library that allows dmenu to be used from a rust-project. It currently only works on linux and depends on dmenu being in the users \$PATH.

**DISCLAIMER: dmenu_facade uses the shell to interact with dmenu, look through the source if this makes you uncomfortable**

I mainly created this library for myself, since i wanted to start writing rust binaries instead of some of my scripts. For some of these scripts i wanted to have the ease of using dmenu as user interactions.

# How to use

dmenu_facade is pretty easy to use. It's used as a builder pattern with an execute method to finalize the instruction. It includes most of the args from `man dmenu`. It takes a list of items that must implement the `Display` trait and returns the picked item by reference or as owned depending on which function is called.

If more that one item's `Display` output is equal, the last item in the `Vec` will be returned.

## Example

```rs
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

// Just a test struct to see that it can work with more complex structs, as long as they implement Display
struct TestStruct {
    id: i32,
    text: String,
}

impl Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.text.fmt(f)
    }
}

```
