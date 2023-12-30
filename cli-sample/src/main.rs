// -*- mode: Rust; coding: utf-8 -*-

use dialoguer::{theme::ColorfulTheme, Confirm, FuzzySelect, History, Input};
use std::collections::VecDeque;

fn main() {
    // confirmation();
    // simple_input_mthod();
    // history_input_method();
    fuzzy_selection_input_method();
}

fn confirmation() {
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
    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("name")
        .interact_text()
        .unwrap();
    println!("{}", name)
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
    let mut history = MyHistory::new();
    loop {
        let text = Input::<String>::new()
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

fn fuzzy_selection_input_method() {
    let selections = vec![
        "set", "get", "show", "quit", "exit", "print", "text", "value",
    ];
    let command = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Command")
        .items(&selections)
        .interact()
        .unwrap();
    println!("{}", selections[command]);
}
