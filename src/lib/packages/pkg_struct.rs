use serde::Deserialize;
use std::str::FromStr;
use toml::value::Datetime;

#[derive(Debug, Deserialize, Clone)]
pub struct PKG {
    pub name: String,
    pub version: f64,
    pub subversion: i64,
    pub description: String,
    pub url: String,
    pub packager: String,
    pub date: Datetime,
    pub license: Licenses,
    pub dependence: Option<Vec<String>>,
    pub architecture: Architecture,
    pub optional_dependence: Option<Vec<String>>,
}

impl PKG {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_version(&self) -> f64 {
        self.version
    }
    pub fn get_subversion(&self) -> i64 {
        self.subversion
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_url(&self) -> String {
        self.url.clone()
    }
    pub fn get_packager(&self) -> String {
        self.packager.clone()
    }
    pub fn get_date(&self) -> Datetime {
        self.date.clone()
    }
    pub fn get_license(&self) -> Licenses {
        self.license.clone()
    }
    pub fn format_license(&self) -> String {
        self.license.format()
    }
    pub fn get_dependence(&self) -> Vec<String> {
        self.dependence
            .as_ref()
            .unwrap_or(&vec![String::new()])
            .to_vec()
    }
    pub fn get_architecture(&self) -> Architecture {
        self.architecture.clone()
    }
    pub fn get_optional_dependence(&self) -> Vec<String> {
        self.optional_dependence
            .as_ref()
            .unwrap_or(&vec![String::new()])
            .to_vec()
    }
    pub fn new() -> PKG {
        PKG {
            name: "".to_string(),
            version: 0.0,
            subversion: 0,
            description: "".to_string(),
            url: "".to_string(),
            packager: "".to_string(),
            date: Datetime::from_str("1979-05-27T07:32:00-08:00").expect("Error"),
            license: Licenses::MIT,
            dependence: Some(Vec::new()),
            architecture: Architecture::X8664,
            optional_dependence: Some(Vec::new()),
        }
    }
    pub fn deps_format(&self) -> String {
        let mut string_result = String::new();

        string_result.push('[');

        if let Some(opt_dep) = self.dependence.clone()  {
            for od in opt_dep {
                string_result.push_str(format!("{}," ,od).as_ref())
            }
        }
        string_result.push(']');
        string_result
    }
    pub fn opt_dep_format(&self) -> String {
        let mut string_result = String::new();
        string_result.push('[');
        if let Some(opt_dep) = self.optional_dependence.clone()  {
            for od in opt_dep {
                string_result.push_str(format!("{}," ,od).as_ref())
            }
        }
        string_result.push(']');
        string_result
    }
}
#[derive(Debug, Deserialize, Clone)]
pub enum Licenses {
    MIT,
    GPL,
    GPLv2,
    APACHE,
    WTFPL,
    PROPRIETARY,
    NO_LICENSE
}

impl Licenses {
    pub fn format(&self) -> String {
        match &self {
            Licenses::MIT => String::from("MIT"),
            Licenses::GPL => String::from("GPL"),
            Licenses::GPLv2 => String::from("GPLv2"),
            Licenses::APACHE => String::from("APACHE"),
            Licenses::PROPRIETARY => String::from("PROPRIETARY"),
            Licenses::WTFPL => String::from("WTFPL"),
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
pub enum Architecture {
    X8664,
    X64,
    RISCV,
}


impl Architecture {
    pub fn format(&self) -> String {
        match &self {
            Architecture::X8664 => String::from("x86_64"),
            Architecture::X64 => String::from("x64"),
            Architecture::RISCV => String::from("RISCV"),
        }
    }
}
