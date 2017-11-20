extern crate gettext;
extern crate yaml_rust;

use std::fs;
use std::fs::File;
use self::gettext::Catalog;
use self::yaml_rust::YamlLoader;

use ui::ui_type::UIType;
use storage::storage_type::StorageType;

/**
 * Default application language setting. The list is representing
 * languages existing as a message catalogs.
 */
enum Language {
    En,
    Ru,
}

/**
 * Program-wide objects.
 */
pub struct Settings {
    language: Language,
    msgcat: Option<Catalog>,
    ui: UIType,
    storage_type: StorageType,
}

/**
 * Global application settings.
 */
impl Settings {
    /**
     * Settings loader-initializer. Tries to load settings from
     * anywhere (from a YAML file as for now) or initialize the
     * application with some sensible defaults.
     */
    pub fn new(config: String) -> Self {
        let mut setting : Settings = Settings { msgcat: None,
            ui: UIType::TUI,
            language: Language::En,
            storage_type: StorageType::SQLite };

        let cfg : String = fs::read_to_string(config)
            .expect("Unable to read file");
        let yaml_doc = YamlLoader::load_from_str(&cfg).unwrap();
        let doc = &yaml_doc[0];

        if !doc["borsh"].is_badvalue() {
            if !doc["borsh"]["lang"].is_badvalue() {
                match doc["borsh"]["lang"].as_str().unwrap() {
                    "ru" => {
                        setting.msgcat = Some(Settings::load_msgcat("ru.mo".to_string()));
                        setting.language = Language::Ru;
                        println!("Default language configured to Russian.");
                        println!("Loading ru.mo message catalog");
                    },
                    _ => {
                        println!("Erroneously configured language setting.");
                    },
                }
            } else {
                println!("No language setting found! Using default value of English UI.");
            }
            if !doc["borsh"]["ui"].is_badvalue() {
                match doc["borsh"]["ui"].as_str().unwrap() {
                    "tui" => {
                        setting.ui = UIType::TUI;
                        println!("Boring Shell UI configured to TUI.");
                    },
                    "cli" => {
                        setting.ui = UIType::CLI;
                        println!("Boring Shell UI configured to CLI.");
                    },
                    _ => {
                        println!("Erroneously configured UI setting.");
                    }
                }
            } else {
                println!("No UI setting found! Using default value of TUI.");
            }
            if !doc["borsh"]["storage"].is_badvalue() {
                match doc["borsh"]["storage"].as_str().unwrap() {
                    "sqlite" => {
                        setting.storage_type = StorageType::SQLite;
                        println!("Storage configured to SQLite database.");
                    }
                    _ => {
                        setting.storage_type = StorageType::SQLite;
                        println!("Storage defaulted to SQLite database.");
                    }
                }
            } else {
                println!("No default storage setting found! Using SQLite3 database as a storage.");
            }
        }

        return setting;
    }

    /**
     * Try to load the message catalog from the specified path or die.
     */
    fn load_msgcat(catalog_path: String) -> Catalog {
        let mcfile : File = File::open("ru.mo")
            .expect("could not open the catalog");
        let catalog : Catalog = Catalog::parse(mcfile)
            .expect("could not parse the catalog");
        return catalog;
    }

    /**
     * Translate the given string according to the loaded message
     * catalog.
     */
    pub fn tr(&self, text: String) -> String {
        match &self.msgcat {
            None => text,
            Some(cat) => String::from(cat.gettext(&text)),
        }
    }

    pub fn get_ui(&self) -> &UIType {
        return &self.ui;
    }

    pub fn get_storage_type(&self) -> &StorageType {
        return &self.storage_type;
    }
}

