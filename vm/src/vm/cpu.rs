#![allow(unused)]
#![allow(dead_code, unreachable_patterns)]
use crate::vm::bytecode::FunctionEntry;
use crate::vm::bytecode::Instruction;
use crate::vm::bytecode::ValueType;
use crate::vm::bytecode::{Frame, Opcode};
use crate::vm::memory::{Heap, Stack};
use crate::vm::op_functions::{DISPATCH_TABLE, DispatchOpcodeTable};

const BYTECODE_MAGIC_VALUE: u32 = 0x44564D31;

pub struct Vm {
    pub prog: Vec<u64>,
    pub fn_table: Vec<FunctionEntry>,
    pub const_table: Vec<u32>, // not Implmemented right now
    pub halted: bool,
    pub h: Heap,
    pub active_frame: usize,
    pub cs: Vec<Frame>, // Call Stack
    pub ip: u64,
    pub wc: u64,
}

impl Vm {
    pub fn new(prog: Vec<u8>) -> Result<Self, u64> {
        let mut cursor: usize = 0;

        let magic = u32::from_le_bytes(prog[cursor..4].try_into().unwrap());
        if magic != BYTECODE_MAGIC_VALUE {
            // TODO: Make a real error value
            return Err(1);
        }
        cursor += 4;

        let version = u32::from_le_bytes(prog[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;

        let fn_table_count = u32::from_le_bytes(prog[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;

        let entry_index = u32::from_le_bytes(prog[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;

        let const_table_count = u32::from_le_bytes(prog[cursor..cursor + 4].try_into().unwrap());
        cursor += 4;

        let word_count = u64::from_le_bytes(prog[cursor..cursor + 8].try_into().unwrap());
        cursor += 8;

        // Function Table
        let fn_table: Vec<FunctionEntry> = Vec::with_capacity(fn_table_count as usize);
        for i in 0..fn_table_count {
            let entry_ip = u64::from_le_bytes(prog[cursor..cursor + 8].try_into().unwrap());
            cursor += 8;
            let reg_count = u16::from_le_bytes(prog[cursor..cursor + 2].try_into().unwrap());
            cursor += 2;
            let arg_count = u16::from_le_bytes(prog[cursor..cursor + 2].try_into().unwrap());
            cursor += 2;
            let mut arg_types: Vec<ValueType> = Vec::with_capacity(arg_count as usize);
            for t in 0..arg_count as usize {
                arg_types.push(prog[cursor + t].into());
            }
            cursor += arg_count as usize;

            let ret_type: Option<ValueType>;
            if prog[cursor] > 4 {
                ret_type = Some(prog[cursor].into());
            } else {
                ret_type = None;
            }
            cursor += 1;

            let flags = u32::from_le_bytes(prog[cursor..cursor + 4].try_into().unwrap());
            cursor += 4; // +2 for padding
            fn_table.push(FunctionEntry {
                entry: entry_ip,
                reg_count,
                arg_count,
                arg_types,
                ret_type,
                flags,
            });
        }

        // Const table here when implemented

        // Code Secion
        let mut code: Vec<u64> = Vec::with_capacity(word_count as usize);
        for _ in 0..word_count {
            code.push(u64::from_le_bytes(
                prog[cursor..cursor + 8].try_into().unwrap(),
            ));
            cursor += 8;
        }

        let entry_index = fn_table[entry_index as usize].entry;

        Ok(Self {
            prog: code,
            fn_table: fn_table,
            const_table: vec![],
            halted: false,
            h: Heap::new(1000),
            cs: vec![Frame::new(u64::MAX, None)],
            active_frame: 0,
            ip: entry_index,
            wc: word_count,
        })
    }

    pub fn start(&mut self) -> Result<usize, ()> {
        while !self.halted && self.ip < self.wc {
            self.step();
        }
        Ok(0)
    }

    fn step(&mut self) {
        let ins: u64 = self.prog[self.ip as usize];
        self.ip += 1;

        DISPATCH_TABLE[ins as usize](self);
    }
}
