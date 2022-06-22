use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("1st argument missing");
    let value = arguments.next().unwrap();
    println!("The key is '{key}' and the value is '{value}'");
    let contents = format!("{key}\t{value}\n");
    match std::fs::write("kv.db", contents) {
        Ok(()) => println!("Write successful"),
        Err(err) => println!("{err}"),
    }
    let database = Database::new().expect("Database initialization crashed.");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        /* let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(err) => {
                return Err(err);
            }
        }; */
        let mut map = HashMap::new();
        let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map: map
        })
    }
}
