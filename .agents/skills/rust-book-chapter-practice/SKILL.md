---
name: rust-book-chapter-practice
description: Create a Rust practice Cargo project from a Rust Book chapter URL, with a README task that checks chapter understanding and includes a small application step. Use when the user sends a Rust Book chapter URL and asks for a chapter exercise, review task, practice project, or README prompt in this rust-book repository.
---

# Rust Book Chapter Practice

## Workflow

Use this skill to turn one Rust Book chapter URL into a local practice project that matches this repository's structure.

1. Read repository context before creating files.
   - Check `pwd`, `git status --short`, and existing top-level chapter project names.
   - Identify the naming convention, such as `c5_structs`, `c5_methods`, or `c5_practice_book_logger`.
   - Inspect nearby `Cargo.toml` files for edition and package naming style.

2. Read the chapter URL.
   - If the user provides a URL, browse or open it before designing the task.
   - Extract the chapter number/title and the main learning goals from the page.
   - If the chapter page is an index, use the chapter's section links or visible summary to infer the core concepts.

3. Design one practice task.
   - Make it broad enough to test understanding of the chapter.
   - Include a small application requirement beyond copying examples.
   - Keep it achievable in one small Cargo project.
   - Prefer a domain that makes the chapter concepts visible, such as structs/methods for Chapter 5 or ownership/borrowing for Chapter 4.
   - Include explicit requirements, expected functions/methods, behavior checks, and optional extensions.

4. Create the Cargo project in the existing style.
   - Use a directory name like `c{chapter}_practice_{short_slug}`.
   - Use `cargo new <dir>` unless the repository clearly creates projects another way.
   - Do not create a workspace unless the repository already uses one.
   - Match the local Cargo edition if existing projects consistently use one.
   - Avoid modifying unrelated chapter projects.

5. Write the task into `<project>/README.md`.
   - Include the chapter URL.
   - Include the purpose of the exercise.
   - Include concrete requirements.
   - Include a checklist of concepts being tested.
   - Write the README in Japanese.
   - Include suggested commands:

```bash
cargo run
cargo fmt -- --check
cargo clippy
```

   - Mention `cargo clippy` as optional if the project does not already use Clippy.

6. Leave implementation to the user unless they explicitly ask you to implement it.
   - It is OK for `src/main.rs` to contain Cargo's default starter code.
   - Do not solve the exercise in the generated project unless requested.

7. Verify the created project.
   - Run `cargo fmt -- --check` in the new project if only starter code and README were created.
   - Run `cargo check` if any Rust code was changed beyond `cargo new` defaults.
   - Report the created path and any commands run.

## README Shape

Use this structure unless this repository gains a stronger convention:

```markdown
# Chapter N Practice: Title

Rust Book: <chapter URL>

## 目的

<この課題で確認すること。>

## 課題

<作る小さなプログラム。>

## 要件

1. <Requirement>
2. <Requirement>
3. <Requirement>

## 動作確認例

<Example input/output or scenario.>

## 確認ポイント

- <Concept>
- <Concept>
- <Concept>

## 追加課題

- <Optional extension>
```

## Task Design Heuristics

- For structs/methods chapters, require `struct`, `impl`, `&self`, `&mut self`, and at least one associated function.
- For ownership/borrowing chapters, require one operation that consumes a value, one that borrows immutably, and one that borrows mutably.
- For enums/pattern matching chapters, require an enum with variants carrying data and a `match` that handles every variant.
- For collections chapters, require `Vec`, `String`, or `HashMap` plus boundary cases.
- For error handling chapters, require `Result`, `Option`, and at least one custom validation branch.

Keep the task specific. Avoid vague prompts like "make something using structs"; name the data model, required operations, and expected behavior.
