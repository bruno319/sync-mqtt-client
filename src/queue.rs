use std::str::from_utf8;

pub trait Queue {
    fn is_empty(&self) -> bool;
    fn add(&mut self, value: String) -> Result<(), String>;
    fn consume_queue(&mut self) -> Vec<String>;
}

pub struct PersistenceQueue {
    storage: sled::Db,
    counter: i32,
}

impl PersistenceQueue {
    pub fn new() -> PersistenceQueue {
        let db = sled::open("db_test").unwrap();

        PersistenceQueue {
            counter: db.len() as i32,
            storage: db,
        }
    }
}

impl Queue for PersistenceQueue {
    fn is_empty(&self) -> bool {
        self.storage.is_empty()
    }

    fn add(&mut self, value: String) -> Result<(), String> {
        self.counter += 1;
        self.storage.insert(self.counter.to_string().as_bytes(), value.as_bytes())
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    fn consume_queue(&mut self) -> Vec<String> {
        let mut k_v = Vec::new();
        for item in self.storage.iter() {
            if let Ok(i) = item {
                k_v.push((from_utf8(&i.0.to_vec()).ok().unwrap_or("").parse::<i32>().unwrap(), from_utf8(&i.1.to_vec()).ok().unwrap_or("").to_string()))
            }
        }
        self.storage.clear();

        k_v.sort_by_key(|i| i.0);
        k_v.iter()
            .map(|i| i.1.clone())
            .collect::<Vec<String>>()
    }
}