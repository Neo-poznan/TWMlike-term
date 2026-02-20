#!/usr/bin/env bash
# Скрипт быстрой установки Terminal Emulator

set -e

# Цвета
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     Terminal Emulator - Установка             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════╝${NC}"
echo ""

# Определяем PREFIX
if [ -w "/usr/bin" ]; then
    PREFIX="/usr"
else
    PREFIX="/usr/local"
fi

echo -e "${YELLOW}Будет использован PREFIX: ${PREFIX}${NC}"
echo -e "${YELLOW}Для изменения используйте: PREFIX=/путь $0${NC}"
echo ""

# Используем make для сборки и установки
if [ ! -f "Makefile" ]; then
    echo -e "${RED}Ошибка: Makefile не найден!${NC}"
    exit 1
fi

echo -e "${BLUE}Запуск установки через Makefile...${NC}"
echo ""

# Если нужны права sudo
if [ "$PREFIX" = "/usr" ] || [ "$PREFIX" = "/usr/local" ]; then
    if [ "$EUID" -ne 0 ]; then
        echo -e "${YELLOW}Требуются права суперпользователя для установки в ${PREFIX}${NC}"
        echo -e "${YELLOW}Попытка использовать sudo...${NC}"
        echo ""
        sudo make install PREFIX="$PREFIX"
    else
        make install PREFIX="$PREFIX"
    fi
else
    make install PREFIX="$PREFIX"
fi

echo ""
echo -e "${GREEN}╔════════════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║  ✓ Установка завершена!                       ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════════════╝${NC}"
