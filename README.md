## Command-Line Syntax: ##

Assembler2i.exe INPUTFILEPATH [-o OUTPUTFILEPATH] [-c] [-i]

INPUTFILEPATH is the File that should be translated, must have .2ia File Type  
OUTPUTFILEPATH is the File Name, that should be output to, must have .2i File Type  
-c specifies, that Comments and Empty Lines should be Copied to the Output File (Currently not working)  
-i specifies, that Instruction should be Copied to the Output File as Comments (Currently not working)  

## Instruction Syntax: ##

A Code Line looks like this:

`INSTRUCTION_ADDRESS: ALU_FUNCTION; WRITECMD; BUSCMD; FLAGCMD; JUMPCMD`

## INSTRUCTION_ADDRESS Syntax: ##

`xxxxx` Where x is either 0 or 1

## ALU_FUNCTION Syntax: ##

### Single Input Commands: 
(These Commands operate with A=B, so only one Register Address is used)  
(R is a Register Address (R0..R7))  

| Command |   ALU Output   | Description                                                                                                      |  
|---------|----------------|------------------------------------------------------------------------------------------------------------------|  
| LSLH R  | F = R << 1     | Left Shifts the Content of R by 1, Carry Out is (Cin OR C), C is the Out-Shifted Bit                             |  
| LSL R   | F = R << 1     | Left Shifts the Content of R by 1, Carry Out is the Out-Shifted Bit                                              |  
| SL1 R   | F = (R << 1)+1 | Left Shifts the Content of R by 1 and In-Shifts 1, Carry Out is the negated Out-Shifted Bit (this may be wrong)  |  
| COM R   | F = NEG R      | Negates the Content of R, Carry Out is 0                                                                         |  
| RLC R   | F = (R<<1)+Cin | Left shifts the Content of R by 1 and In-Shifts Cin, Carry Out is the Out-Shifted Bit                            |  

### Double Input Commands:
(These Commands do not operate with A=B, so both Register Addresses can be used)
(A is a Register Address (R0..R7))
(B is a Register Address (R0..R7) or a Constant (0000..1111))
(If B is a Constant, then the ALU B Input is set to Constant)

|  Command   | ALU Output                 | Description                                                                                            |  
|------------|----------------------------|--------------------------------------------------------------------------------------------------------|  
| ZERO A B   | F = 0                      | Output is Always 0                                                                                     |  
| PASSA A B  | F = A                      | Output is A, Carry Out is 0                                                                            |  
| PASSB A B  | F = B                      | Output is B, Carry Out is 0                                                                            |  
| BSETC A B  | F = B                      | Output is B, Carry Out is 1                                                                            |  
| BHOLDC A B |  F = B                     | Output is B, Carry Out is Carry In                                                                     |  
| BINVC A B  | F = B                      | Output is B, Carry Out is NEG Carry In                                                                 |  
| NOR A B    | F = A NOR B                | Output is A NOR B, Carry Out is 0                                                                      |  
| ADDH A B   | F = A + B                  | Output is A + B, Carry Out is Cin OR C, C is Overflow Carry                                            |  
| ADD A B    | F = A + B                  | Output is A + B, Carry Out is Overflow Carry                                                           |  
| ADDC A B   | F = A + B + Cin            | Output is A + B + Cin, Carry Out is Overflow Carry                                                     |  
| ADDS A B   | F = A + B + 1              | Output is A + B + 1 (For Subtraction), Carry Out is NEG Overflow Carry                                 |  
| ADDSC A B  | F = A + B + NEG Cin        | Output is A + B + 1 + Cin (For Subtraction), Carry Out is NEG Overflow Carry                           |  
| ASR A B    | F(n) = A(n+1), F(7) = A(7) | Output is the Arithmetic Shift Right of A by 1, In-Shift is the last bit of A                          |  
| LSR A B    | F = A >> 1                 | Output is the Logic Right Shift of A by 1, In-Shift is 0, Carry Out is the Out-Shifted Bit             |  
| RR A B     | F = A >> 1                 | Output is the Right Shift of A by 1, In-Shift is the Out-Shifted Bit, Carry Out is the Out-Shifted Bit |  
| RRC A B    | F = (A >> 1), F(7) = Cin   | Output is the Right Shift of A by 1, In-Shift is Cin, Carry-Out is the Out-Shifted Bit                 |  

## WRITECMD Syntax: ##

| Command   | Description                             |
|-----------|-----------------------------------------|
| WRITE A   | Writes ALU Output to Register Address A |
| WRITE B   | Writes ALU Output to Register Address B |
| WRITE OFF | Don't write back ALU Output to Register |

## BUSCMD Syntax: ##

| Command   | Description                                                |  
|-----------|------------------------------------------------------------|  
| BUS READ  | Read from Bus Input (Also forces ALU A Input to Bus Input) |  
| BUS WRITE | Write ALU Output to Bus                                    |  
| BUS OFF   | Don't use Bus                                              |  

## FLAGCMD Syntax: ##

| Command    | Description                                       |
|------------|---------------------------------------------------|
| FLAGS KEEP | Do not update Flags Register with ALU Flag Output |
| FLAGS COPY | Update Flags Register with ALU Flag Output        |



## JUMPCMD Syntax: ##
(NA is a INSTRUCTION_ADDRESS)

| Command     | Description                                                                            |
|-------------|----------------------------------------------------------------------------------------|
| JUMP NA     | Jumps to Next Address                                                                  |
| JUMPINTA NA | Jumps to xxxx1 if INTA is true, xxxx0 if not                                           |
| JUMPINTB NA | Jumps to xxxx1 if INTB is true, xxxx0 if not                                           |
| JUMPCF NA   | Jumps to xxxx1 if the Carry Flag is true, xxxx0 if not (Carry Flag from Flag Register) |
| JUMPCO NA   | Jumps to xxxx1 if the ALU Carry Out is true, xxxx0 if not                              |
| JUMPZO NA   | Jumps to xxxx1 if the ALU Zero Out is true, xxxx0 if not                               |
| JUMPNO NA   | Jumps to xxxx1 if the ALU Negative Out is true, xxxx0 if not                           |
