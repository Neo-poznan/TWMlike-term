use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub window: WindowConfig,
    #[serde(default)]
    pub terminal: TerminalConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WindowConfig {
    #[serde(default = "default_width")]
    pub width: i32,
    #[serde(default = "default_height")]
    pub height: i32,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default = "default_headerbar_style")]
    pub headerbar_style: String,
    #[serde(default = "default_title")]
    pub title: String,
    #[serde(default = "default_border_width")]
    pub border_width: i32,
    #[serde(default = "default_border_color")]
    pub border_color: String,
    #[serde(default = "default_border_radius")]
    pub border_radius: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TerminalConfig {
    #[serde(default = "default_background_color")]
    pub background_color: String,
}

// Значения по умолчанию
fn default_width() -> i32 {
    800
}

fn default_height() -> i32 {
    600
}

fn default_opacity() -> f64 {
    1.0
}

fn default_headerbar_style() -> String {
    "standard".to_string()
}

fn default_title() -> String {
    "terminal".to_string()
}

fn default_border_width() -> i32 {
    0
}

fn default_border_color() -> String {
    "#ffffff".to_string()
}

fn default_border_radius() -> i32 {
    0
}

fn default_background_color() -> String {
    "#1e1e1e".to_string()
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            width: default_width(),
            height: default_height(),
            opacity: default_opacity(),
            headerbar_style: default_headerbar_style(),
            title: default_title(),
            border_width: default_border_width(),
            border_color: default_border_color(),
            border_radius: default_border_radius(),
        }
    }
}

impl Default for TerminalConfig {
    fn default() -> Self {
        Self {
            background_color: default_background_color(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            window: WindowConfig::default(),
            terminal: TerminalConfig::default(),
        }
    }
}

impl Config {
    /// Загружает конфигурацию из файла
    pub fn load() -> Self {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            match fs::read_to_string(&config_path) {
                Ok(contents) => match toml::from_str(&contents) {
                    Ok(config) => {
                        println!("Конфигурация загружена из: {:?}", config_path);
                        return config;
                    }
                    Err(e) => {
                        eprintln!("Ошибка парсинга конфигурации: {}. Используются значения по умолчанию.", e);
                    }
                },
                Err(e) => {
                    eprintln!("Ошибка чтения конфигурации: {}. Используются значения по умолчанию.", e);
                }
            }
        } else {
            println!("Файл конфигурации не найден: {:?}. Используются значения по умолчанию.", config_path);
            println!("Создайте файл конфигурации, чтобы настроить терминал.");
        }
        
        Self::default()
    }
    
    /// Возвращает путь к файлу конфигурации
    pub fn get_config_path() -> PathBuf {
        if let Some(config_dir) = dirs::config_dir() {
            config_dir.join("terminal-emulator").join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }
    
    /// Создает пример конфигурационного файла
    pub fn create_example() -> Result<(), std::io::Error> {
        let config = Self::default();
        let toml_string = toml::to_string_pretty(&config)
            .expect("Не удалось сериализовать конфигурацию");
        
        let config_path = Self::get_config_path();
        
        // Создаем директорию, если её нет
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        fs::write(&config_path, toml_string)?;
        println!("Пример конфигурации создан: {:?}", config_path);
        
        Ok(())
    }
}
