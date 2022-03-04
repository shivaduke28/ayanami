use std::{fs, path::Path};

pub const OUTPUT_FILE_NAME: &str = "render.png";
pub const BACKUP_FILE_NAME: &str = "render_bak.png";

pub fn backup() {
    let output_path = Path::new(OUTPUT_FILE_NAME);
    if output_path.exists(){
        println!("backup {:?} -> {:?}", OUTPUT_FILE_NAME, BACKUP_FILE_NAME);
        fs::rename(OUTPUT_FILE_NAME, BACKUP_FILE_NAME).unwrap();
    }
}