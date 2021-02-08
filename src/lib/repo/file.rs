use futures::StreamExt;
use async_std::path::PathBuf;
use std::fs::read_to_string;
use toml::Value;
use std::collections::BTreeMap;


pub async fn get_repo_from_repod_formatted() -> Vec<(String, String)> {
    let files = get_file_from_repod().await;
    let mut result = Vec::new();
    for file in files {
        let e = read_to_string(file).unwrap();
        let value = e.as_str().parse::<Value>().unwrap();
        for (name, url) in value.table_to_btreemap().unwrap() {
            result.push((name, url));
        }
    }

    result
}

pub async fn get_repo_from_repod() -> Vec<Value> {
    let files = get_file_from_repod().await;
    let mut result = Vec::new();
    for file in files {
        let e = read_to_string(file).unwrap();
        let value = e.as_str().parse::<Value>().unwrap();
        result.push(value);
    }

    result
}



pub async fn get_file_from_repod() -> Vec<PathBuf> {
    let mut result = Vec::new();

    if let Ok(mut readdir) = async_std::fs::read_dir("/etc/sulfur/repo.d/").await {
        while let Some(e) = readdir.next().await {
            result.push(e.unwrap().path());
        }
    }

    result
}

trait TableToBTree {
    fn table_to_btreemap(&self) -> Option<BTreeMap<String, String>>;
}

impl TableToBTree for Value {
    fn table_to_btreemap(&self) -> Option<BTreeMap<String, String>> {
        if self.is_table() {
            let mut btreemap: BTreeMap<String, String> = BTreeMap::new();
            match self {
                Value::String(_) => {}
                Value::Integer(_) => {}
                Value::Float(_) => {}
                Value::Boolean(_) => {}
                Value::Datetime(_) => {}
                Value::Array(_) => {}
                Value::Table(e) => {
                    for (e, i) in e.iter() {
                        btreemap.insert(e.to_string(), i.get("repo").unwrap().to_string());
                    }

                }
            };
            Some(btreemap)
        } else {
            None
        }

    }
}