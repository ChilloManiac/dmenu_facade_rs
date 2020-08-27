use std::{collections::HashMap, error::Error, fmt::Display, process::Command};

/// **DISCLAIMER! This crate uses the shell to pipe strings into dmenu.**
///
/// The dmenu wrapper.
/// This struct is built using a builder pattern and finally executed.
/// The items must implement Display to be displayed by dmenu.
///
/// The item's Display output must be unique, otherwise the latest in the list will be returned if the key is selected.
/// # Example
/// ```
/// use dmenu_facade::*;
/// let items = vec!["Hello", "World", "!"];
/// let chosen = DMenu::default()
///                    .vertical_with_lines(2)
///                    .execute(&items);
/// //Prints selected item to stdout
/// if let Ok(item) = chosen {
///     println!("{}", chosen);
/// }
/// ```
#[derive(Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
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
    /// Display dmenu on the bottom of the screen instead of the top
    /// # Example
    /// ```
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .display_bottom();
    /// ```
    pub fn display_bottom(mut self) -> Self {
        self.on_top = false;
        self
    }

    /// Dmenu will search the list case insensetively
    /// # Example
    /// ```
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .case_insensitive();
    /// ```
    pub fn case_insensitive(mut self) -> Self {
        self.case_insensitive = true;
        self
    }

    /// Display items vertically, with an amount of lines
    /// # Example
    /// ```
    /// //Display dmenu with 5 linex vertically
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .vertical_with_lines(5);
    /// ```
    pub fn vertical_with_lines(mut self, amount: i32) -> Self {
        self.vertical_lines = Some(amount);
        self
    }

    /// Display on a specific monitor. Index starts with 0
    /// # Example
    /// ```
    /// // To display on the second monitor
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .display_on_monitor(1);
    /// ```
    pub fn display_on_monitor(mut self, monitor_id: i32) -> Self {
        self.monitor = Some(monitor_id);
        self
    }

    /// Displays a prompt/title to the left of the selections.
    /// # Example
    /// ```
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .with_prompt("Select an item:");
    /// ```
    pub fn with_prompt(mut self, prompt: &'a str) -> Self {
        self.prompt = Some(prompt);
        self
    }

    /// Specifies which font should be used
    /// # Example
    /// ```
    /// use dmenu_facade::*;
    /// let dmenu = DMenu::default()
    ///                 .with_font("FiraCodeNerdFont:size=13");
    /// ```
    pub fn with_font(mut self, font: &'a str) -> Self {
        self.font = Some(font);
        self
    }

    /// Sets the colors for dmenu.
    ///
    /// If only specific colors are wanted, the rest can be set to None to use defaults
    /// # Examples
    /// ```
    /// use dmenu_facade::*;
    /// //To set normal background to white, normal foreground to black, selected background to red and selected foreground to blue.
    /// let dmenu = DMenu::default()
    ///                 .with_colors(Some(Color("#ffffff")), Some(Color("#000000")),
    ///                 Some(Color("#ff0000")), Some(Color("#0000ff")));
    /// ```
    /// ```
    /// use dmenu_facade::*;
    /// //To set normal background to white, normal foreground to black, selected background and selected foreground to default.
    /// let dmenu = DMenu::default()
    ///                 .with_colors(Some(Color("#ffffff")), Some(Color("#000000")),
    ///                 None, None);
    /// ```
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

    /// Formats the dmenu shell string
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

    /// Execute the dmenu struct as a command. Blocks the program till the user completes
    ///
    /// Takes a reference to a list of items with the Display trait, and returns a Result containing a reference to the chosen item.
    /// # Example
    /// ```
    /// use dmenu_facade::*;
    /// let items = vec!["Hello", "There", "Hope you", "Like my", "Docs :)"];
    /// let chosen = DMenu::default()
    ///                 .vertical_with_lines(4)
    ///                 .case_insensitive()
    ///                 .with_font("FiraCodeNerdFont:size=13")
    ///                 .with_prompt("Select an item!")
    ///                 .execute(&items);
    /// if let Ok(item) = chosen {
    ///     println!("{}", item);
    /// }
    /// ```
    pub fn execute<T: Display>(self, list: &Vec<T>) -> Result<&T, Box<dyn Error>> {
        let mut map: HashMap<String, &T> = HashMap::new();
        let mut list_string = String::from("");
        for item in list {
            let key: String = format!("{}\n", item);
            list_string.push_str(&key);
            map.insert(key, item);
        }

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

    /// Like execute, but consumes the list to return an owned item after the user chooses.
    pub fn execute_consume<T: Display>(self, list: Vec<T>) -> Result<T, Box<dyn Error>> {
        let mut map: HashMap<String, T> = HashMap::new();
        let mut list_string = String::from("");
        for item in list {
            let key: String = format!("{}\n", item);
            list_string.push_str(&key);
            map.insert(key, item);
        }

        let shell_output = Command::new("sh")
            .args(&[
                "-c",
                &format!("echo -e '{}' | {}", list_string, self.to_command()),
            ])
            .output()?;

        let chosen = String::from_utf8(shell_output.stdout)?;

        match map.remove(&chosen) {
            Some(found) => Ok(found),
            None => Err(Box::new(ItemNotFoundError)),
        }
    }

    /// Will launch the configured DMenu without any items and return the string typed by the user
    pub fn execute_as_input(self) -> Result<String, Box<dyn Error>> {
        let shell_output = Command::new("sh")
            .arg("-c")
            .arg(format!("echo -e '\n' | {}", self.to_command()))
            .output()?;

        let mut string = String::from_utf8(shell_output.stdout)?;
        string.pop(); // remove newline
        Ok(string)
    }
}

/// A struct for containing a color string.
#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub struct Color<'a>(pub &'a str);

#[derive(Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
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
