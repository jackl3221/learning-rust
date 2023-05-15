
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::process::Command;

const SPECIAL_CHAR: &str = "!@#$%^&*()_+-=[]{}|;':\",./<>?\\";

fn create_file(path: &str) {
    let mut file = match File::create(path) {
        Ok(file) => file,
        Err(e) => panic!("Unable to create file {}: {}", path, e),
    };
    match file.write_all(b"") {
        Ok(_) => println!("Successfully created file {}", path),
        Err(e) => panic!("Unable to write data to file {}: {}", path, e),
    }
}

fn remove_file(path: &str) {
    match std::fs::remove_file(path) {
        Ok(_) => println!("Successfully removed file {}", path),
        Err(error) => {
            if error.kind() == std::io::ErrorKind::NotFound {
                println!("File not found");
            } else {
                println!("Error deleting file: {:?}", error)
            }
        }
    }
}

fn is_mounted(path: &str) -> bool {
    if let Ok(mounts_file) = File::open("/proc/mounts") {
        let reader = BufReader::new(mounts_file);
        for line in reader.lines() {
            if let Ok(entry) = line {
                let parts: Vec<&str> = entry.split(' ').collect();
                if parts.len() >= 2 && parts[1] == path {
                    return true;
                }
            }
        }
    }
    false
}

fn check_mounted(device: &str) -> bool {
    let output = Command::new("mount").output().unwrap();
    let mount_output = String::from_utf8(output.stdout).unwrap();
    mount_output.contains(device)
}

fn clear_eos(string: &mut String) {
    if let Some('\0') = string.chars().last() {
        string.pop();
    }
}

fn read_line_from_file(filename: &str) -> Result<String, std::io::Error> {
    let file = File::open(filename)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    line = line.trim_end().to_string();
    Ok(line)
}

fn Convert_Units(total_size:u64, str_size: &mut String)
{
    let usb_size: u64;

    if (total_size / 1048576) < 1
    {
        if (total_size / 1024) >= 1
        {
            /* partition < 1G */
            /* partition need to > 1M */
            usb_size = total_size / 1024;
            *str_size = format!("{:.2} MB", usb_size);
        }
    }
    else
    {
        /* partition >= 1G */
        usb_size = total_size / 1048576;
        *str_size = format!("{:.2} GB", usb_size);
    }
}

fn convert_char(s: &mut str) {
    let len = SPECIAL_CHAR.len();
    for i in 0..s.len() {
        for j in 0..len {
            if s[i] == SPECIAL_CHAR[j] {
                s.replace_range(i..=i, "_");
                break;
            }
        }
        if s[i] == "\"" || s[i..=i] == "\\" {
            s.replace_range(i..=i, "_");
        }
    }
}

fn main() {

    let path = String::from("1.txt");
    create_file(&path);
    remove_file(&path);
    
    let device = "/dev/sda1";
    let is_mounted = check_mounted(device);
    if is_mounted {
        println!("{} is mounted.", device);
    } else {
        println!("{} is not mounted.", device);
    }

    let mut string = String::from("Hello, world!\0");
    println!("Before clear: {:?}", string);
    clear_eos(&mut string);
    println!("After clear: {:?}", string);
    
    let mut disk_size = String::new();
    Convert_Units(101594104, &mut disk_size);
    println!("disk_size = {}", disk_size);

    let filename = "11.txt";
    let line = read_line_from_file(filename).expect("Error reading file");
    println!("Read line: {}", line);
    
    let mut s = String::from("Hello, world!@#\\$%^&*()");
    convert_char(&mut s);
    println!("{}", s); // "Hello, world______"
}
