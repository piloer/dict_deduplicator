use std::fs::File;
use std::io::Read;
use rusqlite::{Connection, params};
use std::sync::mpsc::{channel, Receiver, Sender};

pub struct WordLibrary{
    tx: Sender<(String,String)>,
    pub rx: Receiver<(String,String)>
}

impl WordLibrary {
    pub fn new() -> Self{
        let (tx, rx) = channel();
        Self{
            tx,
            rx
        }
    }
    pub fn db(&mut self,path: &str){
        let conn = Connection::open(path).unwrap();
        let mut stmt = conn.prepare("SELECT * FROM entry").unwrap();
        let rows = stmt.query_map(params![], |row| {
            Ok((row.get::<usize, String>(1)?, row.get::<usize, String>(2).unwrap()))
        }).unwrap();

        for row in rows {
            let (word, shortcut) = row.unwrap();

            let new_shortcut = shortcut.replace("'"," ");

            let tx = self.tx.clone();
            tx.send((word,new_shortcut)).expect("channel will be there waiting for the pool");

        }
    }
    pub fn dict(&mut self,path: &str){
        let mut file = File::open(path).unwrap();
        let mut data = String::new();
        let _ = file.read_to_string(&mut data);
        for line in data.lines().skip(13){
            let linevec: Vec<String> = line.to_string().split("\t").map(|x| x.to_string()).collect();
            if linevec[0].is_empty(){
                continue
            }

            let tx = self.tx.clone();

            let word = linevec[0].to_string();
            let shortcut;
            if linevec.len() < 3{
                shortcut = linevec[1].to_string();
            }else{
                let new_shortcut = format!("{}\t{}",linevec[1],linevec[2]);
                shortcut = new_shortcut;
            }

            tx.send((word,shortcut)).expect("channel will be there waiting for the pool");
        }
    }
}