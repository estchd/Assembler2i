pub struct ProgramInformation<'t> {
    pub name: &'t str,
    pub description: &'t str,
    pub version: &'t str,
    pub author: &'t str
}

const VERSION: &str = "0.1";
const PROGRAM_NAME: &str = "Assembler2i";
const AUTHOR: &str = "Erik Schulze";
const PROGRAM_DESCRIPTION: &str = "An Assembly Program for the 2i Microcomputer Language";

pub const CURRENT_INFORMATION: ProgramInformation = ProgramInformation {
    name: PROGRAM_NAME,
    description: PROGRAM_DESCRIPTION,
    version: VERSION,
    author: AUTHOR
};