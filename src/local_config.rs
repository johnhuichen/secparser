use colored::Colorize;
use inquire::{validator::Validation, Select, Text};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct LocalConfig {
    #[serde(default = "default_user_agent")]
    pub user_agent: String,

    #[serde(default = "default_temp_dir")]
    pub temp_dir: String,

    #[serde(default = "default_out_dir")]
    pub out_dir: String,
}

fn default_user_agent() -> String {
    String::from("example@domain.com")
}

fn default_temp_dir() -> String {
    String::from("tmp")
}

fn default_out_dir() -> String {
    String::from("out")
}

impl LocalConfig {
    const FILE_PATH: &'static str = "config/local.toml";

    fn create_file_if_missing() {
        if !Path::new(Self::FILE_PATH).exists() {
            File::create_new(Self::FILE_PATH)
                .unwrap_or_else(|_| panic!("Should create new file {}", Self::FILE_PATH));
            let cfg: Self = toml::from_str("")
                .unwrap_or_else(|_| panic!("Should parse config {}", Self::FILE_PATH));
            Self::save_to_file(&cfg);
            log::warn!(
                "{}",
                "Local config is created. Updating the user agent field to a non-default value is strongly recommended."
                    .bright_yellow()
            );
        }
    }

    fn parse_cfg_from_toml() -> Self {
        let contents = fs::read_to_string(Self::FILE_PATH)
            .unwrap_or_else(|_| panic!("Should open config {}", Self::FILE_PATH));

        toml::from_str(&contents)
            .unwrap_or_else(|_| panic!("Should parse config {}", Self::FILE_PATH))
    }

    pub fn new() -> Self {
        Self::create_file_if_missing();

        Self::parse_cfg_from_toml()
    }

    pub fn config_menu(&mut self) {
        loop {
            let update_user_agent =
                format!("- Update user agent ({})", self.user_agent.bright_green());
            let update_temp_dir =
                format!("- Update temp directory ({})", self.temp_dir.bright_green());
            let update_out_dir =
                format!("- Update out directory ({})", self.out_dir.bright_green());
            let done = String::from("- Done");
            let ans = Select::new(
                "Which config to update?",
                vec![
                    update_user_agent.to_owned(),
                    update_temp_dir.to_owned(),
                    update_out_dir.to_owned(),
                    done.to_owned(),
                ],
            )
            .prompt()
            .unwrap_or_else(|_| panic!("Should get a valid option"));

            if ans == update_user_agent {
                self.update_user_agent();
                continue;
            }
            if ans == update_temp_dir {
                self.update_temp_dir();
                continue;
            }
            if ans == update_out_dir {
                self.update_out_dir();
                continue;
            }
            if ans == done {
                break;
            }
        }
    }

    fn update_user_agent(&mut self) {
        let validator = |input: &str| {
            if input == default_user_agent() {
                return Ok(Validation::Invalid(
                    "Default user agent is not allowed".into(),
                ));
            }

            Ok(Validation::Valid)
        };

        let user_agent = Text::new("What is the new user agent?")
            .with_validator(validator)
            .with_help_message("see https://www.sec.gov/search-filings/edgar-search-assistance/accessing-edgar-data for details")
            .prompt()
            .unwrap_or_else(|_| panic!("Should get valid user agent"));

        if !user_agent.is_empty() {
            self.user_agent = user_agent;
        }

        Self::save_to_file(self);
    }

    fn update_temp_dir(&mut self) {
        let temp_dir = Text::new("What is the new temp directory?")
            .with_help_message("Temp directory is where downloaded files are stored")
            .prompt()
            .unwrap_or_else(|_| panic!("Should get valid temp directory"));

        if !temp_dir.is_empty() {
            self.temp_dir = temp_dir;
        }

        self.save_to_file()
    }

    fn update_out_dir(&mut self) {
        let out_dir = Text::new("What is the new data output directory?")
            .with_help_message("Out directory is where parsed files are stored")
            .prompt()
            .unwrap_or_else(|_| panic!("Should get valid out directory"));

        if !out_dir.is_empty() {
            self.out_dir = out_dir;
        }

        self.save_to_file();
    }

    fn save_to_file(&self) {
        let cfg_content = toml::to_string(self).unwrap();

        let mut file = File::create(Self::FILE_PATH).unwrap_or_else(|_| panic!());
        file.write_all(cfg_content.as_bytes())
            .unwrap_or_else(|_| panic!("Should write default config to empty config file"));
    }
}
