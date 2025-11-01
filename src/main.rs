use std::{collections::HashMap, io, process::exit};

const CREATE: &str = "create";
const SET: &str = "set";
const INSERT: &str = "insert";

enum key_type {
    INT,
    STRING,
}

enum value_type {
    INT,
    STRING,
    DECIMAL,
    DATETIME,
}

struct Record {
    key_type: key_type,
    value_type: value_type,
    data: HashMap<String, String>,
}

struct Database {
    records: HashMap<String, Record>,
}

impl Database {
    fn new() -> Database {
        Database {
            records: HashMap::new(),
        }
    }
}

fn main() {
    println!(
        "This is memora, a key value in-memory database\nTo know what commands to use run : memora --help\ntype q and Enter to quit"
    );
    println!("Initializing the database....");
    let mut database = Database::new();
    println!("Done Initializing the database");
    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Invalid user input.");
        let trimmed_command: String = command.trim().to_lowercase();
        // println!("the value of input is :{}this", trimmed_input);
        //string splitting
        if trimmed_command.is_empty() {
            println!("Please emter a valid command");
        }
        let command_items: Vec<&str> = trimmed_command.split(' ').collect();

        // if command_items.len() < 2 || command_items.len() > 4 {
        //     if trimmed_command != "--help" {
        //         if command_items.len() < 2 {
        //             println!("command is too short");
        //         }
        //         if command_items.len() > 4 {
        //             println!("command is too long");
        //         }
        //     }
        // }
        if trimmed_command == "q" {
            println!("Are you sure you want to quit? type Y");
            let mut choice = String::new();
            io::stdin().read_line(&mut choice).expect("invalid command");
            if choice.trim().to_lowercase() == "y" {
                exit(0)
            } else {
                continue;
            }
        } else if trimmed_command == "--help" {
            println!(
                "Here are the list of commands you can use\nTo create a record : create record <record_name> with key <key_type> and value <value_type>\nTo set a key in a record : set key <key> in <record_name>\nTo insert values to keys that are set in a record : insert value <value> for key <key> in <record_name>"
            )
        } else {
            process_command(command_items, &mut database);
        }
    }
}

fn process_command(command_items: Vec<&str>, database: &Database) {
    match command_items[0] {
        CREATE => {
            if command_items.len() != 9 {
                println!("Please enter a valid command");
            } else {
                if command_items[1].to_lowercase() != "record" {
                    println!("missing \"record\" keyword");
                } else {
                    if database.records.contains_key(command_items[2]) {
                        println!("record \"{}\" already exists", command_items[2]);
                    } else {
                        if command_items[3].to_lowercase() != "with" {
                            println!("missing \"with\" keyword")
                        } else {
                            if command_items[4].to_lowercase() != "key" {
                                println!("missing \"key\" keyword")
                            } else {
                                match command_items[5].to_uppercase().trim() {
                                    "INT" => {
                                        if command_items[6].to_lowercase() != "and" {
                                            println!("missing \"and\" keyword");
                                        } else {
                                            if command_items[7].to_lowercase() != "value" {
                                                println!("missing \"value\" keyword");
                                            } else {
                                                match command_items[8].to_uppercase().trim() {
                                                    "INT" => {}
                                                    "STRING" => {}
                                                    "DECIMAL" => {}
                                                    "DATETIME" => {}
                                                    _ => println!(
                                                        "Invalid value type {} please use : INT/STRING/DECIMAL/DATETIME",
                                                        command_items[8]
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                    "STRING" => {
                                        if command_items[6].to_lowercase() != "and" {
                                            println!("missing \"and\" keyword");
                                        } else {
                                            if command_items[7].to_lowercase() != "key" {
                                                println!("missing \"value\" keyword");
                                            } else {
                                                match command_items[8].to_uppercase().trim() {
                                                    "INT" => {}
                                                    "STRING" => {}
                                                    "DECIMAL" => {}
                                                    "DATETIME" => {}
                                                    _ => println!(
                                                        "Invalid value type {} please use : INT/STRING/DECIMAL/DATETIME",
                                                        command_items[8]
                                                    ),
                                                }
                                            }
                                        }
                                    }
                                    _ => println!(
                                        "Invalid key type {} please use : INT/STRING",
                                        command_items[5]
                                    ),
                                }
                            }
                        }
                    }
                }
            }
        }
        SET => {}
        INSERT => {}
        _ => println!("Invalid Operation : {}", command_items[0]),
    }
}
