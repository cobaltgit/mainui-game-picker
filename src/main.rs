use std::{fs, io};
use rand::Rng;
use serde_json::Value;
use regex::Regex;

fn get_config(file: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let file = fs::File::open(file)?;
    let reader = io::BufReader::new(file);
    let value: Value = serde_json::from_reader(reader)?;
    Ok(value)
}

fn listdir(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let path_str = path.to_str()?;
            (!path_str.starts_with('.')).then(|| path_str.to_string())
        })
        .collect();

    Ok(entries)
}

fn get_roms(system_path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let config: Value = get_config(&format!("{}/config.json", system_path))?;
    let rom_path: String = if let Some(rom_path_relative) = config["rompath"].as_str() {
        if rom_path_relative.starts_with("/") { // absolute path
            rom_path_relative.to_string()
        } else { // assume relative path
            format!("{}/{}", system_path, rom_path_relative)
        }
    } else {
        let system_name = system_path.rsplit('/').next().unwrap_or(system_path);
        format!("/mnt/SDCARD/Roms/{}", system_name)
    };
    
    let all_roms: Vec<String> = listdir(&rom_path)?; // todo: subfolders
    if config["extlist"].is_null() {
        return Ok(all_roms);
    }

    let extensions: &str = config["extlist"].as_str().ok_or("Missing extlist field!")?;
    let re: Regex = Regex::new(&format!(r"\.({})$", extensions))?;

    let roms: Vec<String> = all_roms.into_iter()
        .filter(|s| re.is_match(s))
        .collect();
            
    Ok(roms)
 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::rng();
    let systems: Vec<String> = listdir("/mnt/SDCARD/Emus")?;
    loop {
        let chosen_system: &str = &systems[rng.random_range(0..systems.len())];
        match get_roms(chosen_system) {
            Ok(roms) if !roms.is_empty() => {
                let chosen_rom: &str = &roms[rng.random_range(0..roms.len())];
                println!("{}", chosen_rom);
                break;
            }
            _ => continue,
        }
    }
    Ok(())
}
