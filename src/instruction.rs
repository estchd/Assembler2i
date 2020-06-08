use std::fmt;
use std::fmt::{Formatter, Error};
use std::convert::TryFrom;

#[cfg(test)]
mod conversion_tests {
    #[test]
    fn test_conversion_true() {
        assert_eq!("1".to_string(), format!("{}",true as i32))
    }
    #[test]
    fn test_conversion_false() {
        assert_eq!("0".to_string(), format!("{}",false as i32))
    }
}

fn try_bool_from_char(ch: char) -> Result<bool,String> {
    return match ch {
        '0' => Ok(false),
        '1' => Ok(true),
        _ => Err(format!("Expected 0 or 1, got {}",ch))
    }
}

pub struct ParseError {
    pub symbol: usize,
    pub description: String
}

pub struct InstructionAddress {
    pub bit4: bool,
    pub bit3: bool,
    pub bit2: bool,
    pub bit1: bool,
    pub bit0: bool
}

impl fmt::Display for InstructionAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}{}{}{}",
               self.bit4 as i32,
               self.bit3 as i32,
               self.bit2 as i32,
               self.bit1 as i32,
               self.bit0 as i32
        )
    }
}

impl TryFrom<String> for InstructionAddress {
    type Error = ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let char_array: Vec<char> = value.chars().collect();
        match char_array.len() {
            0..=4 => return Err(ParseError{symbol:char_array.len()-1, description: "Unexpected Symbol : while Parsing Instruction Address".to_string()}),
            5 => {},
            _ => return Err(ParseError{symbol: 5, description: "Unexpected Symbol after Parsing Instruction Address".to_string()})
        }

        let bit0 = match try_bool_from_char(char_array[4]){
            Ok(bit) => bit,
            Err(err) => return Err(ParseError{symbol: 0, description: err})
        };
        let bit1 = match try_bool_from_char(char_array[3]){
            Ok(bit) => bit,
            Err(err) => return Err(ParseError{symbol: 1, description: err})
        };
        let bit2 = match try_bool_from_char(char_array[2]){
            Ok(bit) => bit,
            Err(err) => return Err(ParseError{symbol: 2, description: err})
        };
        let bit3 = match try_bool_from_char(char_array[1]){
            Ok(bit) => bit,
            Err(err) => return Err(ParseError{symbol: 3, description: err})
        };
        let bit4 = match try_bool_from_char(char_array[0])
        {
            Ok(bit) => bit,
            Err(err) => return Err(ParseError{symbol: 4, description: err})
        };
        return Ok(InstructionAddress{
            bit4,
            bit3,
            bit2,
            bit1,
            bit0
        });
    }
}

pub struct AddressControl {
    pub ac1: bool,
    pub ac0: bool
}

impl fmt::Display for AddressControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}",
               self.ac1 as i32,
               self.ac0 as i32
        )
    }
}

pub struct BusControl {
    pub bus_wr: bool,
    pub bus_en: bool
}

impl fmt::Display for BusControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}",
               self.bus_wr as i32,
               self.bus_en as i32
        )
    }
}

#[derive(Copy, Clone)]
pub struct RegisterAddress {
    pub ad2: bool,
    pub ad1: bool,
    pub ad0: bool
}

impl RegisterAddress {
    pub fn to_content_b(&self) -> ContentB{
        return ContentB{
            b3: false,
            b2: self.ad2,
            b1: self.ad1,
            b0: self.ad0
        }
    }
}

impl TryFrom<&str> for RegisterAddress {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value_chars: Vec<char> = value.chars().collect();
        match value_chars.len() {
            0 => return Err("No Register Number".to_string()),
            2 => {},
            _ => return Err("Invalid Register Number".to_string())
        }

        if value_chars[0] != 'R' {
            return Err("Invalid Register Number".to_string())
        }

        return match value_chars[1] {
            '0' => Ok(RegisterAddress {
                ad2: false,
                ad1: false,
                ad0: false
            }),
            '1' => Ok(RegisterAddress {
                ad2: false,
                ad1: false,
                ad0: true
            }),
            '2' => Ok(RegisterAddress {
                ad2: false,
                ad1: true,
                ad0: false
            }),
            '3' => Ok(RegisterAddress {
                ad2: false,
                ad1: true,
                ad0: true
            }),
            '4' => Ok(RegisterAddress {
                ad2: true,
                ad1: false,
                ad0: false
            }),
            '5' => Ok(RegisterAddress {
                ad2: true,
                ad1: false,
                ad0: true
            }),
            '6' => Ok(RegisterAddress {
                ad2: true,
                ad1: true,
                ad0: false
            }),
            '7' => Ok(RegisterAddress {
                ad2: true,
                ad1: true,
                ad0: true
            }),
            _ => Err("Invalid Register Number".to_string())
        }
    }
}

impl fmt::Display for RegisterAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}",
               self.ad2 as i32,
               self.ad1 as i32,
               self.ad0 as i32
        )
    }
}

#[derive(Copy, Clone)]
pub struct ContentB {
    pub b3: bool,
    pub b2: bool,
    pub b1: bool,
    pub b0: bool
}

impl TryFrom<&str> for ContentB {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let char_array: Vec<char> = value.chars().collect();
        match char_array.len() {
            0..=3 => return Err("Could not Parse Constant".to_string()),
            4 => {},
            _ => return Err("Could not Parse Constant".to_string())
        }

        let b0 = match try_bool_from_char(char_array[3]){
            Ok(bit) => bit,
            Err(err) => return Err(err)
        };
        let b1 = match try_bool_from_char(char_array[2]){
            Ok(bit) => bit,
            Err(err) => return Err(err)
        };
        let b2 = match try_bool_from_char(char_array[1]){
            Ok(bit) => bit,
            Err(err) => return Err(err)
        };
        let b3 = match try_bool_from_char(char_array[0]){
            Ok(bit) => bit,
            Err(err) => return Err(err)
        };
        return Ok(ContentB {
            b3,
            b2,
            b1,
            b0
        });
    }
}


impl fmt::Display for ContentB {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}{}{}",
               self.b3 as i32,
               self.b2 as i32,
               self.b1 as i32,
               self.b0 as i32
        )
    }
}

pub struct RegisterControl {
    pub rws: bool,
    pub rwe: bool
}

impl fmt::Display for RegisterControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}",
               self.rws as i32,
               self.rwe as i32
        )
    }
}

pub struct RegisterAddressControl {
    pub aca: bool,
    pub acb: bool
}

impl fmt::Display for RegisterAddressControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}",
               self.aca as i32,
               self.acb as i32
        )
    }
}

pub struct ALUFunction {
    pub alu3: bool,
    pub alu2: bool,
    pub alu1: bool,
    pub alu0: bool
}

impl fmt::Display for ALUFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}{}{}{}",
               self.alu3 as i32,
               self.alu2 as i32,
               self.alu1 as i32,
               self.alu0 as i32
        )
    }
}

pub struct ALUControl {
    pub cf: bool
}

impl fmt::Display for ALUControl {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cf as i32)
    }
}

pub struct Instruction {
    pub address: InstructionAddress,
    pub address_control: AddressControl,
    pub next_address: InstructionAddress,
    pub bus_control: BusControl,
    pub content_a: RegisterAddress,
    pub content_b: ContentB,
    pub register_control: RegisterControl,
    pub register_address_control: RegisterAddressControl,
    pub alu_function: ALUFunction,
    pub alu_control: ALUControl
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {} {} {} {} {} {} {} {} {}",
               self.address,
               self.address_control,
               self.next_address,
               self.bus_control,
               self.content_a,
               self.content_b,
               self.register_control,
               self.register_address_control,
               self.alu_function,
               self.alu_control
        )
    }
}