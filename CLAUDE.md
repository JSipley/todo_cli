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
- `main.rs` — CLI entry point, parses args (`new`/`view`/`done`), orchestrates user interaction via stdin. Uses a `static mut TaskManager` global.
- `models.rs` — `Task` struct (description, due_date, important as Strings). Implements `Display`.
- `task_manager.rs` — `TaskManager` holds a `Vec<Task>`, provides CRUD operations and delegates saving to xml_parser.
- `xml_parser.rs` — Reads/writes tasks from/to XML using the `xml-rs` crate. XML elements: `<Task>` containing `<Description>`, `<Due_Date>`, `<Important>`.

**Data flow:** On startup, `task_database.xml` is read into the global `TaskManager`. After the user's command executes, all tasks are written back to the same file. The app must be run from the directory containing `task_database.xml`.

**Test fixtures:** XML test files live in `xml_test_files/`.
