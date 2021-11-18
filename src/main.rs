use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;
use std::sync::mpsc::Receiver;
use notify::DebouncedEvent;

fn main() -> std::io::Result<()> {
    // get commmand line args -- need path to watch
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("Please supply a valid directory and/or filename.");
        println!("Usage : $ watcher [/path/][filename] ");
        return Ok(());
    }

    // Create a channel to receive the events.
    let (sender, receiver) = channel();
    
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    let mut target_path = &args[1];
    let targetFileExists = Path::new(target_path).is_file();
    let targetDirExists = Path::new(target_path).exists();
    if !targetFileExists && !targetDirExists{
        println!("Please supply a valid directory and/or filename");
        println!("Usage : $ watcher [/path/][filename] ");
        return Ok(());
    }
    watcher.watch(target_path, RecursiveMode::Recursive).unwrap();
    // If the user supplies only a path then we'll display all
    // files which are altered, but won't tail any file.
    if targetFileExists{
        processFile((target_path).to_string(), receiver,true);
        Ok(())
    }
    else{
        processDirectory(receiver);
        Ok(())
    }
}

fn processFile(mut target_path : String, receiver : Receiver<DebouncedEvent>, continuous : bool)-> std::io::Result<()>{
    let file = File::open(&mut target_path)?;
    let mut fileLength : u64 = 0;
    let mut prevFileLength : u64 = 0;
    let mut buf_reader = BufReader::new(file);
        fileLength = buf_reader.seek(SeekFrom::End(0))?;
        prevFileLength = fileLength;
        println!("The file is {} bytes long.",fileLength);

        loop {
            match receiver.recv() {
                Ok(mut event) =>  {  
                match &mut event{
                    notify::DebouncedEvent::NoticeWrite(_) => {
                        //println!("Uhg! it worked!");
                        displayFile((target_path).to_string(), fileLength, &mut prevFileLength);
                        println!("prevFileLength : {}", prevFileLength);
                    },
                    notify::DebouncedEvent::NoticeRemove(_) => {}
                    notify::DebouncedEvent::Write(_) => {},
                    notify::DebouncedEvent::Chmod(_) => {},
                    notify::DebouncedEvent::Remove(_) => {}
                    notify::DebouncedEvent::Rename(_, _) => {}
                    notify::DebouncedEvent::Rescan => {}
                    notify::DebouncedEvent::Error(_, _) => {},
                    notify::DebouncedEvent::Create(x) => {}
                }
                //println!("{:?}", event);
                },
                Err(e) => println!("watch error: {:?}", e),
            }
            if !continuous{
                return Ok(());
            }
            //displayFile((target_path).to_string(), fileLength, &mut prevFileLength);
            
            // displayFile(Path::new(target_path).as_os_str().to_str().unwrap().to_string(), fileLength);
        }
        
}

fn processDirectory(receiver: Receiver<DebouncedEvent>){
    loop {
        match receiver.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
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

#[test]
fn test_processFile(){
    let (sender, receiver) = channel();
    
    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(1)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    
    watcher.watch("3rd.one", RecursiveMode::Recursive).unwrap();
    processFile("3rd.one".to_string(), receiver, false);

}