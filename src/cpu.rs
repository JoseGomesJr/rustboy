use crate::virtual_memory::VirtualMemory;

struct Register{
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

struct RegistersFlags {
    zero: bool,
    operation_subtract: bool,
    half_carry: bool,
    carry: bool
}

impl Register {
    pub fn get_af(self, value: &mut u16){
        *value = (self.a as u16) << 8 | self.f as u16;
    }
    pub fn set_af(&mut self, value : u16){
        self.a = ((value & 0xFF00) << 8) as u8;
        self.f = ((value & 0x00FF) >> 8) as u8;
    }

    pub fn get_bc(self, value: &mut u16){
        *value = (self.b as u16) << 8 | self.c as u16;
    }

    pub fn set_bc(&mut self, value : u16){
        self.b = ((value & 0xFF00) << 8) as u8;
        self.c = ((value & 0x00FF) >> 8) as u8;
    }

    pub fn get_de(self, value: &mut u16){
        *value = (self.d as u16) << 8 | self.e as u16;
    }

    pub fn set_de(&mut self, value : u16){
        self.d = ((value & 0xFF00) << 8) as u8;
        self.e = ((value & 0x00FF) >> 8) as u8;
    }

    pub fn get_hl(self, value: &mut u16){
        *value = (self.h as u16) << 8 | self.l as u16;
    }

    pub fn set_hl(&mut self, value : u16){
        self.h = ((value & 0xFF00) << 8) as u8;
        self.l = ((value & 0x00FF) >> 8) as u8;
    }
}

enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL,
    ADDSP,
    SUB(ArithmeticTarget),
}

enum ArithmeticTarget {
    A, B, C, D, E, H, L, HL, N(u16)
}

struct CPU{
    registers : Register,
    register_flag: RegistersFlags,
    pc: u16,
    sp : u16,
    memory_bus : VirtualMemory,
}

impl CPU {
    pub fn execute(&mut self, instruction: Instruction){
        match instruction {
            Instruction::ADD(target_register) =>{

                self.registers.a = match target_register {
                    ArithmeticTarget::B =>  self.add(self.registers.b),
                    ArithmeticTarget::C =>  self.add(self.registers.c),
                    ArithmeticTarget::D => self.add(self.registers.d),
                    ArithmeticTarget::E => self.add(self.registers.e),
                    ArithmeticTarget::H => self.add(self.registers.h),
                    ArithmeticTarget::L => self.add(self.registers.l),
                    _ => {}
                }
            }
            _ => {}
        }

    }

    fn add(&mut self, value : u8) -> u8 {
        let (result_value, flag_operation) = self.registers.a.overflowing_add(value);

        self.register_flag.zero = result_value == 0;
        self.register_flag.operation_subtract = false;
        self.register_flag.carry = flag_operation;

        self.register_flag.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;

        result_value
    }
}