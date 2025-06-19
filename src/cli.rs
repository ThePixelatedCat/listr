use std::io;

use crate::lists::{List, Lists, Menu};
use anyhow::{Context, Result, anyhow};

const SUPER_HELP: &str = "\nrm [NAME]: delete the specified list
                          \nmk [NAME]: make a new list with the specified name
                          \nname [NAME] [NEW_NAME]: set the name of the specified list
                          \nopen [NAME]: open the specified list
                          \nhelp: view this list of commands
                          \nexit: save your changes and quit the application";
const SUB_HELP: &str = "\nrm [NAME]: delete the specified item
                        \nmk [NAME]: make a new item with the specified name
                        \nname [NAME] [NEW_NAME]: set the name of the specified item
                        \nopen [NAME]: view the description of the specified item
                        \ndesc [NAME]: edit the description of the specified item
                        \nhelp: view this list of commands
                        \nexit: return to lists menu";

pub fn super_handler(mut lists: Lists) -> Result<()> {
    let mut input = String::new();

    loop {
        input.clear();
        println!("\n\n\n\n{}", lists);
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("Error reading input: {err}");
            continue;
        }

        let input_strings = input.split_once(' ').map(|input| (input.0, input.1.trim())).unwrap_or((input.trim(), ""));

        match input_strings.0 {
            "exit" => {
                lists.save_lists().context("Failed to save lists")?;
                break;
            }
            "rm" => println!("{}", rm(&mut lists, input_strings.1)),
            "mk" => println!("{}", mk(&mut lists, input_strings.1)),
            "name" => println!("{}", name(&mut lists, input_strings.1)),
            "open" => match open(&mut lists, input_strings.1) {
                Ok(list) => sub_handler(list),
                Err(err) => println!("{}", err),
            },
            "help" => println!("{}", SUPER_HELP),
            _ => println!("Invalid command"),
        }
    }

    Ok(())
}

fn sub_handler(list: &mut List) {
    let mut input = String::new();

    loop {
        input.clear();
        println!("\n{}", list);
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("Error reading input: {err}");
            continue;
        }

        let input_strings = input.split_once(' ').map(|input| (input.0, input.1.trim())).unwrap_or((input.trim(), ""));
        
        match input_strings.0 {
            "exit" => break,
            "rm" => println!("{}", rm(list, input_strings.1)),
            "mk" => println!("{}", mk(list, input_strings.1)),
            "name" => println!("{}", name(list, input_strings.1)),
            "open" => println!("{}", open_desc(list, input_strings.1)),
            "desc" => println!("{}", desc(list, input_strings.1)),
            "help" => println!("{}", SUB_HELP),
            _ => println!("Invalid command"),
        }
    }
}

fn mk(menu: &mut dyn Menu, name: &str) -> String {
    if name.is_empty() {
        return "Missing NAME arg".to_string();
    }

    if let Err(err) = menu.mk(name) {
        err.to_string()
    } else {
        format!("Added {name}")
    }
}

fn rm(menu: &mut dyn Menu, name: &str) -> String {
    let mut input = String::new();
    if name.is_empty() {
        return "Missing NAME arg".to_string();
    }

    println!("Are you sure you want to delete {name}? [Y/n]");
    if let Err(err) = io::stdin().read_line(&mut input) {
        return format!("Error reading confirmation: {err}");
    }

    if input.trim() == "Y" {
        if let Err(err) = menu.rm(name) {
            err.to_string()
        } else {
            format!("Deleted {name}")
        }
    } else {
        format!("Cancelled deletion of {name}")
    }
}

fn name(menu: &mut dyn Menu, name: &str) -> String {
    let mut input = String::new();
    if name.is_empty() {
        return "Missing NAME arg".to_string();
    }

    println!("Enter new name");
    if let Err(err) = io::stdin().read_line(&mut input) {
        return format!("Error reading new name: {err}");
    }
    let trimmed_input = input.trim();

    if let Err(err) = menu.name(name, trimmed_input) {
        err.to_string()
    } else {
        format!("Renamed {name} to {trimmed_input}")
    }
}

fn open<'a>(lists: &'a mut Lists, name: &str) -> Result<&'a mut List> {
    if name.is_empty() {
        return Err(anyhow!("Missing NAME arg"));
    }

    lists.get_list(name)
}

fn open_desc(list: &mut List, name: &str) -> String {
    if name.is_empty() {
        return "Missing NAME arg".to_string();
    }

    match list.get_desc(name) {
        Ok(desc) => format!("\n{}\n{desc}", name.to_uppercase()),
        Err(err) => err.to_string(),
    }
}

fn desc(list: &mut List, name: &str) -> String {
    let mut input = String::new();
    if name.is_empty() {
        return "Missing NAME arg".to_string();
    }

    println!("Enter description");
    if let Err(err) = io::stdin().read_line(&mut input) {
        return format!("Error reading description: {err}");
    }

    if let Err(err) = list.desc(name, input.trim()) {
        err.to_string()
    } else {
        format!("Changed {name}'s desc")
    }
}
