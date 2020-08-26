use std::{collections::HashMap, error::Error, fmt::Display, process::Command};

pub fn main() -> Result<(), Box<dyn Error>> {
    let dmenu = DMenu::default()
        .vertical_with_lines(2)
        .display_bottom()
        .case_insensitive()
        .with_prompt("Select a greeting:")
        .with_colors(Some(Color("#ffffff")), Some(Color("#000000")), None, None)
        .with_font("FiraCodeNerdFont:size=13");
    let items = vec!["Yo", "dingdong", "Swingdong"];

    println!("{}", dmenu.execute(&items)?);

    Ok(())
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
pub struct DMenu<'a> {
    on_top: bool,
    case_insensitive: bool,
    vertical_lines: Option<i32>,
    monitor: Option<i32>,
    prompt: Option<&'a str>,
    font: Option<&'a str>,
    normal_background_color: Option<Color<'a>>,
    normal_foreground_color: Option<Color<'a>>,
    selected_background_color: Option<Color<'a>>,
    selected_foreground_color: Option<Color<'a>>,
}

impl Default for DMenu<'_> {
    fn default() -> Self {
        Self {
            on_top: true,
            case_insensitive: false,
            vertical_lines: None,
            monitor: None,
            prompt: None,
            font: None,
            normal_background_color: None,
            normal_foreground_color: None,
            selected_background_color: None,
            selected_foreground_color: None,
        }
    }
}

impl<'a> DMenu<'a> {
    pub fn display_bottom(mut self) -> Self {
        self.on_top = false;
        self
    }

    pub fn case_insensitive(mut self) -> Self {
        self.case_insensitive = true;
        self
    }

    pub fn vertical_with_lines(mut self, amount: i32) -> Self {
        self.vertical_lines = Some(amount);
        self
    }

    pub fn display_on_monitor(mut self, monitor_id: i32) -> Self {
        self.monitor = Some(monitor_id);
        self
    }

    pub fn with_prompt(mut self, prompt: &'a str) -> Self {
        self.prompt = Some(prompt);
        self
    }

    pub fn with_font(mut self, font: &'a str) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_colors(
        mut self,
        normal_background_color: Option<Color<'a>>,
        normal_foreground_color: Option<Color<'a>>,
        selected_background_color: Option<Color<'a>>,
        selected_foreground_color: Option<Color<'a>>,
    ) -> Self {
        self.normal_background_color = normal_background_color;
        self.normal_foreground_color = normal_foreground_color;
        self.selected_background_color = selected_background_color;
        self.selected_foreground_color = selected_foreground_color;
        self
    }

    fn to_command(&self) -> String {
        let mut command = "dmenu".to_string();
        if !self.on_top {
            command.push_str(" -b");
        }

        if self.case_insensitive {
            command.push_str(" -i");
        }

        if let Some(lines) = self.vertical_lines {
            command.push_str(&format!(" -l {}", lines))
        };

        if let Some(monitor_index) = self.monitor {
            command.push_str(&format!(" -m {}", monitor_index));
        }

        if let Some(prompt) = &self.prompt {
            command.push_str(&format!(" -p '{}'", prompt));
        }

        if let Some(font) = &self.font {
            command.push_str(&format!(" -fn '{}'", font));
        }

        if let Some(nb) = &self.normal_background_color {
            command.push_str(&format!(" -nb '{}'", nb.0));
        }

        if let Some(nf) = &self.normal_foreground_color {
            command.push_str(&format!(" -nf '{}'", nf.0));
        }

        if let Some(sb) = &self.selected_background_color {
            command.push_str(&format!(" -sb '{}'", sb.0));
        }

        if let Some(sf) = &self.selected_foreground_color {
            command.push_str(&format!(" -sf '{}'", sf.0));
        }

        command
    }

    pub fn execute<T: Display>(self, list: &Vec<T>) -> Result<&T, Box<dyn Error>> {
        let mut map: HashMap<String, &T> = HashMap::new();
        let mut list_string = String::from("");
        for item in list {
            let key: String = format!("{}\n", item);
            list_string.push_str(&key);
            map.insert(key, item);
        }

        println!("{}", self.to_command());

        let shell_output = Command::new("sh")
            .args(&[
                "-c",
                &format!("echo -e '{}' | {}", list_string, self.to_command()),
            ])
            .output()?;

        let chosen = String::from_utf8(shell_output.stdout)?;

        match map.get(&chosen) {
            Some(found) => Ok(found),
            None => Err(Box::new(ItemNotFoundError)),
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub struct Color<'a>(&'a str);

#[derive(Debug)]
pub struct ItemNotFoundError;

impl Display for ItemNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The returned item by DMenu is not found in the original set"
        )
    }
}

impl Error for ItemNotFoundError {}
