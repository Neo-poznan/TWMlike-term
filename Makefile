# Makefile for Terminal Emulator
# Простой настраиваемый эмулятор терминала на Rust + GTK4 + VTE4

# Переменные
PREFIX ?= /usr/local
BINDIR = $(PREFIX)/bin
DATADIR = $(PREFIX)/share
# Desktop файлы всегда должны быть в /usr/share/applications
APPLICATIONSDIR = /usr/share/applications
PIXMAPSDIR = /usr/share/pixmaps
ICONDIR = $(DATADIR)/icons/hicolor/scalable/apps

BINARY_NAME = terminal-emulator
DESKTOP_FILE = terminal-emulator.desktop
ICON_FILE = TWMlike-term-logo.png
TARGET_DIR = target/release

# Цвета для вывода
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
NC = \033[0m # No Color

.PHONY: all check-deps check-rust check-gtk check-vte build install uninstall clean help

# Цель по умолчанию
all: check-deps build

# Справка
help:
	@echo "$(BLUE)Terminal Emulator - Makefile$(NC)"
	@echo ""
	@echo "$(GREEN)Использование:$(NC)"
	@echo "  make              - Проверить зависимости и собрать проект"
	@echo "  make build        - Собрать проект (release)"
	@echo "  make install      - Установить в систему (по умолчанию в /usr/local)"
	@echo "  make uninstall    - Удалить из системы"
	@echo "  make clean        - Очистить собранные файлы"
	@echo "  make check-deps   - Проверить наличие зависимостей"
	@echo ""
	@echo "$(GREEN)Параметры:$(NC)"
	@echo "  PREFIX=/путь      - Указать префикс установки (по умолчанию: /usr/local)"
	@echo ""
	@echo "$(GREEN)Примеры:$(NC)"
	@echo "  make"
	@echo "  make install PREFIX=/usr"
	@echo "  sudo make install PREFIX=/usr"

# Проверка всех зависимостей
check-deps: check-rust check-gtk check-vte
	@echo "$(GREEN)✓ Все зависимости установлены$(NC)"

# Проверка Rust и Cargo
check-rust:
	@echo "$(BLUE)Проверка Rust...$(NC)"
	@if ! command -v cargo > /dev/null 2>&1; then \
		echo "$(RED)✗ Ошибка: cargo (Rust) не найден!$(NC)"; \
		echo "$(YELLOW)Установите Rust: https://rustup.rs/$(NC)"; \
		echo "$(YELLOW)Или выполните: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh$(NC)"; \
		exit 1; \
	fi
	@echo "$(GREEN)✓ Rust установлен: $$(cargo --version)$(NC)"

# Проверка GTK4
check-gtk:
	@echo "$(BLUE)Проверка GTK4...$(NC)"
	@if ! pkg-config --exists gtk4 2>/dev/null; then \
		echo "$(RED)✗ Ошибка: GTK4 не найден!$(NC)"; \
		echo "$(YELLOW)Установите GTK4:$(NC)"; \
		echo "  Ubuntu/Debian: sudo apt install libgtk-4-dev"; \
		echo "  Fedora:        sudo dnf install gtk4-devel"; \
		echo "  Arch Linux:    sudo pacman -S gtk4"; \
		exit 1; \
	fi
	@echo "$(GREEN)✓ GTK4 установлен: $$(pkg-config --modversion gtk4)$(NC)"

# Проверка VTE4
check-vte:
	@echo "$(BLUE)Проверка VTE4...$(NC)"
	@if ! pkg-config --exists vte-2.91-gtk4 2>/dev/null; then \
		echo "$(RED)✗ Ошибка: VTE4 не найден!$(NC)"; \
		echo "$(YELLOW)Установите VTE4:$(NC)"; \
		echo "  Ubuntu/Debian: sudo apt install libvte-2.91-gtk4-dev"; \
		echo "  Fedora:        sudo dnf install vte291-gtk4-devel"; \
		echo "  Arch Linux:    sudo pacman -S vte4"; \
		exit 1; \
	fi
	@echo "$(GREEN)✓ VTE4 установлен: $$(pkg-config --modversion vte-2.91-gtk4)$(NC)"

# Сборка проекта
build: check-deps
	@echo "$(BLUE)Сборка проекта...$(NC)"
	@cargo build --release
	@echo "$(GREEN)✓ Сборка завершена: $(TARGET_DIR)/$(BINARY_NAME)$(NC)"

# Установка
install: build
	@echo "$(BLUE)Установка Terminal Emulator...$(NC)"
	@echo "  PREFIX: $(PREFIX)"
	@echo "  BINDIR: $(BINDIR)"
	@echo "  APPLICATIONS: $(APPLICATIONSDIR)"
	@echo ""
	
	# Создаём директории
	@mkdir -p $(DESTDIR)$(BINDIR)
	@mkdir -p $(DESTDIR)$(APPLICATIONSDIR)
	@mkdir -p $(DESTDIR)$(PIXMAPSDIR)
	
	# Копируем бинарник
	@install -Dm755 $(TARGET_DIR)/$(BINARY_NAME) $(DESTDIR)$(BINDIR)/$(BINARY_NAME)
	@echo "$(GREEN)✓ Установлен бинарник: $(BINDIR)/$(BINARY_NAME)$(NC)"
	
	# Копируем desktop файл
	@install -Dm644 $(DESKTOP_FILE) $(DESTDIR)$(APPLICATIONSDIR)/$(DESKTOP_FILE)
	@echo "$(GREEN)✓ Установлен desktop файл: $(APPLICATIONSDIR)/$(DESKTOP_FILE)$(NC)"
	
	# Копируем иконку
	@install -Dm644 $(ICON_FILE) $(DESTDIR)$(PIXMAPSDIR)/twmlike-term-logo.png
	@echo "$(GREEN)✓ Установлена иконка: $(PIXMAPSDIR)/twmlike-term-logo.png$(NC)"
	
	# Обновляем базу данных desktop файлов
	@if command -v update-desktop-database > /dev/null 2>&1; then \
		update-desktop-database $(DESTDIR)$(APPLICATIONSDIR) 2>/dev/null || true; \
	fi
	
	@echo ""
	@echo "$(GREEN)╔════════════════════════════════════════════════╗$(NC)"
	@echo "$(GREEN)║  ✓ Установка завершена успешно!               ║$(NC)"
	@echo "$(GREEN)╚════════════════════════════════════════════════╝$(NC)"
	@echo ""
	@echo "$(YELLOW)Запуск:$(NC)"
	@echo "  $(BINARY_NAME)"
	@echo ""
	@echo "$(YELLOW)Конфигурация:$(NC)"
	@echo "  ~/.config/terminal-emulator/config.toml"
	@echo ""
	@echo "$(YELLOW)Пример конфигурации:$(NC)"
	@echo "  cp config.toml.example ~/.config/terminal-emulator/config.toml"

# Удаление
uninstall:
	@echo "$(BLUE)Удаление Terminal Emulator...$(NC)"
	@rm -f $(DESTDIR)$(BINDIR)/$(BINARY_NAME)
	@rm -f $(DESTDIR)$(APPLICATIONSDIR)/$(DESKTOP_FILE)
	@rm -f $(DESTDIR)$(PIXMAPSDIR)/twmlike-term-logo.png
	@if command -v update-desktop-database > /dev/null 2>&1; then \
		update-desktop-database $(DESTDIR)$(APPLICATIONSDIR) 2>/dev/null || true; \
	fi
	@echo "$(GREEN)✓ Terminal Emulator удалён$(NC)"

# Очистка
clean:
	@echo "$(BLUE)Очистка...$(NC)"
	@cargo clean
	@echo "$(GREEN)✓ Очистка завершена$(NC)"
