use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{self, Read, Write},
    process::exit,
};
// use time::{OffsetDateTime, macros::format_description};
const AOF_FILE: &str = "memora.aof";
const CREATE: &str = "create";
const SET: &str = "set";
const INSERT: &str = "insert";
const GET: &str = "get";
const COUNT: &str = "count";
const PRINT: &str = "print";
#[derive(Debug)]

enum KeyType {
    INT,
    STRING,
}
#[derive(Debug)]

enum ValueType {
    INT,
    STRING,
    DECIMAL,
    DATETIME,
}
#[derive(Debug)]
struct Record {
    key_type: KeyType,
    value_type: ValueType,
    data: HashMap<String, Vec<String>>,
}

impl Record {
    fn new(key_type: KeyType, value_type: ValueType) -> Record {
        Record {
            key_type,
            value_type,
            data: HashMap::new(),
        }
    }
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

struct memora;
impl memora {
    fn write_aof(command: String) {
        let command_with_new_line = command + "\n";
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(AOF_FILE)
            .expect("Failed to open aof file");

        file.write_all(command_with_new_line.as_bytes())
            .expect("Failed to write to aof");
    }
    fn reload_aof() -> Option<String> {
        let file = File::open(AOF_FILE);

        match file {
            Ok(mut file) => {
                let mut file_content = String::new();
                file.read_to_string(&mut file_content).unwrap();
                Some(file_content)
            }
            Err(_) => None,
        }
    }
}

// fn parse_datetime(s: &str) -> Result<OffsetDateTime, String> {
//     let format = format_description!("[year]-[month]-[day]-[hour]:[minute]:[second]");
//     OffsetDateTime::parse(s, &format).map_err(|e| format!("Invalid datetime '{}': {}", s, e))
// }

fn main() {
    println!(
        "This is memora, a key value in-memory database\nTo know what commands to use run : --help\ntype q and Enter to quit"
    );
    println!("Initializing the database....");
    let mut database = Database::new();
    let aof_content = memora::reload_aof();

    match aof_content {
        Some(content) => {
            let commands: Vec<&str> = content.lines().collect();

            for command in commands {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if !parts.is_empty() {
                    process_command(parts, &mut database, false);
                }
            }
        }
        None => {}
    }

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
                "Available commands:\n\
                    1. create record <record_name> with key <key_type> and value <value_type>\n\
                    2. set key <key> in <record_name>\n\
                    3. insert value <value> for key <key> in <record_name>\n\
                    4. get key <key> from <record_name>\n\
                    5. count keys of <record_name>\n\
                    6. count values of <key_name> in <record_name>\n\
                    7. print keys of <record_name>\n\
                    8. print values of <key_name> in <record_name>\n"
            );
        } else {
            process_command(command_items, &mut database, true);
        }
    }
}

fn process_command(command_items: Vec<&str>, database: &mut Database, is_aof: bool) {
    match command_items[0] {
        CREATE => {
            process_create_command(command_items, database, is_aof);
        }
        SET => {
            process_set_command(command_items, database, is_aof);
        }
        INSERT => {
            process_insert_command(command_items, database, is_aof);
        }
        GET => {
            process_get_command(command_items, database);
        }
        COUNT => {
            process_count_command(command_items, database);
        }
        PRINT => {
            process_print_command(command_items, database);
        }
        _ => println!("Invalid Operation : {}", command_items[0]),
    }
}
fn process_count_command(command_items: Vec<&str>, database: &Database) {
    if command_items.len() > 6 || command_items.len() < 4 {
        println!("Please enter a valid command")
    } else {
        match command_items[1].to_uppercase().trim() {
            "KEYS" => {
                if command_items[2] != "of" {
                    println!("missing \"of\" keyword")
                } else {
                    match database.records.get(command_items[3]) {
                        Some(record) => {
                            println!("{}", record.data.keys().len())
                        }
                        None => println!("record {} does not exist", command_items[3]),
                    }
                }
            }
            "VALUES" => {
                if command_items[2] != "of" {
                    println!("missing \"of\" keyword")
                } else {
                    if command_items[4] != "in" {
                        println!("missing \"in\" keyword")
                    }
                    match database.records.get(command_items[5]) {
                        Some(record) => match record.data.get(command_items[3]) {
                            Some(data) => {
                                println!("{}", data.len())
                            }
                            None => {
                                println!(
                                    "key {} does not exist in record {}",
                                    command_items[3], command_items[5],
                                )
                            }
                        },
                        None => println!("record {} does not exist", command_items[3]),
                    }
                }
            }
            _ => {
                println!("Invalid keyword : {}, Use keys/values", command_items[1])
            }
        }
    }
}
fn process_print_command(command_items: Vec<&str>, database: &Database) {
    if command_items.len() > 6 || command_items.len() < 4 {
        println!("Please enter a valid command")
    } else {
        match command_items[1].to_uppercase().trim() {
            "KEYS" => {
                if command_items[2] != "of" {
                    println!("missing \"of\" keyword")
                } else {
                    match database.records.get(command_items[3]) {
                        Some(record) => {
                            println!("{:?}", record.data.keys())
                        }
                        None => println!("record {} does not exist", command_items[3]),
                    }
                }
            }
            "VALUES" => {
                if command_items[2] != "of" {
                    println!("missing \"of\" keyword")
                } else {
                    if command_items[4] != "in" {
                        println!("missing \"in\" keyword")
                    }
                    match database.records.get(command_items[5]) {
                        Some(record) => match record.data.get(command_items[3]) {
                            Some(data) => {
                                println!("{:?}", data)
                            }
                            None => {
                                println!(
                                    "key {} does not exist in record {}",
                                    command_items[3], command_items[5],
                                )
                            }
                        },
                        None => println!("record {} does not exist", command_items[3]),
                    }
                }
            }
            _ => {
                println!("Invalid keyword : {}, Use keys/values", command_items[1])
            }
        }
    }
}

fn process_create_command(command_items: Vec<&str>, database: &mut Database, is_aof: bool) {
    if command_items.len() != 9 {
        println!("Please enter a valid command");
    } else {
        if command_items[1].to_lowercase() != "record" {
            println!("missing \"record\" keyword");
        } else {
            let record_name = command_items[2];
            if database.records.contains_key(command_items[2]) {
                println!("record \"{}\" already exists", command_items[2]);
            } else {
                if command_items[3].to_lowercase() != "with" {
                    println!("missing \"with\" keyword")
                } else {
                    process_key_validations(command_items, &record_name, database, is_aof);
                }
            }
        }
    }
}
fn process_set_command(command_items: Vec<&str>, database: &mut Database, is_aof: bool) {
    if command_items.len() != 5 {
        println!("Please enter a valid command");
    } else {
        if command_items[1] != "key" {
            println!("missing \"key\" keyword");
        } else {
            match database.records.get_mut(command_items[4]) {
                Some(record) => match record.key_type {
                    KeyType::INT => {
                        match command_items[2].to_string().parse::<i64>() {
                            Ok(parsed_value) => {
                                //set key for record
                                if record.data.contains_key(command_items[2]) {
                                    println!(
                                        "key {} already exists in {}",
                                        parsed_value, command_items[4]
                                    )
                                } else {
                                    record.data.insert(parsed_value.to_string(), Vec::new());

                                    if is_aof {
                                        memora::write_aof(command_items.join(" ").to_string());
                                    }
                                    println!(
                                        "created key {} in {} successfully",
                                        command_items[2], command_items[4]
                                    );
                                }
                            }
                            Err(_) => println!(
                                "value {} cannot be parsed to datatype {:?}",
                                command_items[2], record.key_type
                            ),
                        }
                    }
                    KeyType::STRING => {
                        match command_items[2].to_string().parse::<String>() {
                            Ok(parsed_value) => {
                                //set key for record
                                if record.data.contains_key(command_items[2]) {
                                    println!(
                                        "key {} already exists in {}",
                                        parsed_value, command_items[4]
                                    )
                                } else {
                                    record.data.insert(parsed_value.to_string(), Vec::new());
                                    if is_aof {
                                        memora::write_aof(command_items.join(" ").to_string());
                                    }
                                    println!(
                                        "created key {} in {} successfully",
                                        command_items[2], command_items[4]
                                    );
                                }
                            }
                            Err(_) => println!(
                                "key {} cannot be parsed to datatype {:?}",
                                command_items[2], record.key_type
                            ),
                        }
                    }
                },

                None => println!("record {} does not exist", command_items[4]),
            }
        }
    }
}
fn process_insert_command(command_items: Vec<&str>, database: &mut Database, is_aof: bool) {
    if command_items.len() != 8 {
        println!("Please enter a valid command");
    } else {
        if command_items[1] != "value" {
            println!("missing \"value\" keyword")
        } else {
            let value_to_insert = command_items[2];
            match database.records.get_mut(command_items[7]) {
                Some(record) => match record.value_type {
                    ValueType::INT => {
                        process_value_parsing_for_insertion(
                            command_items,
                            ValueType::INT,
                            record,
                            value_to_insert,
                            is_aof,
                        );
                    }
                    ValueType::STRING => {
                        process_value_parsing_for_insertion(
                            command_items,
                            ValueType::STRING,
                            record,
                            value_to_insert,
                            is_aof,
                        );
                    }
                    ValueType::DECIMAL => {
                        process_value_parsing_for_insertion(
                            command_items,
                            ValueType::DECIMAL,
                            record,
                            value_to_insert,
                            is_aof,
                        );
                    }
                    ValueType::DATETIME => {
                        process_value_parsing_for_insertion(
                            command_items,
                            ValueType::DATETIME,
                            record,
                            value_to_insert,
                            is_aof,
                        );
                    }
                },

                None => println!("record {} does not exist", command_items[7]),
            }
        }
    }
}

fn process_get_command(command_items: Vec<&str>, database: &Database) {
    if command_items.len() != 5 {
        println!("please enter a valid command")
    } else {
        if command_items[1] != "key" {
            println!("missing \"key\" keyword")
        } else {
            match database.records.get(command_items[4]) {
                Some(record) => match record.data.get(command_items[2]) {
                    Some(data) => {
                        println!("{:?}", data)
                    }
                    None => println!(
                        "key {} does not exist in record {}",
                        command_items[2], command_items[4]
                    ),
                },
                None => println!("record {} does not exist", command_items[4]),
            }
        }
    }
}

fn process_value_parsing_for_insertion(
    command_items: Vec<&str>,
    value_type: ValueType,
    record: &mut Record,
    value_to_insert: &str,
    is_aof: bool,
) {
    match record.data.get(command_items[5]) {
        Some(_) => match value_type {
            ValueType::INT => match value_to_insert.parse::<i64>() {
                Ok(_) => {
                    insert_into_record(
                        command_items[5].to_string(),
                        value_to_insert.to_string(),
                        record,
                    );
                    if is_aof {
                        memora::write_aof(command_items.join(" ").to_string());
                    }
                }
                Err(_) => println!(
                    "value {} cannot be parsed to datatype {:?}",
                    value_to_insert, value_type
                ),
            },
            ValueType::STRING => match value_to_insert.parse::<String>() {
                Ok(_) => {
                    insert_into_record(
                        command_items[5].to_string(),
                        value_to_insert.to_string(),
                        record,
                    );
                    if is_aof {
                        memora::write_aof(command_items.join(" ").to_string());
                    }
                }
                Err(_) => println!(
                    "value {} cannot be parsed to datatype {:?}",
                    value_to_insert, value_type
                ),
            },
            ValueType::DECIMAL => match value_to_insert.parse::<f64>() {
                Ok(_) => {
                    insert_into_record(
                        command_items[5].to_string(),
                        value_to_insert.to_string(),
                        record,
                    );
                    if is_aof {
                        memora::write_aof(command_items.join(" ").to_string());
                    }
                }
                Err(_) => println!(
                    "value {} cannot be parsed to datatype {:?}",
                    value_to_insert, value_type
                ),
            },
            ValueType::DATETIME => {
                println!("datetime type is still under development")
            } // match parse_datetime(value_to_insert) {
              //     Ok(parsed_value) => {}
              //     Err(err) => println!(
              //         "value {} cannot be parsed to datatype {:?}\nerror : {}",
              //         value_to_insert, value_type, err
              //     ),
              // },
        },
        None => println!(
            "key {} does not exist in {}",
            command_items[5], command_items[7]
        ),
    }
}

fn insert_into_record(key: String, value: String, record: &mut Record) {
    record.data.entry(key.clone()).or_default().push(value);
    println!("inserted value successfully")
}

fn process_key_validations(
    command_items: Vec<&str>,
    record_name: &str,
    database: &mut Database,
    is_aof: bool,
) {
    if command_items[4].to_lowercase() != "key" {
        println!("missing \"key\" keyword")
    } else {
        match command_items[5].to_uppercase().trim() {
            "INT" => {
                process_value_validations_and_create_record(
                    command_items,
                    KeyType::INT,
                    record_name,
                    database,
                    is_aof,
                );
            }
            "STRING" => {
                process_value_validations_and_create_record(
                    command_items,
                    KeyType::STRING,
                    record_name,
                    database,
                    is_aof,
                );
            }
            _ => println!(
                "Invalid key type {} please use : INT/STRING",
                command_items[5]
            ),
        }
    }
}

fn process_value_validations_and_create_record(
    command_items: Vec<&str>,
    key_type: KeyType,
    record_name: &str,
    database: &mut Database,
    is_aof: bool,
) {
    if command_items[6].to_lowercase() != "and" {
        println!("missing \"and\" keyword");
    } else {
        if command_items[7].to_lowercase() != "value" {
            println!("missing \"value\" keyword");
        } else {
            let value_type_str = command_items[8].trim().to_uppercase();

            let value_type_enum = match value_type_str.as_str() {
                "INT" => ValueType::INT,
                "STRING" => ValueType::STRING,
                "DECIMAL" => ValueType::DECIMAL,
                "DATETIME" => ValueType::DATETIME,
                _ => {
                    println!(
                        "Invalid value type {}. Please use: INT, STRING, DECIMAL, DATETIME",
                        command_items[8]
                    );
                    return;
                }
            };

            database.records.insert(
                record_name.to_string(),
                Record::new(key_type, value_type_enum),
            );
            if is_aof {
                memora::write_aof(command_items.join(" ").to_string());
            }
            println!("Record {} created successfully", record_name);
        }
    }
}
