use crate::instruction::Instruction;
use std::fmt::{Display, Formatter};
use std::fmt;

pub struct TranslatedProgram {
    pub lines: Vec<TranslatedLine>
}

impl TranslatedProgram {
    pub fn new() -> TranslatedProgram {
        return TranslatedProgram {
            lines: Vec::<TranslatedLine>::new()
        }
    }
}

impl Display for TranslatedProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for line in &self.lines {
            writeln!(f,"{}",line)?;
        }
        return Ok(());
    }
}

pub enum TranslatedLine {
    InstructionLine(Instruction, Option<String>),
    CommentLine(String),
    EmptyLine,
}

impl Display for TranslatedLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self {
            TranslatedLine::InstructionLine(instruction, comment) => match comment {
                Some(comment) => write!(f,"{} #{}", instruction, comment),
                None => write!(f,"{}", instruction)
            },
            TranslatedLine::CommentLine(comment) => write!(f, "#{}", comment),
            TranslatedLine::EmptyLine => write!(f,""),
        }
    }
}