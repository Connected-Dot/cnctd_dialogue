use colored::{Color, Colorize};
use dialoguer::{Input, Select, MultiSelect};
use strum::IntoEnumIterator;

pub struct Dialog;

impl Dialog {
    pub fn input<T>(
        prompt: &str,
        default: Option<T>,
        color: Option<Color>,
        is_bold: Option<bool>,
    ) -> T
    where
        T: std::str::FromStr + Clone + ToString,
        <T as std::str::FromStr>::Err: ToString,
    {
        let styled_prompt = if let Some(bold) = is_bold {
            if bold {
                prompt.color(color.unwrap_or(Color::Cyan)).bold().to_string()
            } else {
                prompt.color(color.unwrap_or(Color::Cyan)).to_string()
            }
        } else {
            prompt.color(Color::Cyan).bold().to_string()
        };
    
        let input = if let Some(d) = default {
            Input::new().with_prompt(styled_prompt).default(d)
        } else {
            Input::new().with_prompt(styled_prompt)
        };
    
        input.interact().unwrap()
    }
    
    
    pub fn select<T: Clone + std::fmt::Display + IntoEnumIterator + PartialEq>(
        prompt: &str,
        default: Option<T>,
        color: Option<Color>,
        is_bold: Option<bool>,
    ) -> T {
        let options: Vec<T> = T::iter().collect();
        let default_index = default
            .as_ref()
            .and_then(|default_item| options.iter().position(|item| item == default_item))
            .unwrap_or(0);

        let styled_prompt = if let Some(bold) = is_bold {
            if bold {
                prompt.color(color.unwrap_or(Color::Cyan)).bold().to_string()
            } else {
                prompt.color(color.unwrap_or(Color::Cyan)).to_string()
            }
        } else {
            prompt.color(Color::Cyan).bold().to_string()
        };

        let selected_index = Select::new()
            .with_prompt(styled_prompt)
            .default(default_index)
            .items(&options)
            .interact()
            .unwrap();

        options[selected_index].clone()
    }

    pub fn multi_select<T: Clone + std::fmt::Display + IntoEnumIterator + PartialEq>(
        prompt: &str,
        checked: Option<Vec<T>>,
        color: Option<Color>,
        is_bold: Option<bool>,
    ) -> Vec<T> {
        let options: Vec<T> = T::iter().collect();
        let items_checked: Vec<(T, bool)> = options.clone().into_iter()
            .map(|s| {
                let is_checked = checked.clone().map_or(false, |v| v.contains(&s));
                (s, is_checked)
            })
            .collect();

        let styled_prompt = if let Some(bold) = is_bold {
            if bold {
                prompt.color(color.unwrap_or(Color::Cyan)).bold().to_string()
            } else {
                prompt.color(color.unwrap_or(Color::Cyan)).to_string()
            }
        } else {
            prompt.color(Color::Cyan).bold().to_string()
        };

        let selected_indices = MultiSelect::new()
            .with_prompt(styled_prompt)
            .items_checked(&items_checked)
            .interact()
            .unwrap();

        selected_indices.into_iter().map(|i| options[i].clone()).collect()
    }

    pub fn select_str(
        prompt: &str,
        options: &Vec<&str>,
        default_index: Option<usize>,
        color: Option<Color>,
        is_bold: Option<bool>,
    ) -> String {
        let mut options = options.clone();  // Clone to make it mutable
        let default_index = default_index.unwrap_or(0);
    
        // Reorder the array so the default item is first
        let default_item = options.remove(default_index);
        options.insert(0, default_item);
    
        let styled_prompt = if let Some(bold) = is_bold {
            if bold {
                prompt.color(color.unwrap_or(Color::Cyan)).bold().to_string()
            } else {
                prompt.color(color.unwrap_or(Color::Cyan)).to_string()
            }
        } else {
            prompt.color(Color::Cyan).bold().to_string()
        };
    
        let selected_index = Select::new()
            .with_prompt(styled_prompt)
            .default(0)  // Default index is now 0
            .items(&options)
            .interact()
            .unwrap();
    
        options[selected_index].to_string()
    }
    

    pub fn multi_select_str(
        prompt: &str,
        options: &Vec<&str>,
        checked: Option<Vec<&str>>,
        color: Option<Color>,
        is_bold: Option<bool>,
    ) -> Vec<String> {
        let items_checked: Vec<(&str, bool)> = options.iter().cloned()
            .map(|s| {
                let is_checked = checked.clone().map_or(false, |v| v.contains(&s));
                (s, is_checked)
            })
            .collect();

        let styled_prompt = if let Some(bold) = is_bold {
            if bold {
                prompt.color(color.unwrap_or(Color::Cyan)).bold().to_string()
            } else {
                prompt.color(color.unwrap_or(Color::Cyan)).to_string()
            }
        } else {
            prompt.color(Color::Cyan).bold().to_string()
        };

        let selected_indices = MultiSelect::new()
            .with_prompt(styled_prompt)
            .items_checked(&items_checked)
            .interact()
            .unwrap();

        selected_indices.into_iter().map(|i| options[i].to_string()).collect()
    }
}
