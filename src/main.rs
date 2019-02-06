use ignore::DirEntry;

use sha2::{Digest, Sha256};

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

mod argparse;
mod backup;
mod restore;

use argparse::arg_parse;

#[derive(Serialize, Deserialize, Debug)]
struct SaveStruct {
    path: OsString,
    filename: OsString,
    hashes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct SaveStructArray(Vec<SaveStruct>);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = arg_parse()?;

    println!("{:?}", args);
    return Ok(());

    let home = dirs::home_dir().ok_or("could not open home.")?;
    let project_dir = home.join("Projects");
    let backup_dir = home.join("Backup");

    println!("project dir : {:?}", project_dir);
    println!("backup dir : {:?}", backup_dir);

    let walk = ignore::WalkBuilder::new(project_dir).hidden(false).build();

    let mut saves = Vec::new();
    for w in walk {
        let out = foreach_file(&w?, &backup_dir);

        if let Some(save) = out {
            println!("hashed file:{:?}", save.path);
            saves.push(save);
        }
    }

    let structure = serde_json::to_string(&SaveStructArray(saves))?;

    let mut structure_file = File::create(backup_dir.join("structure.json"))?;
    structure_file.write_all(structure.as_bytes())?;

    Ok(())
}

fn foreach_file(path: &DirEntry, backup_dir: &Path) -> Option<SaveStruct> {
    let mut buf = [0u8; 1024 * 1024];
    let mut read_file = if let Ok(file) = File::open(path.path()) {
        file
    } else {
        return None;
    };
    let mut reader = BufReader::new(&mut read_file);

    let mut hashes = Vec::new();

    let out_file_path = backup_dir.join("tmp");

    'a: loop {
        let mut out_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&out_file_path)
            .unwrap_or_else(|_| panic!("can not make tmp file"));

        let mut hasher = Sha256::new();

        let res = reader.read(&mut buf);

        match res {
            Ok(size) => {
                if size == 0 {
                    break 'a;
                }

                hasher.input(&buf[..size]);

                out_file
                    .write_all(&buf[..size])
                    .unwrap_or_else(|e| panic!("could not write to file : {:?}", e));
            }
            Err(_) => return None,
        }

        out_file.flush().expect("could not flush tmp file");

        let result = hasher.result().to_vec();
        let result_hex = hex::encode_upper(&result);
        let first_letter = result_hex
            .chars()
            .nth(0)
            .expect("This should not be happening");

        let mut index = BufReader::new(
            OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(backup_dir.join(&format!("{}.meta", first_letter.to_string())))
                .unwrap_or_else(|e| panic!("open error with {}: {:?}", first_letter, e)),
        );

        let mut str_buf = String::new();
        index
            .read_to_string(&mut str_buf)
            .unwrap_or_else(|_| panic!());
        let has_index: bool = str_buf
            .split('\n')
            .collect::<Vec<&str>>()
            .iter()
            .filter(|x| x != &&"")
            .map(|x| x.split_whitespace().collect::<Vec<&str>>()[0])
            .any(|x| x == result_hex);

        if has_index {
            continue;
        }

        let mut index = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(backup_dir.join(&format!("{}.meta", first_letter.to_string())))
                .unwrap_or_else(|e| panic!("open error with {}: {:?}", first_letter, e)),
        );

        let mut data = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(backup_dir.join(&first_letter.to_string()))
                .unwrap_or_else(|e| panic!("open error with {}: {:?}", first_letter, e)),
        );

        index
            .write_all(
                format!(
                    "{} {}\n",
                    result_hex,
                    fs::metadata(backup_dir.join(&first_letter.to_string()))
                        .expect("could not get metadata")
                        .len()
                )
                .as_bytes(),
            )
            .expect("could not write index");

        let mut tmp_file = File::open(&out_file_path).expect("could not open tmp file");

        let mut buf = [0u8; 1024 * 1024];
        loop {
            let l = tmp_file.read(&mut buf).unwrap_or_else(|_| panic!());
            //println!("outsize = {}", l);
            if l == 0 {
                break;
            }
            data.write_all(&buf[..l]).unwrap_or_else(|_| panic!());
        }

        hashes.push(result_hex.clone());
    }

    Some(SaveStruct {
        path: path.path().as_os_str().to_owned(),
        filename: path.file_name().to_os_string(),
        hashes,
    })
}
