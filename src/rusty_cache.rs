use std::{collections::HashMap, fs, io};

#[derive(Debug)]
pub enum CacheValues {
    Text(String),
    Number(i32),
    Boolean(bool),
}

pub struct Cache {
    pub storage: HashMap<String, CacheValues>,
    output_file_path: String
}

impl Cache {
    pub fn new(file: String) -> Self {
        Cache {
            storage: HashMap::new(),
            output_file_path: file
        }
    }

    pub fn insert(&mut self, key: String, value: CacheValues) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&CacheValues> {
        self.storage.get(key)
    }

    pub fn remove(&mut self, key:&str) -> Option<CacheValues> {
        self.storage.remove(key)

    }

    pub fn write_to_file(&self) -> io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.storage {
            let value_str = match value {
                CacheValues::Text(s) => s.clone(),
                CacheValues::Number(n) => n.to_string(),
                CacheValues::Boolean(b) => b.to_string(),
            };
            contents.push_str(&format!("{}: {}\n", key, value_str));
        }
        fs::write(&self.output_file_path, contents)?;
        Ok(())
    }

}
