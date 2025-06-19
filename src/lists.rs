use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{
    env,
    fmt::{self, Display, Formatter},
    fs::{self, OpenOptions},
    path::PathBuf,
};

const FILE_NAME: &str = "lists.json";

pub trait Menu: Display {
    fn mk(&mut self, name: &str) -> Result<()>;
    fn rm(&mut self, name: &str) -> Result<()>;
    fn name(&mut self, name: &str, new_name: &str) -> Result<()>;
}

#[derive(Serialize, Deserialize)]
pub struct Lists {
    lists: Vec<List>,
}

#[derive(Serialize, Deserialize)]
pub struct List {
    name: String,
    items: Vec<ListItem>,
}

#[derive(Serialize, Deserialize)]
struct ListItem {
    name: String,
    desc: Option<String>,
}

impl Display for Lists {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let output = self.lists.iter().fold("".to_owned(), |acc, elem| {
            format!("{acc}{} [{}]\n", elem.name, elem.count())
        });

        write!(
            f,
            "\tLISTS\n\t_____\n{}",
            if output.is_empty() {
                "No lists found. \nUse mk [NAME] to make a new list."
            } else {
                &output
            }
        )?;
        Ok(())
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let items_text = self
            .items
            .iter()
            .fold("".to_owned(), |acc, elem| format!("{acc}{elem}\n"));

        write!(
            f,
            "\t{}\n\t{}\n{}",
            self.name,
            "_".repeat(self.name.len()),
            items_text
        )?;
        Ok(())
    }
}

impl Display for ListItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let desc_text = match &self.desc {
            Some(_) => " (...)",
            None => "",
        };

        write!(f, "{}{}", self.name, desc_text)?;
        Ok(())
    }
}

impl Menu for Lists {
    fn mk(&mut self, name: &str) -> Result<()> {
        if self.lists.iter().any(|list| list.name == name) {
            Err(anyhow!("Duplicate name"))
        } else {
            self.lists.push(List::new(name.to_string()));
            Ok(())
        }
    }

    fn rm(&mut self, name: &str) -> Result<()> {
        let index = self
            .lists
            .iter()
            .position(|list| list.name == name)
            .ok_or(anyhow!("List not found"))?;

        self.lists.remove(index);

        Ok(())
    }

    fn name(&mut self, name: &str, new_name: &str) -> Result<()> {
        let index = self
            .lists
            .iter()
            .position(|list| list.name == name)
            .ok_or(anyhow!("List not found"))?;

        self.lists[index].name = new_name.to_string();

        Ok(())
    }
}

impl Lists {
    pub fn open_lists() -> Result<Self> {
        let saves_location = Self::get_saves_location()?;

        if !saves_location.try_exists()? {
            fs::write(&saves_location, "{\"lists\":[]}")
                .context("Failed whilst create saves file")?
        }

        let saves_file = OpenOptions::new()
            .read(true)
            .open(&saves_location)
            .with_context(|| {
                format!(
                    "Insufficient permissions to access saves file at {}",
                    saves_location.to_str().unwrap_or("non-utf8 path")
                )
            })?;

        Ok(serde_json::from_reader(saves_file)?)
    }

    pub fn save_lists(self) -> Result<()> {
        Ok(fs::write(
            Self::get_saves_location()?,
            serde_json::to_string(&self).context("failed to serialise lists")?,
        )?)
    }

    pub fn get_list(&mut self, name: &str) -> Result<&mut List> {
        let index = self
            .lists
            .iter()
            .position(|list| list.name == name)
            .ok_or(anyhow!("List not found"))?;

        Ok(&mut self.lists[index])
    }

    fn get_saves_location() -> Result<PathBuf> {
        Ok(env::current_exe()
            .context("Insufficient permissions to access working directory")?
            .parent().unwrap()
            .join(FILE_NAME))
    }
}

impl Menu for List {
    fn mk(&mut self, name: &str) -> Result<()> {
        if self.items.iter().any(|item| item.name == name) {
            return Err(anyhow!("Duplicate name"));
        } else {
            self.items.push(ListItem::new(name.to_string()))
        }

        Ok(())
    }

    fn rm(&mut self, name: &str) -> Result<()> {
        let index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .ok_or(anyhow!("List not found"))?;

        self.items.remove(index);

        Ok(())
    }

    fn name(&mut self, name: &str, new_name: &str) -> Result<()> {
        let index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .ok_or(anyhow!("List not found"))?;

        self.items[index].name = new_name.to_string();

        Ok(())
    }
}

impl List {
    fn new(name: String) -> Self {
        Self {
            name,
            items: Vec::new(),
        }
    }

    pub fn get_desc(&self, name: &str) -> Result<&str> {
        let index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .ok_or(anyhow!("Item not found"))?;

        Ok(match &self.items[index].desc {
            Some(desc) => &desc[..],
            None => "No description",
        })
    }

    pub fn desc(&mut self, name: &str, desc: &str) -> Result<()> {
        let index = self
            .items
            .iter()
            .position(|item| item.name == name)
            .ok_or(anyhow!("List not found"))?;

        self.items[index].desc = Some(desc.to_string());

        Ok(())
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

impl ListItem {
    fn new(name: String) -> Self {
        Self { name, desc: None }
    }
}
