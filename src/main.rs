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
use chrono;

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
    let target_file_exists = Path::new(target_path).is_file();
    println!("Watching {}", Path::new(target_path).display());
    let target_dir_exists = Path::new(target_path).exists();
    if !target_file_exists && !target_dir_exists{
        println!("Please supply a valid directory and/or filename");
        println!("Usage : $ watcher [/path/][filename] ");
        return Ok(());
    }
    watcher.watch(target_path, RecursiveMode::Recursive);
    // If the user supplies only a path then we'll display all
    // files which are altered, but won't tail any file.
    if target_file_exists{
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
    println!("processFile -- The file is {} bytes long.",fileLength);

    loop {
        match receiver.recv() {
            Ok(mut event) =>  {  
            match &mut event{
                notify::DebouncedEvent::NoticeWrite(_) => {
                    displayFile((target_path).to_string(), &mut prevFileLength);
                    println!("prevFileLength : {}", prevFileLength);
                },
                notify::DebouncedEvent::Error(err, Some(file_path)) => {
                    println!("Error: {}, {}",err,file_path.as_path().display().to_string())
                },
                other => {},
                //other => {println!("{:?}",other)},
                // notify::DebouncedEvent::NoticeRemove(_) => {},
                // notify::DebouncedEvent::Write(_) => {},
                // notify::DebouncedEvent::Chmod(_) => {},
                // notify::DebouncedEvent::Remove(_) => {}
                // notify::DebouncedEvent::Rename(_, _) => {}
                // notify::DebouncedEvent::Rescan => {}
                // notify::DebouncedEvent::Error(_, _) => {},
                // notify::DebouncedEvent::Create(x) => {}
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
        //println!("{}", receiver.recv().unwrap());
        match receiver.recv() {
           Ok(event) => {
            let current_time = chrono::offset::Local::now().format("%Y-%m-%d %H:%M:%S");
            match  event{
                notify::DebouncedEvent::Write(file_path) => {println!("{} Write: {}",current_time,file_path.as_path().display().to_string())},
                notify::DebouncedEvent::Chmod(file_path) => {println!("{} Chmod: {}",current_time,file_path.as_path().display().to_string())},
                notify::DebouncedEvent::Remove(file_path) => {println!("{} Remove: {}",current_time,file_path.as_path().display().to_string())},
                notify::DebouncedEvent::Rename(file_path, _) => {println!("{} Rename: {}",current_time,file_path.as_path().display().to_string())},
                notify::DebouncedEvent::Error(err,Some(file_path)) => {println!("{} Error: {}, {}",current_time,err,file_path.as_path().display().to_string())},
                notify::DebouncedEvent::Create(file_path) => {println!("{} Create: {}",current_time,file_path.as_path().display().to_string())},
                other => {}
                }
            },
           Err(e) => println!("watch error: {:?}", e),
        }
    }
}

fn displayFile(path : String, readBytePosition : &mut u64)-> std::io::Result<()>{

    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    println!("{}",buf_reader.seek(SeekFrom::End(0))?);
    let mut contents = String::new();
    
    //buf_reader.seek(SeekFrom::Start(499))?;
    
    buf_reader.seek(SeekFrom::Start(*readBytePosition))?;
    //println!("The file is {} bytes long.",fileLength);
    //let currentFilePos = buf_reader.seek(SeekFrom::Start(*prevFileLength))?;
    
    let before = buf_reader.stream_position()?;
    buf_reader.read_to_string(&mut contents);
    print!("{}",contents);
    
    // buf_reader.read_line(&mut String::new())?;
    let after = buf_reader.stream_position()?;
    //inLength += after;
    *readBytePosition += after-before;

    println!("The line is {} bytes long", after - before);
    
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