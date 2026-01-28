use crate::vm::bytecode::Register;
#[allow(unused)]
/*
#[derive(Copy, Clone, Debug)]
pub enum Value {
    I32(u64),
    I64(u64),
    F32(u64),
    F64(u64),
    Ptr(u64),
}
*/

pub struct Heap {
    pub data: Vec<Option<HeapObject>>,
}

impl Heap {
    pub fn new(cap: u64) -> Self {
        Self {
            data: Vec::with_capacity(cap as usize),
        }
    }
    pub fn push(&mut self, obj: HeapObject) -> u64 {
        for i in 0..self.data.len() {
            if self.data[i].is_none() {
                self.data[i] = Some(obj);
                return i as u64;
            }
        }
        self.data.push(Some(obj));
        return (self.data.len() - 1) as u64;
    }

    pub fn free(&mut self, index: u64) {
        debug_assert!(index < self.data.len() as u64);
        self.data[index as usize] = None;
    }
}

pub enum HeapObject {
    String(String),
    Struct(Vec<u8>, u64), // (data, word size)
    Vector(Vec<u8>, u64), // (data, word size)
}

pub struct Stack {
    pub data: Vec<Register>,
}

impl Stack {
    pub fn new(cap: u64) -> Self {
        Self {
            data: Vec::with_capacity(cap as usize),
        }
    }
    pub fn push(&mut self, val: Register) {
        self.data.push(val);
    }

    pub fn pop(&mut self) -> Register {
        self.data.pop().unwrap()
    }
}
