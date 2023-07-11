use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename: std::path::PathBuf = "foo.txt".into();

    {
        println!("create file");
        let mut file = File::create(filename.clone()).unwrap();
        file.write_all(b"Hello, world!").unwrap();
    }

    {
        println!("check that file has been saved");
        let meta = std::fs::metadata(filename.clone()).unwrap();
        println!("metadata: {:?}", meta);
    }

    {
        println!("reopen file");
        // in order to open file with append, explicitly use OpenOptions or write_all will fail
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .open(filename.clone())
            .unwrap();
        println!("append to file");
        file.write_all(b"Hello again!")
            .expect("file.write_all failed");
    }

    {
        println!("read file");
        let mut file = std::fs::File::open(filename.clone()).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        assert_eq!("Hello, world!Hello again!", content.as_str())
    }
}
