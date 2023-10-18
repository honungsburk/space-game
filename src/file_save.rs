use std::{error::Error, fs::File, io::Read};

use serde::{Deserialize, Serialize};

/// Trait for saving and loading from a file
pub trait FileSave {
    type Item: Serialize + for<'de> Deserialize<'de> + Default;
    fn load_from_file(path: &str) -> Result<Self::Item, Box<dyn Error>>;
    fn save_to_file(&self, path: &str) -> Result<(), Box<dyn Error>>;
}

pub fn load_from_file(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn save_to_file(path: &str, content: &str) -> Result<(), Box<dyn Error>> {
    std::fs::write(path, content)?;
    Ok(())
}
