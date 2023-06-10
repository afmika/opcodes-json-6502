(Opcode::new(Instr::BRK, AdrMode::IMPL, vec!["BRK".to_string()]), 0x00),

(Opcode::new(Instr::ORA, AdrMode::INDX, vec!["ORA ($44,X)".to_string()]), 0x01),
(Opcode::new(Instr::ORA, AdrMode::ZP, vec!["ORA $44".to_string()]), 0x05),
(Opcode::new(Instr::ORA, AdrMode::IMM, vec!["ORA #$44".to_string()]), 0x09),
(Opcode::new(Instr::ORA, AdrMode::ABS, vec!["ORA $4400".to_string()]), 0x0D),
(Opcode::new(Instr::ORA, AdrMode::INDY, vec!["ORA ($44),Y".to_string()]), 0x11),
(Opcode::new(Instr::ORA, AdrMode::ZPX, vec!["ORA $44,X".to_string()]), 0x15),
(Opcode::new(Instr::ORA, AdrMode::ABSY, vec!["ORA $4400,Y".to_string()]), 0x19),
(Opcode::new(Instr::ORA, AdrMode::ABSX, vec!["ORA $4400,X".to_string()]), 0x1D),

(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x02),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x12),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x22),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x32),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x42),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x52),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x62),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x72),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0x92),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0xB2),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0xD2),
(Opcode::new(Instr::STP, AdrMode::IMPL, vec!["STP".to_string()]), 0xF2),

(Opcode::new(Instr::SLO, AdrMode::INDX, vec!["SLO ($44,X)".to_string()]), 0x03),
(Opcode::new(Instr::SLO, AdrMode::ZP, vec!["SLO $44".to_string()]), 0x07),
(Opcode::new(Instr::SLO, AdrMode::ABS, vec!["SLO $4400".to_string()]), 0x0F),
(Opcode::new(Instr::SLO, AdrMode::INDY, vec!["SLO ($44),Y".to_string()]), 0x13),
(Opcode::new(Instr::SLO, AdrMode::ZPX, vec!["SLO $44,X".to_string()]), 0x17),
(Opcode::new(Instr::SLO, AdrMode::ABSY, vec!["SLO $4400,Y".to_string()]), 0x1B),
(Opcode::new(Instr::SLO, AdrMode::ABSX, vec!["SLO $4400,X".to_string()]), 0x1F),

(Opcode::new(Instr::NOP, AdrMode::ZP, vec!["NOP $44".to_string()]), 0x04),
(Opcode::new(Instr::NOP, AdrMode::ABS, vec!["NOP $4400".to_string()]), 0x0C),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0x14),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0x1A),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0x1C),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0x34),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0x3A),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0x3C),
(Opcode::new(Instr::NOP, AdrMode::ZP, vec!["NOP $44".to_string()]), 0x44),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0x54),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0x5A),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0x5C),
(Opcode::new(Instr::NOP, AdrMode::ZP, vec!["NOP $44".to_string()]), 0x64),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0x74),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0x7A),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0x7C),
(Opcode::new(Instr::NOP, AdrMode::IMM, vec!["NOP #$44".to_string()]), 0x80),
(Opcode::new(Instr::NOP, AdrMode::IMM, vec!["NOP #$44".to_string()]), 0x82),
(Opcode::new(Instr::NOP, AdrMode::IMM, vec!["NOP #$44".to_string()]), 0x89),
(Opcode::new(Instr::NOP, AdrMode::IMM, vec!["NOP #$44".to_string()]), 0xC2),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0xD4),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0xDA),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0xDC),
(Opcode::new(Instr::NOP, AdrMode::IMM, vec!["NOP #$44".to_string()]), 0xE2),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0xEA),
(Opcode::new(Instr::NOP, AdrMode::ZPX, vec!["NOP $44,X".to_string()]), 0xF4),
(Opcode::new(Instr::NOP, AdrMode::IMPL, vec!["NOP".to_string()]), 0xFA),
(Opcode::new(Instr::NOP, AdrMode::ABSX, vec!["NOP $4400,X".to_string()]), 0xFC),

(Opcode::new(Instr::ASL, AdrMode::ZP, vec!["ASL $44".to_string()]), 0x06),
(Opcode::new(Instr::ASL, AdrMode::IMPL, vec!["ASL".to_string()]), 0x0A),
(Opcode::new(Instr::ASL, AdrMode::ABS, vec!["ASL $4400".to_string()]), 0x0E),
(Opcode::new(Instr::ASL, AdrMode::ZPX, vec!["ASL $44,X".to_string()]), 0x16),
(Opcode::new(Instr::ASL, AdrMode::ABSX, vec!["ASL $4400,X".to_string()]), 0x1E),

(Opcode::new(Instr::PHP, AdrMode::IMPL, vec!["PHP".to_string()]), 0x08),

(Opcode::new(Instr::ANC, AdrMode::IMM, vec!["ANC #$44".to_string()]), 0x0B),
(Opcode::new(Instr::ANC, AdrMode::IMM, vec!["ANC #$44".to_string()]), 0x2B),

(Opcode::new(Instr::BPL, AdrMode::REL, vec!["BPL $10".to_string(), "BPL label".to_string()]), 0x10),

(Opcode::new(Instr::CLC, AdrMode::IMPL, vec!["CLC".to_string()]), 0x18),

(Opcode::new(Instr::JSR, AdrMode::ABS, vec!["JSR $4400".to_string()]), 0x20),

(Opcode::new(Instr::AND, AdrMode::INDX, vec!["AND ($44,X)".to_string()]), 0x21),
(Opcode::new(Instr::AND, AdrMode::ZP, vec!["AND $44".to_string()]), 0x25),
(Opcode::new(Instr::AND, AdrMode::IMM, vec!["AND #$44".to_string()]), 0x29),
(Opcode::new(Instr::AND, AdrMode::ABS, vec!["AND $4400".to_string()]), 0x2D),
(Opcode::new(Instr::AND, AdrMode::INDY, vec!["AND ($44),Y".to_string()]), 0x31),
(Opcode::new(Instr::AND, AdrMode::ZPX, vec!["AND $44,X".to_string()]), 0x35),
(Opcode::new(Instr::AND, AdrMode::ABSY, vec!["AND $4400,Y".to_string()]), 0x39),
(Opcode::new(Instr::AND, AdrMode::ABSX, vec!["AND $4400,X".to_string()]), 0x3D),

(Opcode::new(Instr::RLA, AdrMode::INDX, vec!["RLA ($44,X)".to_string()]), 0x23),
(Opcode::new(Instr::RLA, AdrMode::ZP, vec!["RLA $44".to_string()]), 0x27),
(Opcode::new(Instr::RLA, AdrMode::ABS, vec!["RLA $4400".to_string()]), 0x2F),
(Opcode::new(Instr::RLA, AdrMode::INDY, vec!["RLA ($44),Y".to_string()]), 0x33),
(Opcode::new(Instr::RLA, AdrMode::ZPX, vec!["RLA $44,X".to_string()]), 0x37),
(Opcode::new(Instr::RLA, AdrMode::ABSY, vec!["RLA $4400,Y".to_string()]), 0x3B),
(Opcode::new(Instr::RLA, AdrMode::ABSX, vec!["RLA $4400,X".to_string()]), 0x3F),

(Opcode::new(Instr::BIT, AdrMode::ZP, vec!["BIT $44".to_string()]), 0x24),
(Opcode::new(Instr::BIT, AdrMode::ABS, vec!["BIT $4400".to_string()]), 0x2C),

(Opcode::new(Instr::ROL, AdrMode::ZP, vec!["ROL $44".to_string()]), 0x26),
(Opcode::new(Instr::ROL, AdrMode::IMPL, vec!["ROL".to_string()]), 0x2A),
(Opcode::new(Instr::ROL, AdrMode::ABS, vec!["ROL $4400".to_string()]), 0x2E),
(Opcode::new(Instr::ROL, AdrMode::ZPX, vec!["ROL $44,X".to_string()]), 0x36),
(Opcode::new(Instr::ROL, AdrMode::ABSX, vec!["ROL $4400,X".to_string()]), 0x3E),

(Opcode::new(Instr::PLP, AdrMode::IMPL, vec!["PLP".to_string()]), 0x28),

(Opcode::new(Instr::BMI, AdrMode::REL, vec!["BMI $10".to_string(), "BMI label".to_string()]), 0x30),

(Opcode::new(Instr::SEC, AdrMode::IMPL, vec!["SEC".to_string()]), 0x38),

(Opcode::new(Instr::RTI, AdrMode::IMPL, vec!["RTI".to_string()]), 0x40),

(Opcode::new(Instr::EOR, AdrMode::INDX, vec!["EOR ($44,X)".to_string()]), 0x41),
(Opcode::new(Instr::EOR, AdrMode::ZP, vec!["EOR $44".to_string()]), 0x45),
(Opcode::new(Instr::EOR, AdrMode::IMM, vec!["EOR #$44".to_string()]), 0x49),
(Opcode::new(Instr::EOR, AdrMode::ABS, vec!["EOR $4400".to_string()]), 0x4D),
(Opcode::new(Instr::EOR, AdrMode::INDY, vec!["EOR ($44),Y".to_string()]), 0x51),
(Opcode::new(Instr::EOR, AdrMode::ZPX, vec!["EOR $44,X".to_string()]), 0x55),
(Opcode::new(Instr::EOR, AdrMode::ABSY, vec!["EOR $4400,Y".to_string()]), 0x59),
(Opcode::new(Instr::EOR, AdrMode::ABSX, vec!["EOR $4400,X".to_string()]), 0x5D),

(Opcode::new(Instr::SRE, AdrMode::INDX, vec!["SRE ($44,X)".to_string()]), 0x43),
(Opcode::new(Instr::SRE, AdrMode::ZP, vec!["SRE $44".to_string()]), 0x47),
(Opcode::new(Instr::SRE, AdrMode::ABS, vec!["SRE $4400".to_string()]), 0x4F),
(Opcode::new(Instr::SRE, AdrMode::INDY, vec!["SRE ($44),Y".to_string()]), 0x53),
(Opcode::new(Instr::SRE, AdrMode::ZPX, vec!["SRE $44,X".to_string()]), 0x57),
(Opcode::new(Instr::SRE, AdrMode::ABSY, vec!["SRE $4400,Y".to_string()]), 0x5B),
(Opcode::new(Instr::SRE, AdrMode::ABSX, vec!["SRE $4400,X".to_string()]), 0x5F),

(Opcode::new(Instr::LSR, AdrMode::ZP, vec!["LSR $44".to_string()]), 0x46),
(Opcode::new(Instr::LSR, AdrMode::IMPL, vec!["LSR".to_string()]), 0x4A),
(Opcode::new(Instr::LSR, AdrMode::ABS, vec!["LSR $4400".to_string()]), 0x4E),
(Opcode::new(Instr::LSR, AdrMode::ZPX, vec!["LSR $44,X".to_string()]), 0x56),
(Opcode::new(Instr::LSR, AdrMode::ABSX, vec!["LSR $4400,X".to_string()]), 0x5E),

(Opcode::new(Instr::PHA, AdrMode::IMPL, vec!["PHA".to_string()]), 0x48),

(Opcode::new(Instr::ALR, AdrMode::IMM, vec!["ALR #$44".to_string()]), 0x4B),

(Opcode::new(Instr::JMP, AdrMode::ABS, vec!["JMP $4400".to_string()]), 0x4C),
(Opcode::new(Instr::JMP, AdrMode::IND, vec!["JMP ($5597)".to_string()]), 0x6C),

(Opcode::new(Instr::BVC, AdrMode::REL, vec!["BVC $10".to_string(), "BVC label".to_string()]), 0x50),

(Opcode::new(Instr::CLI, AdrMode::IMPL, vec!["CLI".to_string()]), 0x58),

(Opcode::new(Instr::RTS, AdrMode::IMPL, vec!["RTS".to_string()]), 0x60),

(Opcode::new(Instr::ADC, AdrMode::INDX, vec!["ADC ($44,X)".to_string()]), 0x61),
(Opcode::new(Instr::ADC, AdrMode::ZP, vec!["ADC $44".to_string()]), 0x65),
(Opcode::new(Instr::ADC, AdrMode::IMM, vec!["ADC #$44".to_string()]), 0x69),
(Opcode::new(Instr::ADC, AdrMode::ABS, vec!["ADC $4400".to_string()]), 0x6D),
(Opcode::new(Instr::ADC, AdrMode::INDY, vec!["ADC ($44),Y".to_string()]), 0x71),
(Opcode::new(Instr::ADC, AdrMode::ZPX, vec!["ADC $44,X".to_string()]), 0x75),
(Opcode::new(Instr::ADC, AdrMode::ABSY, vec!["ADC $4400,Y".to_string()]), 0x79),
(Opcode::new(Instr::ADC, AdrMode::ABSX, vec!["ADC $4400,X".to_string()]), 0x7D),

(Opcode::new(Instr::RRA, AdrMode::INDX, vec!["RRA ($44,X)".to_string()]), 0x63),
(Opcode::new(Instr::RRA, AdrMode::ZP, vec!["RRA $44".to_string()]), 0x67),
(Opcode::new(Instr::RRA, AdrMode::ABS, vec!["RRA $4400".to_string()]), 0x6F),
(Opcode::new(Instr::RRA, AdrMode::INDY, vec!["RRA ($44),Y".to_string()]), 0x73),
(Opcode::new(Instr::RRA, AdrMode::ZPX, vec!["RRA $44,X".to_string()]), 0x77),
(Opcode::new(Instr::RRA, AdrMode::ABSY, vec!["RRA $4400,Y".to_string()]), 0x7B),
(Opcode::new(Instr::RRA, AdrMode::ABSX, vec!["RRA $4400,X".to_string()]), 0x7F),

(Opcode::new(Instr::ROR, AdrMode::ZP, vec!["ROR $44".to_string()]), 0x66),
(Opcode::new(Instr::ROR, AdrMode::IMPL, vec!["ROR".to_string()]), 0x6A),
(Opcode::new(Instr::ROR, AdrMode::ABS, vec!["ROR $4400".to_string()]), 0x6E),
(Opcode::new(Instr::ROR, AdrMode::ZPX, vec!["ROR $44,X".to_string()]), 0x76),
(Opcode::new(Instr::ROR, AdrMode::ABSX, vec!["ROR $4400,X".to_string()]), 0x7E),

(Opcode::new(Instr::PLA, AdrMode::IMPL, vec!["PLA".to_string()]), 0x68),

(Opcode::new(Instr::ARR, AdrMode::IMM, vec!["ARR #$44".to_string()]), 0x6B),

(Opcode::new(Instr::BVS, AdrMode::REL, vec!["BVS $10".to_string(), "BVS label".to_string()]), 0x70),

(Opcode::new(Instr::SEI, AdrMode::IMPL, vec!["SEI".to_string()]), 0x78),

(Opcode::new(Instr::STA, AdrMode::INDX, vec!["STA ($44,X)".to_string()]), 0x81),
(Opcode::new(Instr::STA, AdrMode::ZP, vec!["STA $44".to_string()]), 0x85),
(Opcode::new(Instr::STA, AdrMode::ABS, vec!["STA $4400".to_string()]), 0x8D),
(Opcode::new(Instr::STA, AdrMode::INDY, vec!["STA ($44),Y".to_string()]), 0x91),
(Opcode::new(Instr::STA, AdrMode::ZPX, vec!["STA $44,X".to_string()]), 0x95),
(Opcode::new(Instr::STA, AdrMode::ABSY, vec!["STA $4400,Y".to_string()]), 0x99),
(Opcode::new(Instr::STA, AdrMode::ABSX, vec!["STA $4400,X".to_string()]), 0x9D),

(Opcode::new(Instr::SAX, AdrMode::INDX, vec!["SAX ($44,X)".to_string()]), 0x83),
(Opcode::new(Instr::SAX, AdrMode::ZP, vec!["SAX $44".to_string()]), 0x87),
(Opcode::new(Instr::SAX, AdrMode::ABS, vec!["SAX $4400".to_string()]), 0x8F),
(Opcode::new(Instr::SAX, AdrMode::ZPY, vec!["SAX $44,Y".to_string()]), 0x97),

(Opcode::new(Instr::STY, AdrMode::ZP, vec!["STY $44".to_string()]), 0x84),
(Opcode::new(Instr::STY, AdrMode::ABS, vec!["STY $4400".to_string()]), 0x8C),
(Opcode::new(Instr::STY, AdrMode::ZPX, vec!["STY $44,X".to_string()]), 0x94),

(Opcode::new(Instr::STX, AdrMode::ZP, vec!["STX $44".to_string()]), 0x86),
(Opcode::new(Instr::STX, AdrMode::ABS, vec!["STX $4400".to_string()]), 0x8E),
(Opcode::new(Instr::STX, AdrMode::ZPY, vec!["STX $44,Y".to_string()]), 0x96),

(Opcode::new(Instr::DEY, AdrMode::IMPL, vec!["DEY".to_string()]), 0x88),

(Opcode::new(Instr::TXA, AdrMode::IMPL, vec!["TXA".to_string()]), 0x8A),

(Opcode::new(Instr::XAA, AdrMode::IMM, vec!["XAA #$44".to_string()]), 0x8B),

(Opcode::new(Instr::BCC, AdrMode::REL, vec!["BCC $10".to_string(), "BCC label".to_string()]), 0x90),

(Opcode::new(Instr::AHX, AdrMode::INDY, vec!["AHX ($44),Y".to_string()]), 0x93),
(Opcode::new(Instr::AHX, AdrMode::ABSY, vec!["AHX $4400,Y".to_string()]), 0x9F),

(Opcode::new(Instr::TYA, AdrMode::IMPL, vec!["TYA".to_string()]), 0x98),

(Opcode::new(Instr::TXS, AdrMode::IMPL, vec!["TXS".to_string()]), 0x9A),

(Opcode::new(Instr::TAS, AdrMode::ABSY, vec!["TAS $4400,Y".to_string()]), 0x9B),

(Opcode::new(Instr::SHY, AdrMode::ABSX, vec!["SHY $4400,X".to_string()]), 0x9C),

(Opcode::new(Instr::SHX, AdrMode::ABSY, vec!["SHX $4400,Y".to_string()]), 0x9E),

(Opcode::new(Instr::LDY, AdrMode::IMM, vec!["LDY #$44".to_string()]), 0xA0),
(Opcode::new(Instr::LDY, AdrMode::ZP, vec!["LDY $44".to_string()]), 0xA4),
(Opcode::new(Instr::LDY, AdrMode::ABS, vec!["LDY $4400".to_string()]), 0xAC),
(Opcode::new(Instr::LDY, AdrMode::ZPX, vec!["LDY $44,X".to_string()]), 0xB4),
(Opcode::new(Instr::LDY, AdrMode::ABSX, vec!["LDY $4400,X".to_string()]), 0xBC),

(Opcode::new(Instr::LDA, AdrMode::INDX, vec!["LDA ($44,X)".to_string()]), 0xA1),
(Opcode::new(Instr::LDA, AdrMode::ZP, vec!["LDA $44".to_string()]), 0xA5),
(Opcode::new(Instr::LDA, AdrMode::IMM, vec!["LDA #$44".to_string()]), 0xA9),
(Opcode::new(Instr::LDA, AdrMode::ABS, vec!["LDA $4400".to_string()]), 0xAD),
(Opcode::new(Instr::LDA, AdrMode::INDY, vec!["LDA ($44),Y".to_string()]), 0xB1),
(Opcode::new(Instr::LDA, AdrMode::ZPX, vec!["LDA $44,X".to_string()]), 0xB5),
(Opcode::new(Instr::LDA, AdrMode::ABSY, vec!["LDA $4400,Y".to_string()]), 0xB9),
(Opcode::new(Instr::LDA, AdrMode::ABSX, vec!["LDA $4400,X".to_string()]), 0xBD),

(Opcode::new(Instr::LDX, AdrMode::IMM, vec!["LDX #$44".to_string()]), 0xA2),
(Opcode::new(Instr::LDX, AdrMode::ZP, vec!["LDX $44".to_string()]), 0xA6),
(Opcode::new(Instr::LDX, AdrMode::ABS, vec!["LDX $4400".to_string()]), 0xAE),
(Opcode::new(Instr::LDX, AdrMode::ZPY, vec!["LDX $44,Y".to_string()]), 0xB6),
(Opcode::new(Instr::LDX, AdrMode::ABSY, vec!["LDX $4400,Y".to_string()]), 0xBE),

(Opcode::new(Instr::LAX, AdrMode::INDX, vec!["LAX ($44,X)".to_string()]), 0xA3),
(Opcode::new(Instr::LAX, AdrMode::ZP, vec!["LAX $44".to_string()]), 0xA7),
(Opcode::new(Instr::LAX, AdrMode::IMM, vec!["LAX #$44".to_string()]), 0xAB),
(Opcode::new(Instr::LAX, AdrMode::ABS, vec!["LAX $4400".to_string()]), 0xAF),
(Opcode::new(Instr::LAX, AdrMode::INDY, vec!["LAX ($44),Y".to_string()]), 0xB3),
(Opcode::new(Instr::LAX, AdrMode::ZPY, vec!["LAX $44,Y".to_string()]), 0xB7),
(Opcode::new(Instr::LAX, AdrMode::ABSY, vec!["LAX $4400,Y".to_string()]), 0xBF),

(Opcode::new(Instr::TAY, AdrMode::IMPL, vec!["TAY".to_string()]), 0xA8),

(Opcode::new(Instr::TAX, AdrMode::IMPL, vec!["TAX".to_string()]), 0xAA),

(Opcode::new(Instr::BCS, AdrMode::REL, vec!["BCS $10".to_string(), "BCS label".to_string()]), 0xB0),

(Opcode::new(Instr::CLV, AdrMode::IMPL, vec!["CLV".to_string()]), 0xB8),

(Opcode::new(Instr::TSX, AdrMode::IMPL, vec!["TSX".to_string()]), 0xBA),

(Opcode::new(Instr::LAS, AdrMode::ABSY, vec!["LAS $4400,Y".to_string()]), 0xBB),

(Opcode::new(Instr::CPY, AdrMode::IMM, vec!["CPY #$44".to_string()]), 0xC0),
(Opcode::new(Instr::CPY, AdrMode::ZP, vec!["CPY $44".to_string()]), 0xC4),
(Opcode::new(Instr::CPY, AdrMode::ABS, vec!["CPY $4400".to_string()]), 0xCC),

(Opcode::new(Instr::CMP, AdrMode::INDX, vec!["CMP ($44,X)".to_string()]), 0xC1),
(Opcode::new(Instr::CMP, AdrMode::ZP, vec!["CMP $44".to_string()]), 0xC5),
(Opcode::new(Instr::CMP, AdrMode::IMM, vec!["CMP #$44".to_string()]), 0xC9),
(Opcode::new(Instr::CMP, AdrMode::ABS, vec!["CMP $4400".to_string()]), 0xCD),
(Opcode::new(Instr::CMP, AdrMode::INDY, vec!["CMP ($44),Y".to_string()]), 0xD1),
(Opcode::new(Instr::CMP, AdrMode::ZPX, vec!["CMP $44,X".to_string()]), 0xD5),
(Opcode::new(Instr::CMP, AdrMode::ABSY, vec!["CMP $4400,Y".to_string()]), 0xD9),
(Opcode::new(Instr::CMP, AdrMode::ABSX, vec!["CMP $4400,X".to_string()]), 0xDD),

(Opcode::new(Instr::DCP, AdrMode::INDX, vec!["DCP ($44,X)".to_string()]), 0xC3),
(Opcode::new(Instr::DCP, AdrMode::ZP, vec!["DCP $44".to_string()]), 0xC7),
(Opcode::new(Instr::DCP, AdrMode::ABS, vec!["DCP $4400".to_string()]), 0xCF),
(Opcode::new(Instr::DCP, AdrMode::INDY, vec!["DCP ($44),Y".to_string()]), 0xD3),
(Opcode::new(Instr::DCP, AdrMode::ZPX, vec!["DCP $44,X".to_string()]), 0xD7),
(Opcode::new(Instr::DCP, AdrMode::ABSY, vec!["DCP $4400,Y".to_string()]), 0xDB),
(Opcode::new(Instr::DCP, AdrMode::ABSX, vec!["DCP $4400,X".to_string()]), 0xDF),

(Opcode::new(Instr::DEC, AdrMode::ZP, vec!["DEC $44".to_string()]), 0xC6),
(Opcode::new(Instr::DEC, AdrMode::ABS, vec!["DEC $4400".to_string()]), 0xCE),
(Opcode::new(Instr::DEC, AdrMode::ZPX, vec!["DEC $44,X".to_string()]), 0xD6),
(Opcode::new(Instr::DEC, AdrMode::ABSX, vec!["DEC $4400,X".to_string()]), 0xDE),

(Opcode::new(Instr::INY, AdrMode::IMPL, vec!["INY".to_string()]), 0xC8),

(Opcode::new(Instr::DEX, AdrMode::IMPL, vec!["DEX".to_string()]), 0xCA),

(Opcode::new(Instr::AXS, AdrMode::IMM, vec!["AXS #$44".to_string()]), 0xCB),

(Opcode::new(Instr::BNE, AdrMode::REL, vec!["BNE $10".to_string(), "BNE label".to_string()]), 0xD0),

(Opcode::new(Instr::CLD, AdrMode::IMPL, vec!["CLD".to_string()]), 0xD8),

(Opcode::new(Instr::CPX, AdrMode::IMM, vec!["CPX #$44".to_string()]), 0xE0),
(Opcode::new(Instr::CPX, AdrMode::ZP, vec!["CPX $44".to_string()]), 0xE4),
(Opcode::new(Instr::CPX, AdrMode::ABS, vec!["CPX $4400".to_string()]), 0xEC),

(Opcode::new(Instr::SBC, AdrMode::INDX, vec!["SBC ($44,X)".to_string()]), 0xE1),
(Opcode::new(Instr::SBC, AdrMode::ZP, vec!["SBC $44".to_string()]), 0xE5),
(Opcode::new(Instr::SBC, AdrMode::IMM, vec!["SBC #$44".to_string()]), 0xE9),
(Opcode::new(Instr::SBC, AdrMode::IMM, vec!["SBC #$44".to_string()]), 0xEB),
(Opcode::new(Instr::SBC, AdrMode::ABS, vec!["SBC $4400".to_string()]), 0xED),
(Opcode::new(Instr::SBC, AdrMode::INDY, vec!["SBC ($44),Y".to_string()]), 0xF1),
(Opcode::new(Instr::SBC, AdrMode::ZPX, vec!["SBC $44,X".to_string()]), 0xF5),
(Opcode::new(Instr::SBC, AdrMode::ABSY, vec!["SBC $4400,Y".to_string()]), 0xF9),
(Opcode::new(Instr::SBC, AdrMode::ABSX, vec!["SBC $4400,X".to_string()]), 0xFD),

(Opcode::new(Instr::ISC, AdrMode::INDX, vec!["ISC ($44,X)".to_string()]), 0xE3),
(Opcode::new(Instr::ISC, AdrMode::ZP, vec!["ISC $44".to_string()]), 0xE7),
(Opcode::new(Instr::ISC, AdrMode::ABS, vec!["ISC $4400".to_string()]), 0xEF),
(Opcode::new(Instr::ISC, AdrMode::INDY, vec!["ISC ($44),Y".to_string()]), 0xF3),
(Opcode::new(Instr::ISC, AdrMode::ZPX, vec!["ISC $44,X".to_string()]), 0xF7),
(Opcode::new(Instr::ISC, AdrMode::ABSY, vec!["ISC $4400,Y".to_string()]), 0xFB),
(Opcode::new(Instr::ISC, AdrMode::ABSX, vec!["ISC $4400,X".to_string()]), 0xFF),

(Opcode::new(Instr::INC, AdrMode::ZP, vec!["INC $44".to_string()]), 0xE6),
(Opcode::new(Instr::INC, AdrMode::ABS, vec!["INC $4400".to_string()]), 0xEE),
(Opcode::new(Instr::INC, AdrMode::ZPX, vec!["INC $44,X".to_string()]), 0xF6),
(Opcode::new(Instr::INC, AdrMode::ABSX, vec!["INC $4400,X".to_string()]), 0xFE),

(Opcode::new(Instr::INX, AdrMode::IMPL, vec!["INX".to_string()]), 0xE8),

(Opcode::new(Instr::BEQ, AdrMode::REL, vec!["BEQ $10".to_string(), "BEQ label".to_string()]), 0xF0),

(Opcode::new(Instr::SED, AdrMode::IMPL, vec!["SED".to_string()]), 0xF8),
