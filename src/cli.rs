use std::{io, str::SplitWhitespace};

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

        let mut input_strings = input.split_whitespace();

        if let Some(command) = input_strings.next() {
            match command {
                "exit" => {
                    lists.save_lists().context("Failed to save lists")?;
                    break;
                }
                "rm" => println!("{}", rm(&mut lists, input_strings)),
                "mk" => println!("{}", mk(&mut lists, input_strings)),
                "name" => println!("{}", name(&mut lists, input_strings)),
                "open" => match open(&mut lists, input_strings) {
                    Ok(list) => sub_handler(list),
                    Err(err) => println!("{}", err),
                },
                "help" => println!("{}", SUPER_HELP),
                _ => println!("Invalid command"),
            }
        } else {
            println!("Please provide command")
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

        let mut input_strings = input.split_whitespace();

        if let Some(command) = input_strings.next() {
            match command {
                "exit" => break,
                "rm" => println!("{}", rm(list, input_strings)),
                "mk" => println!("{}", mk(list, input_strings)),
                "name" => println!("{}", name(list, input_strings)),
                "open" => println!("{}", open_desc(list, input_strings)),
                "desc" => println!("{}", desc(list, input_strings)),
                "help" => println!("{}", SUB_HELP),
                _ => println!("Invalid command"),
            }
        } else {
            println!("Please provide command")
        }
    }
}

fn mk(menu: &mut dyn Menu, mut args: SplitWhitespace<'_>) -> String {
    let name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NAME arg".to_string(),
    };

    if let Err(err) = menu.mk(name.to_string()) {
        err.to_string()
    } else {
        format!("Added {name}")
    }
}

fn rm(menu: &mut dyn Menu, mut args: SplitWhitespace<'_>) -> String {
    let mut input = String::new();
    let name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NAME arg".to_string(),
    };

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

fn name(menu: &mut dyn Menu, mut args: SplitWhitespace<'_>) -> String {
    let name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NAME arg".to_string(),
    };

    let new_name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NEW_NAME arg".to_string(),
    };

    if let Err(err) = menu.name(name, new_name.to_string()) {
        err.to_string()
    } else {
        format!("Renamed {name} to {new_name}")
    }
}

fn open<'a>(lists: &'a mut Lists, mut args: SplitWhitespace<'_>) -> Result<&'a mut List> {
    let name = args.next().ok_or(anyhow!("Missing NAME arg"))?;

    lists.get_list(name)
}

fn open_desc(list: &mut List, mut args: SplitWhitespace<'_>) -> String {
    let name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NAME arg".to_string(),
    };

    match list.get_desc(name) {
        Ok(desc) => format!("\n{}\n{desc}", name.to_uppercase()),
        Err(err) => err.to_string(),
    }
}

fn desc(list: &mut List, mut args: SplitWhitespace<'_>) -> String {
    let name = match args.next() {
        Some(arg) => arg,
        None => return "Missing NAME arg".to_string(),
    };

    let new_desc = match args.next() {
        Some(arg) => arg,
        None => return "Missing NEW_DESC arg".to_string(),
    };

    if let Err(err) = list.desc(name, new_desc) {
        err.to_string()
    } else {
        format!("Changed {name}'s desc")
    }
}
