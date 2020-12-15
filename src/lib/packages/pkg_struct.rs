use serde::Deserialize;
use std::io::ErrorKind;
use std::str::FromStr;
use toml::value::Datetime;

#[derive(Debug, Deserialize, Clone)]
pub struct PKG {
    pub id: Option<i64>,
    pub name: String,
    pub version: f64,
    pub subversion: i64,
    pub description: String,
    pub url: String,
    pub packager: String,
    pub date: Datetime,
    pub license: Licenses,
    pub source: Option<Source>,
    pub dependence: Option<Vec<String>>,
    pub architecture: Architecture,
    pub optional_dependence: Option<Vec<String>>,
    pub installed: Option<bool>,
    pub file: Option<Vec<String>>,
}

impl PKG {
    pub fn new() -> PKG {
        PKG {
            id: Some(0),
            name: "".to_string(),
            version: 0.0,
            subversion: 0,
            description: "".to_string(),
            url: "".to_string(),
            packager: "".to_string(),
            date: Datetime::from_str("1979-05-27T07:32:00-08:00").expect("Error"),
            license: Licenses::MIT,
            source: Some(Source::Repo),
            dependence: Some(Vec::new()),
            architecture: Architecture::X8664,
            optional_dependence: Some(Vec::new()),
            installed: Some(false),
            file: None,
        }
    }
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
    pub fn get_source(&self) -> Option<Source> {
        self.source.clone()
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
    pub fn get_installed(&self) -> Option<bool> {
        self.installed
    }
    pub fn get_files(&self) -> Option<Vec<String>> {
        self.file.clone()
    }
    pub fn get_id(&self) -> Option<i64> {
        self.id.clone()
    }
    pub fn deps_format(&self) -> String {
        let mut string_result = String::new();

        string_result.push('[');

        if let Some(opt_dep) = self.dependence.clone() {
            for od in opt_dep {
                string_result.push_str(format!("{},", od).as_ref())
            }
        }
        string_result.push(']');
        string_result
    }
    pub fn opt_dep_format(&self) -> String {
        let mut string_result = String::new();
        string_result.push('[');
        for od in self.get_optional_dependence().clone() {
            string_result.push_str(format!("{},", od).as_ref())
        }
        string_result.push(']');
        string_result
    }
    pub fn set_name(&mut self, name: String) -> &mut PKG {
        self.name = name;
        self
    }
    pub fn set_version(&mut self, version: f64) -> &mut PKG {
        self.version = version;
        self
    }
    pub fn set_subversion(&mut self, subversion: i64) -> &mut PKG {
        self.subversion = subversion;
        self
    }
    pub fn set_description(&mut self, description: String) -> &mut PKG {
        self.description = description;
        self
    }
    pub fn set_url(&mut self, url: String) -> &mut PKG {
        self.url = url;
        self
    }
    pub fn set_packager(&mut self, packager: String) -> &mut PKG {
        self.packager = packager;
        self
    }
    pub fn set_date(&mut self, date: Datetime) -> &mut PKG {
        self.date = date;
        self
    }
    pub fn set_license(&mut self, license: Licenses) -> &mut PKG {
        self.license = license;
        self
    }
    pub fn set_dependence(&mut self, dependence: Vec<String>) -> &mut PKG {
        self.dependence = Some(dependence);
        self
    }
    pub fn set_architecture(&mut self, architecture: Architecture) -> &mut PKG {
        self.architecture = architecture;
        self
    }
    pub fn set_optional_dependence(&mut self, optional_dependence: Vec<String>) -> &mut PKG {
        self.optional_dependence = Some(optional_dependence);
        self
    }
    pub fn set_installed(&mut self, installed: bool) -> &mut PKG {
        self.installed = Some(installed);
        self
    }
    pub fn set_installed_from_i64(&mut self, value: i64) -> Result<&mut PKG, std::io::Error> {
        if value == 0 {
            self.set_installed(false);
            Ok(self)
        } else if value == 1 {
            self.set_installed(true);
            Ok(self)
        } else {
            Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Can't parse value",
            ))
        }
    }
    pub fn set_files(&mut self, files: Vec<String>) -> &mut PKG {
        self.file = Some(files);
        self
    }
    pub fn set_id(&mut self, id: i64) -> &mut PKG {
        self.id = Some(id);
        self
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
    NOLICENSE,
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
            Licenses::NOLICENSE => String::from("NO_LICENSE"),
        }
    }
    pub fn from_str<'a>(str: &str) -> Result<Licenses, std::io::Error> {
        match str.to_uppercase().as_str() {
            "MIT" => Ok(Licenses::MIT),
            "GPL" => Ok(Licenses::GPL),
            "GPLv2" => Ok(Licenses::GPLv2),
            "APACHE" => Ok(Licenses::APACHE),
            "PROPRIETARY" => Ok(Licenses::PROPRIETARY),
            "WTFPL" => Ok(Licenses::WTFPL),
            "NO_LICENSE" => Ok(Licenses::NOLICENSE),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Can't parse from str",
            )),
        }
    }
}
#[derive(Debug, Deserialize, Clone)]
pub enum Architecture {
    X8664,
    X64,
    RISCV,
    ANY,
}

impl Architecture {
    pub fn format(&self) -> String {
        match &self {
            Architecture::X8664 => String::from("x86_64"),
            Architecture::X64 => String::from("x64"),
            Architecture::RISCV => String::from("RISCV"),
            Architecture::ANY => String::from("ANY"),
        }
    }
    pub fn from_str(str: &str) -> Result<Architecture, std::io::Error> {
        match str.to_uppercase().as_str() {
            "ANY" => Ok(Architecture::ANY),
            "RISCV" => Ok(Architecture::RISCV),
            "X64" => Ok(Architecture::X64),
            "X8664" => Ok(Architecture::X8664),
            _ => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                "Can't parse from str",
            )),
        }
    }
}
#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Source {
    Repo,
    Appimage,
}

impl Source {
    pub fn format(&self) -> String {
        match &self {
            Source::Repo => "Repo".to_string(),
            Source::Appimage => "AppImage".to_string(),
        }
    }
}
