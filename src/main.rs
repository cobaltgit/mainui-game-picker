use rand::Rng;
use serde_json::Value;
use std::{env, fs, io, process};
use std::os::unix::process::CommandExt;

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

#[inline]
fn has_extension(path: &str, extensions: &[&str]) -> bool {
    extensions.iter().any(|ext| {
        path.to_lowercase().ends_with(&format!(".{}", ext.to_lowercase()))
    })
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

    let extlist: &str = config["extlist"].as_str().ok_or("")?;
    let extensions: Vec<&str> = extlist.split("|").collect();

    let roms: Vec<String> = all_roms.into_iter()
        .filter(|s| has_extension(s, &extensions))
        .collect();
            
    Ok(roms)
 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argv: Vec<String> = env::args().collect();
    let should_launch = argv.contains(&"--launch".to_string());
    let posargs: Vec<String> = argv.into_iter().filter(|a| a != "--launch").collect();
    let system_arg: String = posargs.get(1).map_or("NULL", |v| v).to_string();
    
    let mut rng = rand::rng();
    
    let emu_path: &str = if fs::metadata("/mnt/SDCARD/Emu").is_ok() {
        "/mnt/SDCARD/Emu"
    } else {
        "/mnt/SDCARD/Emus"
    };
    let systems: Vec<String> = listdir(emu_path)?;
    
    let mut chosen_system: &str = if posargs.len() > 1 {
        &format!("{}/{}", emu_path, system_arg)
    } else {
        &systems[rng.random_range(0..systems.len())]
    };

    if !fs::metadata(chosen_system).is_ok() || system_arg.starts_with("/") || system_arg.contains("..") {
        eprintln!("{}: invalid or nonexistent directory", chosen_system);
        process::exit(1)
    }

    loop {
        match get_roms(&chosen_system) {
            Ok(roms) if !roms.is_empty() => {
                let chosen_rom: &str = &roms[rng.random_range(0..roms.len())];
                println!("{}", chosen_rom);

                if should_launch {
                    let config = get_config(&format!("{}/config.json", chosen_system))?;
                    let launch_path: String = if let Some(launch_path_relative) = config["launch"].as_str() {
                        if launch_path_relative.starts_with("/") { // absolute path
                            launch_path_relative.to_string()
                        } else { // assume relative path
                            format!("{}/{}", chosen_system, launch_path_relative)
                        }
                    } else { // default to /mnt/SDCARD/Emu(s)/<system>/launch.sh
                        let system_name = chosen_system.rsplit('/').next().unwrap_or(chosen_system);
                        format!("{}/{}/launch.sh", emu_path, system_name)
                    };
                    
                    let _ = process::Command::new("/bin/sh")
                        .arg(&launch_path)
                        .arg(&chosen_rom)
                        .exec();
                }
                
                break;
            }
            _ => {
                if posargs.len() == 1 {
                    chosen_system = &systems[rng.random_range(0..systems.len())]
                }
                continue;
            }
        }
    }
    
    Ok(())
}
