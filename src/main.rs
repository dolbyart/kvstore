use std::collections::HashMap;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let key = arguments.next().expect("1st argument missing");
    let value = arguments.next().expect("2nd argument missing");
    println!("The key is '{key}' and the value is '{value}'");
    /* let contents = format!("{key}\t{value}\n");
    match std::fs::write("kv.db", contents) {
        Ok(()) => println!("Write successful"),
        Err(err) => println!("{err}"),
    } */
    let mut database = Database::new().expect("Database initialization crashed.");
    database.insert_key_value(key.clone(), value.clone());
    database.insert_key_value(key.to_uppercase(), value);
    match database.flush_map() {
        Ok(()) => println!("FLush kvs to kv.db"),
        Err(e) => println!("Failed to flush, error: {:?}", e),
    };
    //can't do this, because database is moved to flush_map
    //database.insert_key_value(String::from("OK"), String::from("BITCH"));
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        let contents = match std::fs::read_to_string("kv.db") {
            Ok(c) => c,
            Err(err) => return Err(err),
        };
        //let contents = std::fs::read_to_string("kv.db")?;
        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("Corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }
        Ok(Database {
            map,
        })
    }

    fn insert_key_value(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush_map(self) -> std::io::Result<()> {
        let mut contents = String::new();
        for (key, value) in &self.map {
            //let kv_pair = format!("{}\t{}\n", key, value);
            /*
            contents = contents + &kv_pair; */
            //contents.push_str(&kv_pair);
            /* contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n'); */
            contents.push_str(format!("{}\t{}\n", key, value).as_str());
        }
        std::fs::write("kv.db", contents)
    }
}
