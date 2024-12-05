#!/bin/bash

INSTALL_DIR="$HOME/.local/bin/todo"
RELEASE_DIR="target/release/todo_cli"
ALIAS_NAME="todo"
SHELL_CONFIG_BASH="$HOME/.bashrc"
SHELL_CONFIG_ZSH="$HOME/.zshrc"
selected=0
options=("bash" "zsh")

draw_menu() {
    echo "Choose your shell that you use"
    for i in "${!options[@]}"; do
        if [ "$i" -eq "$selected" ]; then
            echo -e "\e[1;32m> ${options[$i]}\e[0m"
        else
            echo "  ${options[$i]}"
        fi
    done
}

read_input() {
    read -rsn1 key
    if [[ "$key" == $'\e' ]]; then
        read -rsn2 key
        case "$key" in
            "[B") # Down
                ((selected++))
                if [ "$selected" -ge "${#options[@]}" ]; then
                    selected=0
                fi
                ;;
            "[A") # Up
                ((selected--))
                if [ "$selected" -lt 0 ]; then
                    selected=$((${#options[@]} - 1))
                fi
                ;;
        esac
    elif [[ "$key" == "" ]]; then
        return 0
    fi
    return 1
}

echo "Building Rust project..."
cargo build --release
if [ $? -ne 0 ]; then
    echo "Rust build failed. Please fix the errors and try again."
    exit 1
fi

echo "Installing Binary"
while true; do
    draw_menu
    read_input
    if [ $? -eq 0 ]; then
        case ${options[$selected]} in
            "bash") SHELL_CONFIG_FILE="$HOME/.bashrc";;
            "zsh") SHELL_CONFIG_FILE="$HOME/.zshrc";;
        esac
        break
    fi
    clear
done
cp $RELEASE_DIR $INSTALL_DIR
echo "alias $ALIAS_NAME='$INSTALL_DIR'" >> "$SHELL_CONFIG_FILE"
echo "Installation complete. Reopen your shell and then you can use the 'todo' command."
