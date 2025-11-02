# Memora

**Memora** is an in-memory key-value database written in Rust. It supports storing multiple keys and values per record, with flexible data types for both keys and values.

---

## Features

* **Create records** with customizable key and value types.
* **Set keys** in a record.
* **Insert multiple values** for each key.
* **Retrieve keys and values** from records.
* **Count keys and values**.
* **Print keys or values** for inspection.
* Supports the following data types:

  * **Keys:** INT, STRING
  * **Values:** INT, STRING, DECIMAL, DATETIME (work in progress)

---

## Commands

Run the program and use the following commands:

| Command                                                                  | Description                                             |
| ------------------------------------------------------------------------ | ------------------------------------------------------- |
| `create record <record_name> with key <key_type> and value <value_type>` | Create a new record with specified key and value types. |
| `set key <key> in <record_name>`                                         | Add a key to an existing record.                        |
| `insert value <value> for key <key> in <record_name>`                    | Insert a value into a key in the specified record.      |
| `get key <key> from <record_name>`                                       | Retrieve all values for a key.                          |
| `count keys of <record_name>`                                            | Count the number of keys in a record.                   |
| `count values of <key_name> in <record_name>`                            | Count the number of values for a key.                   |
| `print keys of <record_name>`                                            | Print all keys in a record.                             |
| `print values of <key_name> in <record_name>`                            | Print all values of a key.                              |
| `--help`                                                                 | Show available commands.                                |
| `q`                                                                      | Quit the program.                                       |

---

## Installation

1. Clone the repository:

```bash
git clone https://github.com/lorennnzzoo/memora.git
cd memora
```

2. Build the project with Cargo:

```bash
cargo build --release
```

3. Run the program:

```bash
cargo run
```

---

## Example Usage

```text
> create record users with key INT and value STRING
Record users created successfully

> set key 101 in users
created key 101 in users successfully

> insert value "John Doe" for key 101 in users
inserted value successfully

> get key 101 from users
["John Doe"]

> count keys of users
1

> print values of 101 in users
["John Doe"]
```

## Notes

* DATETIME value type is under development.

---

This README gives users a clear understanding of how to use **Memora**, including setup, usage, and available commands.
