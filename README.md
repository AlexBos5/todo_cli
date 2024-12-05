# todo_cli

## Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Commands](#commands)

## Description

This is a basic CLI program designed for creating `todo.md` files and making them interactive through the command line.
This is done through a new bash alias `todo` which allows for new commands to manage tasks in the `todo.md` file.
For a list of supported commands view the [Commands](#Commands) chapter.

## Installation

To run the installation run:

```bash
bash install.sh
```

This will run the installation script which creates a binary file which is then stored in `~/.local/bin/todo`.
When running the installation script you will receive a menu asking if you want to install this for thte bash or zsh shell. This part is to setup the alias automatically in either your `.bashrc` or `.zshrc`.
After installing the `todo` alias will be setup but and will require you to reopen your shell in order for the new command to work.

## Usage

The core usage of the `todo_cli` program after installation is performed through command-line inputs prefixed with the alias `todo`. An example of this is:

```bash
todo help
```

This command will display the usage guidelines for the program: ![alt text](image.png)

## Commands

```
Command - Description

todo help - Shows the usage of the help command.
todo help commands - Shows a list of all the commands.
todo createList - Creates a new todo.md unless one exists.
todo list - Displays all the current lines in the todo.md.
todo deletelist - Deletes the current todo.md.
todo add <text> - Adds the inputted <text> as a new line in todo.md.
todo del <entry_num> - Deletes the line at line number <entry_num> from todo.md.
```
