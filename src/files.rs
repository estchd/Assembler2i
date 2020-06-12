use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use dialoguer::Confirm;
use crate::arguments::ProgramArguments;

pub struct ProgramFiles {
    pub input_file: File,
    pub output_file: File,
}

pub fn open_program_files(program_arguments: &ProgramArguments) -> Result<ProgramFiles,String> {
    let input_file = File::open(&program_arguments.input_file_path)
        .map_err(|_| "Could not open Input File".to_string())?;

    let output_file = try_create_output_file(&program_arguments.output_file_path)?;

    return Ok(ProgramFiles {
        input_file,
        output_file
    });
}

pub fn try_create_output_file(output_file_path: &str) -> Result<File, String> {
    if Path::new(output_file_path).exists() && !ask_for_output_file_overwrite() {
        return Err("Could not open Output File!".to_string());
    }

    return File::create(output_file_path)
        .map_err(|_| "Could not open Output File!".to_string());
}

pub fn ask_for_output_file_overwrite() -> bool {
    return Confirm::new()
        .with_prompt("Output file already exists, overwrite?")
        .interact()
        .unwrap_or(false);
}

pub fn read_file_to_string(file: &mut File) -> Result<String, ()> {
    let mut content: String = String::new();
    file.read_to_string(&mut content).map_err(|_| ())?;
    return Ok(content);
}

pub fn write_string_to_file(content: String, file: &mut File) -> Result<(), ()> {
    return file.write_all(content.as_bytes()).map_err(|_| ());
}