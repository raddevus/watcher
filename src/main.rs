use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

fn main() -> std::io::Result<()> {
    // get commmand line args -- need path to watch
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut prevFileLength : u64 = 0;
    
    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let target_path = &args[1];
    watcher.watch(target_path, RecursiveMode::Recursive).unwrap();

    let file = File::open(target_path)?;
    let mut fileLength : u64 = 0;
    // If the user supplies only a path then we'll display all
    // files which are altered, but won't tail any file.
    if Path::new(target_path).is_file(){
        let mut buf_reader = BufReader::new(file);
        fileLength = buf_reader.seek(SeekFrom::End(0))?;
        prevFileLength = fileLength;
        println!("The file is {} bytes long.",fileLength);
    }
        
    loop {
        match receiver.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
        displayFile((target_path).to_string(), fileLength, &mut prevFileLength);
        println!("prevFileLength : {}", prevFileLength);
        // displayFile(Path::new(target_path).as_os_str().to_str().unwrap().to_string(), fileLength);
    }
   
}

fn displayFile(path : String, mut inLength: u64, prevFileLength : &mut u64)-> std::io::Result<()>{
    println!("in displayFile() {:?}", path);
    // if inLength <= 0 then no file has been passed (only path)
    if inLength > 0{
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        
        let mut contents = String::new();
        //buf_reader.seek(SeekFrom::Start(499))?;
        
        let fileLength = buf_reader.seek(SeekFrom::End(0))?;
        println!("The file is {} bytes long.",fileLength);
        let currentFilePos = buf_reader.seek(SeekFrom::Start(*prevFileLength))?;
        
        let before = buf_reader.stream_position()?;
        buf_reader.read_to_string(&mut contents);
        print!("{}",contents);
        
        // buf_reader.read_line(&mut String::new())?;
        let after = buf_reader.stream_position()?;
        inLength += after;
        *prevFileLength += after-before;

        println!("The line is {} bytes long", after - before);
    }
    Ok(())
}