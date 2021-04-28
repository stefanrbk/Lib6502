use super::Decoder as Decoder;

pub const NOTSTACKCHECK: u32 = 0b10010111000010000000000;

pub const PD1XX000X0: u32 = 0b00011101100000000000000;
pub const PD0XX0XX0X: u32 = 0b10010010000000000000000;
pub const PDXXXX10X0: u32 = 0b00000101000010000000000;
pub const PDXXX010X1: u32 = 0b00010100000010010000000;

pub fn check_opcode(opcode: u8, tstate: u8, check: u32) -> bool {
    let op = !opcode as u32;
    let op = op << 8;
    let op = op | (opcode as u32);
    let op = op << 4;
    let op = op
        | match tstate {
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 8,
            _ => 0,
        };
    let op = op << 2;
    let op = op
        | match tstate {
            0 => 2,
            1 => 1,
            _ => 0,
        };
    let op = op << 1;
    let not_stack = (op | NOTSTACKCHECK) != NOTSTACKCHECK;
    let op = op
        | match not_stack {
            true => 1,
            false => 0,
        };
    return (op | check) == check;
}
pub type PlaFn = fn(&mut Decoder);

const PLA: [(u32,PlaFn); 130] = [
    (0b01100011100001000000000, |d:&mut Decoder| d.set_sty_cpy_mem(true)),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
    (0,|d:&mut Decoder| {}),
];

fn decode_opcode(op: u32) -> u8 {
    match op {
        //STY
        0b01100011100001000000000 => 0,

        //ind, Y
        0b00001100000100010010000 => 0,

        //abs, Y
        0b00000100000110010001000 => 0,

        //INY/DEY
        0b00110111100010000000010 => 0,

        //TYA
        0b01100111100110000000010 => 0,

        //CPY/INY
        0b00110011110000000000010 => 0,

        //X/Y indexed
        0b00000000000101000001000 => 0,

        //X&Y reg ops
        0b01000000100000100000000 => 0,

        //X, ind
        0b00011100000000010001000 => 0,

        //TXA
        0b01110100100010100000010 => 0,

        //DEX
        0b00110100110010100000010 => 0,

        //CPX/INX
        0b00010011111000000000010 => 0,

        //from X
        0b01100000100000100000000 => 0,

        //TXS
        0b01100000100110100000010 => 0,

        //LDX/TAX/TSX
        0b01000000101000100000010 => 0,

        //DEX
        0b00110100110010100000100 => 0,

        //INX
        0b00010111111010000000100 => 0,

        //TSX
        0b01000100101110100000010 => 0,

        //INY/DEY
        0b00110111100010000000100 => 0,

        //LDY mem
        0b01000011101001000000010 => 0,

        //TAY/LDY not X indexed
        0b01010011101000000000010 => 0,

        //JSR
        0b11011111001000000000010 => 0,

        //BRK
        0b11111111000000001000000 => 0,

        //PHP/PHA
        0b10110111000010000000010 => 0,

        //RTS
        0b10011111011000000100000 => 0,

        //PLP/PLA
        0b10010111001010000010000 => 0,

        //RTI
        0b10111111010000001000000 => 0,

        //ROR
        0b10000000011000100000000 => 0,

        //T2
        0b00000000000000000001000 => 0,

        //EOR
        0b10100000010000010000010 => 0,

        //JMP
        0b10010011010011000000000 => 0,

        //abs
        0b00010000000011000001000 => 0,

        //ORA
        0b11100000000000010000010 => 0,

        //ADL/ADD
        0b00001000000000000001000 => 0,

        //T0
        0b00000000000000000000010 => 0,

        //Stack
        0b10010111000000000001000 => 0,

        //Stack/BIT/JMP
        0b10010011000000000010000 => 0,

        //BRK/JSR
        0b11011111000000000100000 => 0,

        //RTI
        0b10111111010000000100000 => 0,

        //X, ind
        0b00011100000000010010000 => 0,

        //ind, Y
        0b00001100000100010100000 => 0,

        //ind, Y
        0b00001100000100010001000 => 0,

        //abs idx
        0b00000000000110000010000 => 0,

        //PLP/PLA
        0b10010111001010000000000 => 0,

        //INC/NOP
        0b00000000111000100000000 => 0,

        //X, ind
        0b00011100000000010100000 => 0,

        //ind, Y
        0b00001100000100010010000 => 0,

        //RTI/RTS
        0b10011111010000000000000 => 0,

        //JSR
        0b11011111001000000001000 => 0,

        //CPX/CPY/INX/INY
        0b00010011110000000000010 => 0,

        //CMP
        0b00100000110000010000010 => 0,

        //SBC
        0b00000000111000010000010 => 0,

        //ADC/SBC
        0b00000000011000010000010 => 0,

        //ROL
        0b11000000001000100000000 => 0,

        //JMP
        0b10010011010011000010000 => 0,

        //ROL/ASL
        0b11000000000000100000000 => 0,

        //JSR
        0b11011111001000001000000 => 0,

        //Stack access
        0b10010111000000000001000 => 0,

        //TYA
        0b01100111100110000000010 => 0,

        //ORA/AND/EOR/ADC
        0b10000000000000010000100 => 0,

        //ADC/SBC
        0b00000000011000010000100 => 0,

        //Shift A
        0b10010100000010100000100 => 0,

        //TXA
        0b01110100100010100000010 => 0,

        //PLA
        0b10010111011010000000010 => 0,

        //LDA
        0b01000000101000010000010 => 0,

        //A reg
        0b00000000000000010000010 => 0,

        //TAY
        0b01010111101010000000010 => 0,

        //Shift A
        0b10010100000010100000010 => 0,

        //TAX
        0b01010100101010100000010 => 0,

        //BIT
        0b11010011001001000000010 => 0,

        //AND
        0b11000000001000010000010 => 0,

        //abs idx
        0b00000000000110000100000 => 0,

        //ind, Y
        0b00001100000100011000000 => 0,

        //Branch done
        0b00001111000100000000010 => 0,

        //PHA
        0b10110111010010000001000 => 0,

        //Shift right A
        0b10010100010010100000010 => 0,

        //Shift right
        0b10000000010000100000000 => 0,

        //BRK
        0b11111111000000000001000 => 0,

        //JSR
        0b11011111001000000010000 => 0,

        //STA/CMP
        0b01100000100000010000000 => 0,

        //Branch
        0b00001111000100000001000 => 0,

        //zp/zp, X/Y
        0b00001000000001000001000 => 0,

        //X/Y indirect
        0b00001100000000010001000 => 0,

        //abs access
        0b00000000000010000001001 => 0,

        //RTS
        0b10011111011000001000000 => 0,

        //T4
        0b00000000000000000100000 => 0,

        //T3
        0b00000000000000000010000 => 0,

        //BRK/RTI
        0b10111111000000000000010 => 0,

        //JMP
        0b10010011010011000000010 => 0,

        //X, ind
        0b00011100000000011000000 => 0,

        //abs/idx/ind
        0b00000000000010000010000 => 0,

        //ind, Y
        0b00001100000100010100000 => 0,

        //abs idx
        0b00000000000110000010000 => 0,

        //branch
        0b00001111000100000010000 => 0,

        //BRK/RTI
        0b10111111000000000000000 => 0,

        //JSR
        0b11011111001000000000000 => 0,

        //JMP
        0b10010011010011000000000 => 0,

        //store
        0b01100000100000000000000 => 0,

        //BRK
        0b11111111000000000100000 => 0,

        //PHP
        0b11110111000010000001000 => 0,

        //PHP/PHA
        0b10110111000010000001000 => 0,

        //JMP
        0b10010011010011000100000 => 0,

        //RTI/RTS
        0b10011111010000001000000 => 0,

        //JSR
        0b11011111001000001000000 => 0,

        //JMP abs
        0b10110011010011000001000 => 0,

        //PLP/PLA
        0b10010111001010000010000 => 0,

        //LSR/ROR/DEC/INC
        0b00000000010000100000000 => 0,

        //ROL/ASL
        0b11000000000000100000000 => 0,

        //CLI/SEI
        0b10000111010110000000010 => 0,

        //BIT
        0b11010011001001000000100 => 0,

        //CLC/SEC
        0b11000111000110000000010 => 0,

        //zp, X/Y
        0b00001000000101000010000 => 0,

        //ADC/SBC
        0b00000000011000010000100 => 0,

        //BIT
        0b11010011001001000000010 => 0,

        //PLP
        0b11010111001010000000010 => 0,

        //RTI
        0b10111111010000000100000 => 0,

        //CMP
        0b00100000110000010000100 => 0,

        //CPX/CPY abs
        0b00010011110011000000100 => 0,

        //ROL/ASL A
        0b11010100000010100000100 => 0,

        //CPX/CPY imm/zp
        0b00011011110000000000100 => 0,

        //CLD/SED
        0b00000111110110000000010 => 0,

        //bit 6
        0b00000000010000000000000 => 0,

        //abs
        0b00010000000011000010000 => 0,

        //zp
        0b00011000000001000001000 => 0,

        //X, ind/ind, Y
        0b00001100000000011000000 => 0,

        //abs idx
        0b00000000000110000100000 => 0,

        //bit 7
        0b00000000100000000000000 => 0,

        //CLV
        0b01000111101110000000000 => 0,

        //impl
        0b00000101000010000000001 => 0,

        _ => 0,
    }
}
