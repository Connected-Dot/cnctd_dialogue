use colored::Colorize;
use dialoguer::{Input, Select, MultiSelect};
use strum::IntoEnumIterator;

pub struct Dialog;

impl Dialog {
    pub fn input<T>(prompt: &str, default: Option<T>) -> T
    where
        T: std::str::FromStr + Clone + ToString,
        <T as std::str::FromStr>::Err: ToString,
    {
        let mut input = Input::new();
        input.with_prompt(prompt.cyan().bold().to_string());
    
        if let Some(d) = default {
            input.default(d);
        }
    
        input.interact().unwrap()
    }    

    pub fn select<T: Clone + std::fmt::Display + IntoEnumIterator + PartialEq>(
        prompt: &str,
        default: Option<T>,
    ) -> T {
        let options: Vec<T> = T::iter().collect();
        let default_index = default
            .as_ref()
            .and_then(|default_item| options.iter().position(|item| item == default_item))
            .unwrap_or(0);  // default to the first item if not found
    
        let selected_index = Select::new()
            .with_prompt(prompt.cyan().bold().to_string())
            .default(default_index)
            .items(&options)
            .interact()
            .unwrap();
    
        options[selected_index].clone()
    }

    pub fn multi_select<T: Clone + std::fmt::Display + IntoEnumIterator  + PartialEq>(prompt: &str, checked: Option<Vec<T>>) -> Vec<T> {
        let options: Vec<T> = T::iter().collect();
        let items_checked: Vec<(T, bool)> = options.clone().into_iter()
            .map(|s| {
                let is_checked = checked.clone().map_or(false, |v| v.contains(&s));
                (s, is_checked)
            })
            .collect();
    
        let selected_indices = MultiSelect::new()
            .with_prompt(prompt.cyan().bold().to_string())
            .items_checked(&items_checked)
            .interact()
            .unwrap();
        
        selected_indices.into_iter().map(|i| options[i].clone()).collect()
    }
    
}

