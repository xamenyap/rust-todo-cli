# Todo CLI

A simple command-line todo application written in Rust.

## Description

This program allows you to manage a list of todo items. You can add new items, display existing items, and exit the program. Todo items are stored in a `todo.txt` file, with each line containing a date and a task.

## Usage

1.  **Clone the repository:**

    ```bash
    git clone git@github.com:xamenyap/rust-todo-cli.git
    cd rust-todo-cli
    ```

2.  **Build the project:**

    ```bash
    cargo build
    ```

3.  **Run the program:**

    ```bash
    cargo run
    ```

## Features

*   **Display Todo Items:** Shows a sorted list of todo items from the `todo.txt` file, ordered by date.
*   **Add Todo Item:** Prompts you to enter a new todo item, which is then added to the `todo.txt` file with the current date.
*   **Exit:** Exits the program.

## Todo File Format

The `todo.txt` file stores todo items in the following format:

```
YYYY-MM-DD  Task description
```

For example:

```
2025-05-22  Buy groceries
2025-05-23  Walk the dog
```

## Notes

*   The program assumes that the `todo.txt` file exists in the same directory as the executable. If `todo.txt` does not exist, it will be created upon running the program.
*   The date format is `YYYY-MM-DD`.
