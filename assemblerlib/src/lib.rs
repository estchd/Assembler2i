use crate::translated::{TranslatedProgram, TranslatedLine};
use crate::parsing::{CodeLine, parse_line};

mod instruction;
mod parsing;
pub mod translated;

pub struct AssemblerSettings {
    pub copy_comments: bool,
    pub copy_instructions: bool
}

pub struct Assembler {
    settings: AssemblerSettings
}

impl Assembler {
    pub fn new() -> Assembler {
        return Assembler {
            settings: AssemblerSettings {
                copy_comments: false,
                copy_instructions: false
            }
        };
    }

    pub fn new_with_settings(settings: AssemblerSettings) -> Assembler {
        return Assembler {
            settings
        };
    }

    pub fn settings(&mut self, settings: AssemblerSettings) -> &mut Self {
        self.settings = settings;
        return self;
    }

    pub fn copy_comments(&mut self, copy_comments: bool) -> &mut Self {
        self.settings.copy_comments = copy_comments;
        return self;
    }

    pub fn copy_instructions(&mut self, copy_instructions: bool) -> &mut Self {
        self.settings.copy_instructions = copy_instructions;
        return self;
    }

    pub fn translate_program(&self, program_string: &str) -> Result<TranslatedProgram, String> {
        let input_lines = program_string.lines();
        let mut code_lines = Vec::<CodeLine>::new();
        for (i,line) in input_lines.enumerate() {
            let code_line = match parse_line(line) {
                Ok(code_line) => {code_line},
                Err(error) => {
                    return Err(format!("Error in Line {}, Description: {}", i, error));
                }
            };
            code_lines.push(code_line);
        }
        let mut program = TranslatedProgram::new();
        for line in code_lines {
            program.lines.push(TranslatedLine::InstructionLine(line.instruction, line.comment))
        }

        return Ok(program)
    }

    pub fn translate_line(&self, line: &str) {

    }
}