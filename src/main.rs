use console::style;
use crate::arguments::get_program_arguments;
use crate::information::CURRENT_INFORMATION;
use crate::files::{open_program_files, read_file_to_string, ProgramFiles, write_string_to_file};
use assemblerlib::{AssemblerSettings, Assembler};


mod files;
mod arguments;
mod information;

fn main() {
    // Parse Command Line Arguments into Program Arguments
    let args = match get_program_arguments(CURRENT_INFORMATION) {
        Ok(args) => args,
        Err(err) => return handle_error(&err)
    };

    // Open Input and Output File
    let mut files: ProgramFiles = match open_program_files(&args) {
        Ok(files) => files,
        Err(err) => return handle_error(&err)
    };

    // Read Input File
    let input_file_content: String = match read_file_to_string(&mut files.input_file) {
        Ok(content) => content,
        Err(_) => return handle_error("Could not Read Input File!")
    };

    // Actually Translate the Program
    let assembler_settings: AssemblerSettings = AssemblerSettings {
        copy_instructions: args.copy_instructions,
        copy_comments: args.copy_comments
    };

    let assembler: Assembler = Assembler::new_with_settings(assembler_settings);

    let translated_program = match assembler
        .translate_program(input_file_content) {
        Ok(program) => program,
        Err(err) => return handle_error(&err)
    };

    // Write the Translated Program to the Output File
    let translated_string = translated_program.to_string();

    match write_string_to_file(translated_string, &mut files.output_file) {
        Ok(_) => {},
        Err(_) => return handle_error("Could not Write to Output File")
    };

    println!("Successfully translated the Program");
    return;
}

fn handle_error(error: &str) {
    eprintln!("{}", style(error).red());
}

