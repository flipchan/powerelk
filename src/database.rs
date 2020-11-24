#![allow(dead_code)]
use sled::IVec;
use std::io::Read;

pub struct Database {
    pub filelocation: String,
}

impl Database {
    /// find entry in database
    pub fn find(&mut self, findme: String) -> String {
        let tree = sled::open(self.filelocation.as_str()).expect("open");
        let result: String = match tree.get(findme) {
            Ok(Some(val)) => String::from_utf8(val.to_vec()).unwrap(),
            _ => "nope".to_string(),
        };
        result //.to_string()
    }

    /// get entry from database
    pub fn get(self, entryid: String) -> Result<Option<IVec>, Box<dyn std::error::Error>> {
        let tree = sled::open(self.filelocation.as_str()).expect("open");
        match tree.get(&entryid).unwrap() {
            Some(t) => return Ok(Some(t)),
            _ => return Ok(None),
        }
    }

    /// store entry in the database
    pub fn store(self, entryid: String, data: serde_json::Value) -> bool {
        let tree = sled::open(self.filelocation.as_str()).expect("open");

        let storeme: String = serde_json::to_string(&data).unwrap();
        tree.insert(entryid, storeme.as_str())
            .map_err(|err| println!("{:?}", err))
            .ok();
        tree.flush().map_err(|err| println!("{:?}", err)).ok();
        true
    }

    /// remove from database
    pub fn remove(self, fluff: String) -> bool {
        let tree = sled::open(self.filelocation).expect("open");
        tree.remove(fluff).map_err(|err| println!("{:?}", err)).ok();
        true
    }

    /// convert a sled::ivec::IVec to String
    pub fn ivec2string(self, original: IVec) -> Result<String, std::io::Error> {
        Ok(String::from_utf8(original.to_vec()).unwrap())
    }

    pub fn ivvectou8(self, original: IVec) -> Result<u8, std::io::Error> {
        original.bytes().next().unwrap()
    }
}
