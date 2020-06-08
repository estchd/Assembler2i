use std::fmt;
use std::fmt::Formatter;
use crate::instruction::{Instruction, InstructionAddress, ParseError, RegisterAddress, ContentB, ALUFunction, ALUControl, RegisterControl, BusControl, AddressControl, RegisterAddressControl};
use std::convert::{TryFrom, Infallible};
use crate::parsing::WriteCommand::{WriteA, WriteB, WriteOff};
use crate::parsing::BusCommand::{BusRead, BusWrite, BusOff};
use crate::parsing::FlagCommand::{UpdateFlags, KeepFlags};
use std::slice::SplitN;
use crate::parsing::RegisterOrConstant::{Register, Constant};

pub struct CodeLine {
    instruction: Instruction,
    comment: Option<String>
}

impl fmt::Display for CodeLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match &self.comment{
            None => write!(f,"{}", self.instruction),
            Some(comment) => write!(f,"{} #{}", self.instruction, comment),
        }
    }
}

pub fn parse_line(line: &str) -> Result<CodeLine,String> {
    let split_comment: Vec<&str> = line.splitn(2,"#").collect();

    let comment: Option<String>;
    match split_comment.len() {
        0 => return Err("Nothing after Split".to_string()),
        1 => comment = None,
        2 => comment = Some(split_comment[1].to_string()),
        _ => return Err("Too much after Split".to_string())
    }

    let line_without_comment = split_comment[0].to_string();

    let split_instruction_line: Vec<&str> = line_without_comment.splitn(2,":").collect();

    match split_instruction_line.len() {
        0 => return Err("Nothing after Split".to_string()),
        1 => return Err("No Instruction Address".to_string()),
        2 => {},
        _ => return Err("Too much after Split".to_string())
    }

    let instruction_address_string = split_instruction_line[0];

    let instruction_address = match InstructionAddress::try_from(instruction_address_string.to_string()) {
        Ok(address) => address,
        Err(err) => return Err(format!("Error Parsing Instruction Address: {}", err.description))
    };

    let instruction_string = split_instruction_line[1];

    let split_instructions: Vec<&str> = instruction_string.split(";").collect();

    match split_instructions.len() {
        0..=4 => return Err("Too few Commands".to_string()),
        5 => {},
        _ => return Err("Too many Commands".to_string())
    }

    let alu_command_string = split_instructions[0];
    let write_command_string = split_instructions[1];
    let bus_command_string = split_instructions[2];
    let flag_command_string = split_instructions[3];
    let jump_command_string = split_instructions[4];

    let alu_command = match parse_alu_command(alu_command_string) {
        Ok(cmd) => cmd,
        Err(err) => return Err(format!("Error Parsing ALU Command: {}",err))
    };


    let write_command = match parse_write_command(write_command_string) {
        Ok(cmd) => cmd,
        Err(err) => return Err(format!("Error Parsing Write Command: {}", err))
    };

    let bus_command = match parse_bus_command(bus_command_string) {
        Ok(cmd) => cmd,
        Err(err) => return Err(format!("Error Parsing Bus Command: {}", err))
    };

    let flag_command = match parse_flag_command(flag_command_string) {
        Ok(cmd) => cmd,
        Err(err) => return Err(format!("Error Parsing Flag Command: {}", err))
    };

    let jump_command = match parse_jump_command(jump_command_string) {
        Ok(cmd) => cmd,
        Err(err) => return Err(format!("Error Parsing Jump Command: {}", err))
    };

    let alu_control = flag_command.to_alu_control();
    let alu_function = alu_command.to_alu_function();
    let register_control = write_command.to_register_control();
    let bus_control = bus_command.to_bus_control();
    let address_control = jump_command.jump_type.to_address_control();
    let register_contents = alu_command.to_content();
    let mut register_address_control = alu_command.to_register_address_control();
    register_address_control.aca = bus_control.bus_en && !bus_control.bus_wr;

    let instruction = Instruction {
        address: instruction_address,
        address_control,
        next_address: jump_command.next_address,
        bus_control,
        content_a: register_contents.content_a,
        content_b: register_contents.content_b,
        register_control,
        register_address_control,
        alu_function,
        alu_control
    };

    return Ok(CodeLine { instruction , comment });
}

enum RegisterOrConstant {
    Register(RegisterAddress),
    Constant(ContentB)
}

impl RegisterOrConstant {
    fn to_content_b(&self) -> ContentB{
        return match self {
            RegisterOrConstant::Register(r) => r.to_content_b(),
            RegisterOrConstant::Constant(c) => *c,
        }
    }
}

impl TryFrom<&str> for RegisterOrConstant {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_chars: Vec<char> = value.chars().collect();
        return match value_chars.len() {
            0 => return Err("No Content".to_string()),
            2 => match RegisterAddress::try_from(value) {
                Ok(address) => Ok(Register(address)),
                Err(err) => Err(err),
            },
            4 => match ContentB::try_from(value) {
                Ok(content) => Ok(Constant(content)),
                Err(err) => Err(err),
            },
            _ => return Err("Invalid Content".to_string())
        }
    }
}

enum ALUCommand {
    Zero(RegisterAddress, RegisterOrConstant),
    PassA(RegisterAddress, RegisterOrConstant),
    PassB(RegisterAddress, RegisterOrConstant),
    PassBSetC(RegisterAddress, RegisterOrConstant),
    PassBHoldC(RegisterAddress, RegisterOrConstant),
    PassBInvertC(RegisterAddress, RegisterOrConstant),
    Complement(RegisterAddress),
    Nor(RegisterAddress, RegisterOrConstant),
    AddHoldC(RegisterAddress, RegisterOrConstant),
    Add(RegisterAddress, RegisterOrConstant),
    AddSub(RegisterAddress, RegisterOrConstant),
    AddC(RegisterAddress, RegisterOrConstant),
    AddSubC(RegisterAddress, RegisterOrConstant),
    ArithShiftRight(RegisterAddress,RegisterOrConstant),
    LogicShiftLeftHoldC(RegisterAddress),
    LogicShiftLeft(RegisterAddress),
    LogicShiftRight(RegisterAddress, RegisterOrConstant),
    ShiftLeftAppend1(RegisterAddress),
    RotateRight(RegisterAddress, RegisterOrConstant),
    RotateRightCarry(RegisterAddress, RegisterOrConstant),
    RotateLeftCarry(RegisterAddress)
}

struct ContentAB {
    content_a: RegisterAddress,
    content_b: ContentB
}

impl ALUCommand {
    fn to_alu_function(&self) -> ALUFunction {
        return match self {
            ALUCommand::Zero(_, _) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: true,
                alu0: true
            }},
            ALUCommand::PassA(_, _) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: false,
                alu0: true
            }},
            ALUCommand::PassB(_, _) => {ALUFunction{
                alu3: true,
                alu2: true,
                alu1: false,
                alu0: false
            }},
            ALUCommand::PassBSetC(_, _) => {ALUFunction{
                alu3: true,
                alu2: true,
                alu1: false,
                alu0: true
            }},
            ALUCommand::PassBHoldC(_, _) => {ALUFunction{
                alu3: true,
                alu2: true,
                alu1: true,
                alu0: false
            }},
            ALUCommand::PassBInvertC(_, _) => {ALUFunction{
                alu3: true,
                alu2: true,
                alu1: true,
                alu0: true
            }},
            ALUCommand::Complement(_) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: true,
                alu0: false
            }},
            ALUCommand::Nor(_, _) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: true,
                alu0: false
            }},
            ALUCommand::AddHoldC(_, _) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: false,
                alu0: false
            }},
            ALUCommand::Add(_, _) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: false,
                alu0: false
            }},
            ALUCommand::AddSub(_, _) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: false,
                alu0: true
            }},
            ALUCommand::AddC(_, _) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: true,
                alu0: false
            }},
            ALUCommand::AddSubC(_, _) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: true,
                alu0: true
            }},
            ALUCommand::ArithShiftRight(_, _) => {ALUFunction{
                alu3: true,
                alu2: false,
                alu1: true,
                alu0: true
            }},
            ALUCommand::LogicShiftLeftHoldC(_) => {ALUFunction{
                alu3: false,
                alu2: false,
                alu1: false,
                alu0: false
            }},
            ALUCommand::LogicShiftLeft(_) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: false,
                alu0: false
            }},
            ALUCommand::LogicShiftRight(_, _) => {ALUFunction{
                alu3: true,
                alu2: false,
                alu1: false,
                alu0: false
            }},
            ALUCommand::ShiftLeftAppend1(_) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: false,
                alu0: true
            }},
            ALUCommand::RotateRight(_, _) => {ALUFunction{
                alu3: true,
                alu2: false,
                alu1: false,
                alu0: true
            }},
            ALUCommand::RotateRightCarry(_, _) => {ALUFunction{
                alu3: true,
                alu2: false,
                alu1: true,
                alu0: false
            }},
            ALUCommand::RotateLeftCarry(_) => {ALUFunction{
                alu3: false,
                alu2: true,
                alu1: true,
                alu0: false
            }},
        }
    }

    fn to_content(&self) -> ContentAB {
        return match self {
            ALUCommand::Zero(a, b) |
            ALUCommand::PassA(a, b) |
            ALUCommand::PassB(a, b) |
            ALUCommand::PassBSetC(a, b) |
            ALUCommand::PassBHoldC(a, b) |
            ALUCommand::PassBInvertC(a, b) |
            ALUCommand::Nor(a, b) |
            ALUCommand::AddHoldC(a, b) |
            ALUCommand::Add(a, b) |
            ALUCommand::AddSub(a, b) |
            ALUCommand::AddC(a, b) |
            ALUCommand::AddSubC(a, b) |
            ALUCommand::LogicShiftRight(a, b) |
            ALUCommand::RotateRight(a, b) |
            ALUCommand::ArithShiftRight(a,b) |
            ALUCommand::RotateRightCarry(a, b) => ContentAB {
                content_a: *a,
                content_b: b.to_content_b()
            },
            ALUCommand::LogicShiftLeftHoldC(r) |
            ALUCommand::LogicShiftLeft(r) |
            ALUCommand::ShiftLeftAppend1(r) |
            ALUCommand::Complement(r) |
            ALUCommand::RotateLeftCarry(r) => ContentAB{
                content_a: *r,
                content_b: r.to_content_b() },
        }
    }

    fn to_register_address_control(&self) -> RegisterAddressControl {
        return match self {
            ALUCommand::Zero(_, b) |
            ALUCommand::PassA(_, b) |
            ALUCommand::PassB(_, b) |
            ALUCommand::PassBSetC(_, b) |
            ALUCommand::PassBHoldC(_, b) |
            ALUCommand::PassBInvertC(_, b) |
            ALUCommand::Nor(_, b) |
            ALUCommand::AddHoldC(_, b) |
            ALUCommand::Add(_, b) |
            ALUCommand::AddSub(_, b) |
            ALUCommand::AddC(_, b) |
            ALUCommand::AddSubC(_, b) |
            ALUCommand::LogicShiftRight(_, b) |
            ALUCommand::RotateRight(_, b) |
            ALUCommand::ArithShiftRight(_, b) |
            ALUCommand::RotateRightCarry(_, b) => return match b {
                RegisterOrConstant::Register(_) => RegisterAddressControl {
                    aca: false,
                    acb: false
                },
                RegisterOrConstant::Constant(_) => RegisterAddressControl {
                    aca: false,
                    acb: true
                },
            },
            ALUCommand::LogicShiftLeftHoldC(r) |
            ALUCommand::LogicShiftLeft(r) |
            ALUCommand::ShiftLeftAppend1(r) |
            ALUCommand::Complement(r) |
            ALUCommand::RotateLeftCarry(r) => RegisterAddressControl {
                aca: false,
                acb: false
            }
        }
    }
}

fn parse_alu_command(command_string: &str) -> Result<ALUCommand, String>{
    let command_string = command_string.trim();
    let split_command_string: Vec<&str> = command_string.split(" ").collect();

    return match split_command_string.len() {
        0 => return Err("Nothing after Split".to_string()),
        1 => return Err("No Register Contents".to_string()),
        2 => parse_single_alu_command(split_command_string[0], split_command_string[1]),
        3 => parse_double_alu_command(split_command_string[0], split_command_string[1], split_command_string[2]),
        _ => return Err("Too much after Split".to_string()),
    }
}

fn parse_single_alu_command(command: &str, content: &str) -> Result<ALUCommand, String> {
    let content = match RegisterAddress::try_from(content) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };

    return match command {
        "LSLH" => Ok(ALUCommand::LogicShiftLeftHoldC(content)),
        "LSL" => Ok(ALUCommand::LogicShiftLeft(content)),
        "SL1" => Ok(ALUCommand::ShiftLeftAppend1(content)),
        "COM" => Ok(ALUCommand::Complement(content)),
        "RLC" => Ok(ALUCommand::RotateLeftCarry(content)),
        _ => Err("Got only one Register Content, but no Command fits".to_string())
    }
}

fn parse_double_alu_command(command: &str, content_a: &str, content_b: &str) -> Result<ALUCommand, String> {
    let content_a = match RegisterAddress::try_from(content_a) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };
    let content_b = match RegisterOrConstant::try_from(content_b) {
        Ok(content) => content,
        Err(err) => return Err(err),
    };

    return match command {
        "ZERO" => Ok(ALUCommand::Zero(content_a,content_b)),
        "PASSA" => Ok(ALUCommand::PassA(content_a,content_b)),
        "PASSB" => Ok(ALUCommand::PassB(content_a,content_b)),
        "BSETC" => Ok(ALUCommand::PassBSetC(content_a,content_b)),
        "BHOLDC" => Ok(ALUCommand::PassBHoldC(content_a,content_b)),
        "BINVC" => Ok(ALUCommand::PassBInvertC(content_a,content_b)),
        "NOR" => Ok(ALUCommand::Nor(content_a,content_b)),
        "ADDH" => Ok(ALUCommand::AddHoldC(content_a,content_b)),
        "ADD" => Ok(ALUCommand::Add(content_a,content_b)),
        "ADDC" => Ok(ALUCommand::ADDC(content_a,content_b)),
        "ADDS" => Ok(ALUCommand::AddSub(content_a,content_b)),
        "ADDSC" => Ok(ALUCommand::AddSubC(content_a,content_b)),
        "ASR" => Ok(ALUCommand::ArithShiftRight(content_a,content_b)),
        "LSR" => Ok(ALUCommand::LogicShiftRight(content_a,content_b)),
        "RR" => Ok(ALUCommand::RotateRight(content_a,content_b)),
        "RRC" => Ok(ALUCommand::RotateRightCarry(content_a,content_b)),
        _ => Err("Got two Register Contents, but no Command fits".to_string())
    }
}

enum WriteCommand {
    WriteA,
    WriteB,
    WriteOff
}

impl WriteCommand {
    fn to_register_control(&self) -> RegisterControl {
        return match self {
            WriteA => RegisterControl{ rws: false, rwe: true },
            WriteB => RegisterControl{ rws: true, rwe: true },
            WriteOff => RegisterControl{ rws: false, rwe: false }
        }
    }
}

fn parse_write_command(command_string: &str) -> Result<WriteCommand, String> {
    return match command_string.trim() {
        "WRITE A" => Ok(WriteA),
        "WRITE B" => Ok(WriteB),
        "WRITE OFF" => Ok(WriteOff),
        _ => Err("Unknown Write Command".to_string())
    }
}

enum BusCommand {
    BusRead,
    BusWrite,
    BusOff
}

impl BusCommand {
    fn to_bus_control(&self) -> BusControl {
        return match self {
            BusRead => BusControl{ bus_wr: false, bus_en: true },
            BusWrite => BusControl{ bus_wr: true, bus_en: true },
            BusOff => BusControl{ bus_wr: false, bus_en: false }
        }
    }
}

fn parse_bus_command(command_string: &str) -> Result<BusCommand, String> {
    return match command_string.trim() {
        "BUS READ" => Ok(BusRead),
        "BUS WRITE" => Ok(BusWrite),
        "BUS OFF" => Ok(BusOff),
        _ => Err("Unknown Bus Command".to_string())
    }
}

enum FlagCommand {
    UpdateFlags,
    KeepFlags,
}

impl FlagCommand {
    fn to_alu_control(&self) -> ALUControl {
        return match self {
            UpdateFlags => ALUControl{ cf: true },
            KeepFlags => ALUControl{ cf: false },
        }
    }
}

fn parse_flag_command(command_string: &str) -> Result<FlagCommand, String> {
    return match command_string.trim() {
        "FLAGS COPY" => Ok(UpdateFlags),
        "FLAGS KEEP" => Ok(KeepFlags),
        _ => Err("Unknown Flag Command".to_string())
    }
}

enum JumpType {
    Jump,
    TestIntAJump,
    TestIntBJump,
    TestCFJump,
    TestCOJump,
    TestZOJump,
    TestNOJump
}

struct JumpCommand {
    jump_type: JumpType,
    next_address: InstructionAddress
}

impl JumpType {
    fn to_address_control(&self) -> AddressControl {
        return match self {
            JumpType::Jump => AddressControl{ ac1: false, ac0: false },
            JumpType::TestIntAJump => AddressControl{ ac1: false, ac0: true },
            JumpType::TestIntBJump => AddressControl{ ac1: true, ac0: true },
            JumpType::TestCFJump => AddressControl{ ac1: false, ac0: true },
            JumpType::TestCOJump => AddressControl{ ac1: true, ac0: false },
            JumpType::TestZOJump => AddressControl{ ac1: true, ac0: false },
            JumpType::TestNOJump => AddressControl{ ac1: true, ac0: true },
        }
    }
}

fn parse_jump_command(command_string: &str) -> Result<JumpCommand, String> {
    let command_string = command_string.trim();
    let split_command_string: Vec<&str> = command_string.split( " ").collect();
    match split_command_string.len() {
        0 => return Err("Nothing after Split".to_string()),
        1 => return Err("No Jump Address".to_string()),
        2 => {},
        _ => return Err("Too much after Split".to_string())
    }

    let jump_command = split_command_string[0];
    let mut jump_address = split_command_string[1];

    let jump_type = match jump_command {
        "JUMP" => JumpType::Jump,
        "JUMPINTA" => JumpType::TestIntAJump,
        "JUMPINTB" => JumpType::TestIntBJump,
        "JUMPCF" => JumpType::TestCFJump,
        "JUMPCO" => JumpType::TestCOJump,
        "JUMPZO" => JumpType::TestZOJump,
        "JUMPNO" => JumpType::TestNOJump,
        _ => return Err("Unknown Jump Type".to_string())
    };

    let mut next_address = match InstructionAddress::try_from(jump_address.to_string()) {
        Ok(addr) => addr,
        Err(err) => return Err(format!("Could not parse Next Address: {}",err.description)),
    };

    match jump_type {
        JumpType::TestIntAJump |
        JumpType::TestCOJump |
        JumpType::TestNOJump => {
            next_address.bit0 = false;
        },
        JumpType::TestIntBJump |
        JumpType::TestCFJump |
        JumpType::TestZOJump => {
            next_address.bit0 = true;
        },
        _ => {}
    }

    return Ok(JumpCommand{ jump_type, next_address })
}