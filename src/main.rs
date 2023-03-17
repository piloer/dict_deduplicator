use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::process::exit;
use std::time::Duration;
use clap::Parser;
use dict_deduplicator::WordLibrary;

#[derive(Parser,Debug)]
struct Cli {
    #[arg(long,help = "gboard db file")]
    db: Option<String>,
    #[arg(long,help="rime dict file")]
    dict: Option<String>,

    #[arg(long,help="output file path",default_value = "gboard_sougou.dict.yaml")]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    if cli.db.is_none() && cli.dict.is_none(){
        println!("Use the --help command");
        exit(1);
    }

    let mut wl = WordLibrary::new();

    if !cli.db.is_none(){
        let dbpath = cli.db.unwrap();
        wl.db(&dbpath)
    }

    if !cli.dict.is_none(){
        let path = cli.dict.unwrap();
        wl.dict(&path)
    }

    let mut word_and_shortcut = vec![];
    let mut count = 0;
    while let Ok((word,shortcut) ) = wl.rx.recv_timeout(Duration::from_secs(3)){
        word_and_shortcut.push((word,shortcut));
        count += 1;
        println!("读取到第 {} 个",count);
    }

    let mut first_elements = HashSet::new();
    let mut output = Vec::new();

    for tuple in &word_and_shortcut {
        let first_element = &tuple.0;
        if first_elements.insert(first_element) {
            output.push(tuple.clone());
        }
    }

    let mut file = File::options()
        .create(true)
        .write(true)
        .open(cli.output.unwrap())
        .unwrap();

    let first_string = r#"---
name: luna_pinyin.sogou
version: "2022.3.31"
sort: by_weight
use_preset_vocabulary: true
...

"#;

    let _ = file.write_all(first_string.as_bytes());

    for (a,b) in &output{
        let dict = format!("{}\t{}\n",a,b);
        let _ = file.write_all(dict.as_bytes());
    }
    println!("去重复前: {}\n去重复后: {}",word_and_shortcut.len(),output.len());
}
