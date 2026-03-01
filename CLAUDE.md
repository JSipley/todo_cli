# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build & Test Commands

- **Build:** `cargo build` (or `cargo build --release` for optimized binary)
- **Run:** `cargo run -- <command>` where command is `new`, `view`, or `done`
- **Run all tests:** `cargo test`
- **Run a single test:** `cargo test <test_name>` (e.g., `cargo test read_valid_task_data`)

## Architecture

Rust CLI app for managing a to-do list with XML-based persistence. Single crate, no workspaces.

**Modules:**
- `main.rs` — CLI entry point, parses args (`new`/`view`/`done`) via `match`, orchestrates user interaction via stdin. Creates a local `TaskManager` and passes it by reference to helper functions.
- `models.rs` — `Task` struct (description, due_date as Strings, important as bool). Implements `Display`.
- `task_manager.rs` — `TaskManager` holds a private `Vec<Task>`, provides CRUD operations (`new()`, `add_task()`, `fetch_tasks()`, `remove_task()` with bounds checking) and delegates saving to xml_parser.
- `xml_parser.rs` — Reads/writes tasks from/to XML using the `xml-rs` crate. XML elements: `<Task>` containing `<Description>`, `<Due_Date>`, `<Important>`. Parsing tracks the current element name to assign fields correctly regardless of order. Reads `"true"`/`"y"` as important, writes `"true"`/`"false"`.

**Data flow:** On startup, `task_database.xml` is read into a local `TaskManager`. After the user's command executes, all tasks are written back to the same file via a full overwrite (`File::create`). If the file doesn't exist and the command is `new`, the app proceeds with an empty task list.

**XML format quirk:** The written XML has no wrapping root element — each `<Task>` is a top-level element. This is technically malformed XML but `xml-rs` parses it correctly. Do not add a root element without updating both read and write.

**Parser behavior:** `xml_parser::read` preserves field text verbatim (no trimming). Whitespace inside `<Description>`, `<Due_Date>`, or `<Important>` is kept as-is. `Important` accepts `"true"` or `"y"` (case-insensitive) as truthy; all other values are false.

**Task IDs:** The `done` command displays and accepts 0-based task indices matching the order in `task_database.xml`.

**Test fixtures:** XML test files live in `xml_test_files/`. Write tests use temp directories and clean up after themselves.
