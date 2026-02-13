use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, HeaderBar};
use gtk::gdk::RGBA;
use vte::{TerminalExt, TerminalExtManual};

mod config;
use config::Config;

const APP_ID: &str = "com.example.TerminalEmulator";

fn main() {
    // Создаем GTK приложение
    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    // Подключаем обработчик активации
    app.connect_activate(build_ui);

    // Запускаем приложение
    app.run();
}

fn build_ui(app: &Application) {
    // Загружаем конфигурацию
    let config = Config::load();
    
    // Создаем терминальный виджет
    let terminal = vte::Terminal::new();
    
    // Применяем настройки цвета фона
    if let Ok(color) = parse_color(&config.terminal.background_color) {
        terminal.set_color_background(&color);
    } else {
        eprintln!("Некорректный цвет фона: {}", config.terminal.background_color);
    }
    
    // Применяем внутренние отступы к терминалу
    if config.window.padding > 0 {
        apply_terminal_padding(&terminal, config.window.padding);
    }
    
    // Создаем главное окно
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(config.window.width)
        .default_height(config.window.height)
        .child(&terminal)
        .title(&config.window.title)
        .build();

    // Применяем стиль заголовка
    apply_headerbar_style(&window, &config.window.headerbar_style, &config.terminal.background_color);
    
    // Применяем обводку окна
    apply_window_border(
        &window, 
        config.window.border_width, 
        &config.window.border_color, 
        config.window.border_radius,
        &config.terminal.background_color
    );
    
    // Применяем прозрачность (opacity должна быть от 0.0 до 1.0)
    let opacity = config.window.opacity.clamp(0.0, 1.0);
    window.set_opacity(opacity);
    
    // Обработка выхода из терминала (Ctrl+D, exit и т.д.)
    let window_clone = window.clone();
    terminal.connect_child_exited(move |_, _status| {
        println!("Терминал завершен, закрываем окно");
        window_clone.close();
    });
    
    // Запускаем оболочку в терминале
    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string());
    
    // Используем spawn_async с правильной сигнатурой для версии 0.5
    terminal.spawn_async(
        vte::PtyFlags::DEFAULT,
        None,  // рабочая директория (None = текущая)
        &[shell.as_str()],  // команда для запуска
        &[],  // переменные окружения (пустой массив = наследуем)
        glib::SpawnFlags::DEFAULT,
        || {},  // child setup
        -1,  // timeout
        None::<&gtk::gio::Cancellable>,  // cancellable
        |_terminal, _pid, error| {
            if let Some(err) = error {
                eprintln!("Ошибка при запуске оболочки: {}", err);
            }
        },
    );

    // Показываем окно
    window.show();
}

/// Применяет стиль заголовка окна
fn apply_headerbar_style(window: &ApplicationWindow, style: &str, bg_color: &str) {
    match style {
        "integrated" | "монолитный" => {
            // Создаем HeaderBar (монолитный заголовок)
            let headerbar = HeaderBar::new();
            headerbar.set_show_title_buttons(true); // Показываем кнопки (закрыть, свернуть, развернуть)
            
            // Применяем цвет фона к headerbar
            if let Ok(color) = parse_color(bg_color) {
                // Создаем CSS для стилизации headerbar
                // Используем rgb() формат для GTK CSS
                let css = format!(
                    "headerbar {{ 
                        background-color: rgb({}, {}, {});
                        background-image: none;
                        border: none;
                        box-shadow: none;
                        color: rgba(255, 255, 255, 0.9);
                    }}
                    headerbar button {{
                        color: rgba(255, 255, 255, 0.9);
                    }}",
                    (color.red() * 255.0) as u8,
                    (color.green() * 255.0) as u8,
                    (color.blue() * 255.0) as u8
                );
                
                apply_css(&css);
            }
            
            window.set_titlebar(Some(&headerbar));
        }
        "standard" | "стандартный" | _ => {
            // Стандартный заголовок (по умолчанию в GTK4)
            // Ничего не делаем, GTK использует системный titlebar
        }
    }
}

/// Применяет обводку окна
fn apply_window_border(_window: &ApplicationWindow, border_width: i32, border_color: &str, border_radius: i32, bg_color: &str) {
    let mut css_parts = Vec::new();
    
    // Применяем border-radius если задан
    if border_radius > 0 {
        css_parts.push(format!("border-radius: {}px;", border_radius));
    }
    
    // Применяем обводку если задана
    if border_width > 0 {
        // Проверяем, является ли это градиентом
        if border_color.trim().starts_with("linear_gradient") {
            // Парсим градиент
            if let Some(gradient_css) = parse_gradient(border_color) {
                // Используем трюк с двойным background для поддержки border-radius
                // Метод аналогичный браузерному CSS!
                
                // Получаем цвет фона окна
                let window_bg = if let Ok(color) = parse_color(bg_color) {
                    format!("rgb({}, {}, {})",
                        (color.red() * 255.0) as u8,
                        (color.green() * 255.0) as u8,
                        (color.blue() * 255.0) as u8
                    )
                } else {
                    "rgb(30, 30, 30)".to_string()
                };
                
                css_parts.push(format!("border: {}px solid transparent;", border_width));
                
                // Два слоя background:
                // 1. Слой с цветом фона (padding-box)
                // 2. Слой с градиентом (border-box)
                css_parts.push(format!(
                    "background: linear-gradient({bg}, {bg}) padding-box, {gradient} border-box;",
                    bg = window_bg,
                    gradient = gradient_css
                ));
            } else {
                eprintln!("Некорректный формат градиента: {}", border_color);
            }
        } else {
            // Обычный цвет
            if let Ok(color) = parse_color(border_color) {
                css_parts.push(format!(
                    "border: {}px solid rgb({}, {}, {});",
                    border_width,
                    (color.red() * 255.0) as u8,
                    (color.green() * 255.0) as u8,
                    (color.blue() * 255.0) as u8
                ));
            } else {
                eprintln!("Некорректный цвет обводки: {}", border_color);
            }
        }
    }
    
    // Применяем CSS если есть стили
    if !css_parts.is_empty() {
        let css = format!("window {{\n    {}\n}}", css_parts.join("\n    "));
        apply_css(&css);
    }
}

/// Применяет внутренние отступы к терминалу
fn apply_terminal_padding(_terminal: &vte::Terminal, padding: i32) {
    let css = format!(
        "vte-terminal {{
    padding: {}px;
}}",
        padding
    );
    
    apply_css(&css);
}

/// Парсит строку градиента и преобразует в CSS формат
/// Формат входной строки: "linear_gradient to right bottom #color1 #color2 ..."
/// Результат: "linear-gradient(to right bottom, #color1, #color2, ...)"
fn parse_gradient(gradient_str: &str) -> Option<String> {
    let gradient_str = gradient_str.trim();
    
    // Убираем "linear_gradient" в начале
    if !gradient_str.starts_with("linear_gradient") {
        return None;
    }
    
    let rest = gradient_str["linear_gradient".len()..].trim();
    
    // Парсим направление и цвета
    let parts: Vec<&str> = rest.split_whitespace().collect();
    
    if parts.is_empty() {
        return None;
    }
    
    let mut direction = String::new();
    let mut colors = Vec::new();
    let mut idx = 0;
    
    // Ищем направление (to ...)
    if parts[idx] == "to" {
        idx += 1;
        // Собираем направление пока не встретим цвет (начинается с #)
        while idx < parts.len() && !parts[idx].starts_with('#') {
            if !direction.is_empty() {
                direction.push(' ');
            }
            direction.push_str(parts[idx]);
            idx += 1;
        }
        direction = format!("to {}", direction);
    }
    
    // Собираем цвета
    while idx < parts.len() {
        if parts[idx].starts_with('#') {
            colors.push(parts[idx].to_string());
        }
        idx += 1;
    }
    
    if colors.is_empty() {
        return None;
    }
    
    // Формируем CSS градиент
    if direction.is_empty() {
        Some(format!("linear-gradient({})", colors.join(", ")))
    } else {
        Some(format!("linear-gradient({}, {})", direction, colors.join(", ")))
    }
}

/// Применяет CSS стили к приложению
fn apply_css(css: &str) {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css.as_bytes());
    
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::StyleContext::add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

/// Парсит цвет из строки (hex формат: #RRGGBB)
fn parse_color(color_str: &str) -> Result<RGBA, String> {
    let color_str = color_str.trim();
    
    if !color_str.starts_with('#') || color_str.len() != 7 {
        return Err(format!("Неверный формат цвета. Ожидается #RRGGBB, получено: {}", color_str));
    }
    
    let r = u8::from_str_radix(&color_str[1..3], 16)
        .map_err(|_| format!("Неверный красный компонент: {}", &color_str[1..3]))?;
    let g = u8::from_str_radix(&color_str[3..5], 16)
        .map_err(|_| format!("Неверный зелёный компонент: {}", &color_str[3..5]))?;
    let b = u8::from_str_radix(&color_str[5..7], 16)
        .map_err(|_| format!("Неверный синий компонент: {}", &color_str[5..7]))?;
    
    Ok(RGBA::new(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
        1.0,
    ))
}
