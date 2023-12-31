// -*- mode: Rust; coding: utf-8 -*-

use dialoguer::{theme::ColorfulTheme, Completion, Confirm, FuzzySelect, History, Input};
use std::collections::VecDeque;

fn main() {
    confirmation();
    simple_input_mthod();
    history_input_method();
    fuzzy_selection_input_method();
    completion_input_method();
}

fn confirmation() {
    println!("Comfirmation method.");
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue?")
        .default(true)
        .interact()
        .unwrap();
    match confirm {
        true => println!("Looks like you want to continue."),
        false => println!("Nevermind then :("),
    }
}

fn simple_input_mthod() {
    println!("The simplest input text method.");
    let text: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("name")
        .interact_text()
        .unwrap();
    println!("{}", text)
}

struct MyHistory(VecDeque<String>);

impl MyHistory {
    fn new() -> Self {
        Self(VecDeque::new())
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        self.0.get(pos).cloned()
    }
    fn write(&mut self, val: &T) {
        self.0.push_front(val.to_string());
    }
}

fn history_input_method() {
    println!("Input text with historical method.");
    let mut history = MyHistory::new();
    loop {
        let text = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Command")
            .history_with(&mut history)
            .interact_text()
            .unwrap();
        println!("{}", text);
        if text == "exit" {
            break;
        }
    }
}

struct MyCompletion(Vec<String>);
impl MyCompletion {
    fn new() -> Self {
        Self(vec![
            "set".to_string(),
            "get".to_string(),
            "show".to_string(),
            "quit".to_string(),
            "exit".to_string(),
            "print".to_string(),
            "text".to_string(),
            "value".to_string(),
        ])
    }
}
impl Completion for MyCompletion {
    fn get(&self, input: &str) -> Option<String> {
        let matches = self
            .0
            .iter()
            .filter(|option| option.starts_with(input))
            .collect::<Vec<_>>();
        match matches.len() {
            1 => Some(matches[0].to_string()),
            _ => None,
        }
    }
}

fn completion_input_method() {
    println!("Input text with completion method.");
    let completion = MyCompletion::new();
    let text = Input::<String>::with_theme(&ColorfulTheme::default())
        .with_prompt("Command")
        .completion_with(&completion)
        .interact_text()
        .unwrap();
    println!("{}", text);
}

fn fuzzy_selection_input_method() {
    println!("Input text with fuzzy selection method.");
    let selections = vec![
        "set", "get", "show", "quit", "exit", "print", "text", "value",
    ];
    let text = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Command")
        .items(&selections)
        .interact()
        .unwrap();
    println!("{}", selections[text]);
}
