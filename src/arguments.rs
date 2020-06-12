use std::string::ToString;
use clap::{ArgMatches, App, Arg};
use crate::information::ProgramInformation;

pub struct ProgramArguments {
    pub copy_instructions: bool,
    pub copy_comments: bool,
    pub input_file_path: String,
    pub output_file_path: String
}

fn get_app(information: ProgramInformation) -> App {
    return App::new(information.name)
        .about(information.description)
        .version(information.version)
        .author(information.author)
        .arg(
            Arg::with_name("INPUT")
                .about("The Input File to translate, must have .2ia file type.")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::with_name("OUTPUT")
                .about("The Output File to write to, must have .2i file type.")
                .short('o')
                .long("output")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("COPY_INSTRUCTIONS")
                .about("Copy the Instructions as Comments.")
                .short('i')
                .long("instructions")
                .required(false)
        )
        .arg(
            Arg::with_name("COPY_COMMENTS")
                .about("Copy Comments and empty lines from the Input file.")
                .short('c')
                .long("comments")
                .required(false)
        );
}

pub fn get_program_arguments(information: ProgramInformation) -> Result<ProgramArguments, String> {
    let app = get_app(information);

    let matches = app.get_matches();

    return args_to_program_arguments(&matches);
}

fn args_to_program_arguments(args: &ArgMatches) -> Result<ProgramArguments, String> {

    let copy_instructions: bool = args_to_copy_instructions(args);
    let copy_comments: bool = args_to_copy_comments(args);

    let input_file_path = args_to_input_file_path(args)?;
    let output_file_path = args_to_output_file_path(&input_file_path, args)?;

    let program_arguments = ProgramArguments {
        copy_instructions,
        copy_comments,
        input_file_path,
        output_file_path
    };

    return Ok(program_arguments);
}

fn args_to_copy_instructions(args: &ArgMatches) -> bool {
    return args.is_present("COPY_INSTRUCTIONS");
}

fn args_to_copy_comments(args: &ArgMatches) -> bool {
    return args.is_present("COPY_COMMENTS");
}

fn args_to_input_file_path(args: &ArgMatches) -> Result<String, String> {
    let input_file_path = args.value_of("INPUT")
        .ok_or("Input File was not provided".to_string())?;

    if !input_file_path.ends_with(".2ia") {
        return Err("Input File does not have .2ia File Type".to_string());
    }

    return Ok(input_file_path.to_string());
}

fn args_to_output_file_path(input_file_path: &str, args: &ArgMatches) -> Result<String, String> {
    let output_file_path = match args.value_of("OUTPUT") {
        None => output_file_path_from_input_file_path(input_file_path),
        Some(file_path) => file_path.to_string()
    };

    if !output_file_path.ends_with(".2i") {
        return Err("Output File does not have .2i File Type".to_string());
    }

    return Ok(output_file_path);
}

fn output_file_path_from_input_file_path(input_file_path: &str) -> String {
    let mut output_file_path = input_file_path.to_string();
    output_file_path.replace_range(output_file_path.len() - 4..output_file_path.len(), ".2i");
    return output_file_path;
}