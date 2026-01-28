use crate::vm::Vm;

// Types Aliases
pub type OpFn = fn(&mut Vm);

#[allow(unused)]
#[repr(u16)]
#[derive(Copy, Clone)]
pub enum Opcode {
    // Loads imm -> r1
    LoadI32,
    LoadI64,
    LoadF32,
    LoadF64,
    // Moves r1 -> r2
    Mov,
    // Adds r1 + r2
    AddI32,
    AddI64,
    AddF32,
    AddF64,
    // Subtracts r1 - r2
    SubI32,
    SubI64,
    SubF32,
    SubF64,
    // Multiply r1 * r2
    MulI32,
    MulI64,
    MulF32,
    MulF64,
    // Divide r1 / r2
    DivI32,
    DivI64,
    DivF32,
    DivF64,
    // Conditional Jumps
    Equal,
    GreaterThan,
    LessThan,
    Call,
    // Move Instruction Pointer by r1
    Jmp,
    // Move Instruction Pointer by r1 in r2 direction if r3 == 0
    Jmpif,
    // Move Instruction Pointer by r1 in r2 direction if r3 != 0
    Jmpnif,
    // Print (Debugging)
    Print,
    // Pointer into Arena
    AddI32Ptr,
    AddI64Ptr,
    // return value from frame
    Ret,
    // Halt the VM
    Halt,
}

impl Into<u16> for Opcode {
    fn into(self) -> u16 {
        match self {
            Opcode::LoadI32 => Opcode::LoadI32 as u16,
            Opcode::LoadI64 => Opcode::LoadI64 as u16,
            Opcode::LoadF32 => Opcode::LoadF32 as u16,
            Opcode::LoadF64 => Opcode::LoadF64 as u16,

            Opcode::Mov => Opcode::Mov as u16,

            Opcode::AddI32 => Opcode::AddI32 as u16,
            Opcode::AddI64 => Opcode::AddI64 as u16,
            Opcode::AddF32 => Opcode::AddF32 as u16,
            Opcode::AddF64 => Opcode::AddF64 as u16,

            Opcode::SubI32 => Opcode::SubI32 as u16,
            Opcode::SubI64 => Opcode::SubI64 as u16,
            Opcode::SubF32 => Opcode::SubF32 as u16,
            Opcode::SubF64 => Opcode::SubF64 as u16,

            Opcode::MulI32 => Opcode::LoadI32 as u16,
            Opcode::MulI64 => Opcode::LoadI32 as u16,
            Opcode::MulF32 => Opcode::LoadI32 as u16,
            Opcode::MulF64 => Opcode::LoadI32 as u16,

            Opcode::DivI32 => Opcode::LoadI32 as u16,
            Opcode::DivI64 => Opcode::LoadI32 as u16,
            Opcode::DivF32 => Opcode::LoadI32 as u16,
            Opcode::DivF64 => Opcode::LoadI32 as u16,

            Opcode::Equal => Opcode::Equal as u16,
            Opcode::GreaterThan => Opcode::GreaterThan as u16,
            Opcode::LessThan => Opcode::LessThan as u16,

            Opcode::Call => Opcode::Call as u16,
            Opcode::Jmp => Opcode::Jmp as u16,
            Opcode::Jmpif => Opcode::Jmpif as u16,
            Opcode::Jmpnif => Opcode::Jmpnif as u16,
            Opcode::Print => Opcode::Print as u16,
            Opcode::AddI32Ptr => Opcode::AddI32Ptr as u16,
            Opcode::AddI64Ptr => Opcode::AddI64Ptr as u16,
            Opcode::Ret => Opcode::Ret as u16,
            Opcode::Halt => Opcode::Halt as u16,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
    Ptr,
}

#[derive(Copy, Clone)]
pub struct Register {
    pub bits: u64,
    pub kind: ValueType,
}

impl Register {
    #[inline(always)]
    pub fn new(bits: u64, kind: ValueType) -> Self {
        Self { bits, kind }
    }
    #[inline(always)]
    pub fn zero(&self) -> bool {
        self.bits == 0
    }
}

// Instruction
// u16: opcode
// U16: op1
// U16: op2
// U16: op3
pub struct Instruction(u64);

impl From<u64> for Instruction {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Into<u64> for Instruction {
    fn into(self) -> u64 {
        self.0
    }
}

impl Instruction {
    #[inline(always)]

    pub fn new(data: u64) -> Self {
        Self(data)
    }

    pub fn code(&self) -> u16 {
        (self.0 >> 48) as u16
    }

    pub fn op1(&self) -> u16 {
        (self.0 >> 32) as u16
    }

    pub fn op2(&self) -> u16 {
        (self.0 >> 16) as u16
    }

    pub fn op3(&self) -> u16 {
        self.0 as u16
    }
}

pub const FRAME_LOCALS_SIZE: usize = 300;
pub struct Frame {
    pub locals: [Register; FRAME_LOCALS_SIZE],
    pub ret_ip: u64,
    pub ret_dst: Option<u64>,
}

impl Frame {
    pub fn new(ret_point: u64, ret_dst: Option<u64>) -> Self {
        Self {
            locals: [Register {
                bits: 0,
                kind: ValueType::I32,
            }; FRAME_LOCALS_SIZE],
            ret_ip: ret_point,
            ret_dst: ret_dst,
        }
    }
}

pub struct FunctionEntry {
    pub entry: u64,
    pub reg_count: u16,
    pub arg_count: u16,
    pub arg_types: Vec<ValueType>,
    pub ret_type: Option<ValueType>,

    pub flags: u32,
}

impl Into<ValueType> for u8 {
    fn into(self) -> ValueType {
        match self {
            0 => ValueType::I32,
            1 => ValueType::I64,
            2 => ValueType::F32,
            3 => ValueType::F64,
            4 => ValueType::Ptr,
            _ => unreachable!(),
        }
    }
}
