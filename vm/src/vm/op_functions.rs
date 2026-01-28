use crate::vm::cpu::VmState;

use super::bytecode::*;
use super::cpu::Vm;

pub type DispatchOpcodeTable = [OpFn; 33];
pub const DISPATCH_TABLE: DispatchOpcodeTable = [
    load_i32,
    load_i64,
    load_f32,
    load_f64,
    mov,
    add_i32,
    add_i64,
    add_f32,
    add_f64,
    sub_i32,
    sub_i64,
    sub_f32,
    sub_f64,
    mul_i32,
    mul_i64,
    mul_f32,
    mul_f64,
    div_i32,
    div_i64,
    div_f32,
    div_f64,
    equal,
    greater_than,
    less_than,
    call,
    jmp,
    jmpif,
    jmpnif,
    print,
    addi32_ptr,
    addi64_ptr,
    ret,
    halt,
];

//////////////////
// Load OpCodes //
//////////////////
pub fn load_i32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let val = v.prog[(v.ip + 1) as usize];
    v.cs[v.active_frame].locals[dst] = Register {
        bits: val,
        kind: ValueType::I32,
    };
    v.ip += 2;
}

#[inline(always)]
pub fn load_i64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let val = v.prog[(v.ip + 1) as usize];
    v.cs[v.active_frame].locals[dst] = Register::new(val, ValueType::I64);
    v.ip += 2;
}

#[inline(always)]
pub fn load_f32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let val = v.prog[(v.ip + 1) as usize];
    v.cs[v.active_frame].locals[dst] = Register::new(val, ValueType::F32);
    v.ip += 2;
}

#[inline(always)]
pub fn load_f64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let val = v.prog[(v.ip + 1) as usize];
    v.cs[v.active_frame].locals[dst] = Register::new(val, ValueType::F64);
    v.ip += 2;
}

/////////////////
// Mov OpCodes //
/////////////////
#[inline(always)]
pub fn mov(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let src = v.prog[(v.ip + 1) as usize] as usize;
    v.cs[v.active_frame].locals[dst] = v.cs[v.active_frame].locals[src];
    v.ip += 2;
}

/////////////////
// Add OpCodes //
/////////////////
#[inline(always)]
pub fn add_i32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i32).wrapping_add(b.bits as i32) as u64,
        ValueType::I32,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn add_i64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i64).wrapping_add(b.bits as i64) as u64,
        ValueType::I64,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn add_f32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f32::from_bits(a.bits as u32) + f32::from_bits(b.bits as u32);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F32);
    v.ip += 3;
}

#[inline(always)]
pub fn add_f64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f64::from_bits(a.bits as u64) + f64::from_bits(b.bits as u64);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F64);
    v.ip += 3;
}

/////////////////
// Sub OpCodes //
/////////////////
#[inline(always)]
pub fn sub_i32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i32).wrapping_sub(b.bits as i32) as u64,
        ValueType::I32,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn sub_i64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i64).wrapping_sub(b.bits as i64) as u64,
        ValueType::I64,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn sub_f32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f32::from_bits(a.bits as u32) - f32::from_bits(b.bits as u32);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F32);
    v.ip += 3;
}

#[inline(always)]
pub fn sub_f64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f64::from_bits(a.bits as u64) - f64::from_bits(b.bits as u64);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F64);
    v.ip += 3;
}

/////////////////
// Mul OpCodes //
/////////////////
#[inline(always)]
pub fn mul_i32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i32).wrapping_mul(b.bits as i32) as u64,
        ValueType::I32,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn mul_i64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i64).wrapping_mul(b.bits as i64) as u64,
        ValueType::I64,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn mul_f32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f32::from_bits(a.bits as u32) * f32::from_bits(b.bits as u32);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F32);
    v.ip += 3;
}

#[inline(always)]
pub fn mul_f64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f64::from_bits(a.bits as u64) * f64::from_bits(b.bits as u64);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F64);
    v.ip += 3;
}

/////////////////
// Div OpCodes //
/////////////////
#[inline(always)]
pub fn div_i32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i32).wrapping_div(b.bits as i32) as u64,
        ValueType::I32,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn div_i64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    v.cs[v.active_frame].locals[dst] = Register::new(
        (a.bits as i64).wrapping_div(b.bits as i64) as u64,
        ValueType::I64,
    );
    v.ip += 3;
}

#[inline(always)]
pub fn div_f32(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f32::from_bits(a.bits as u32) / f32::from_bits(b.bits as u32);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F32);
    v.ip += 3;
}

#[inline(always)]
pub fn div_f64(v: &mut Vm) {
    let dst = v.prog[v.ip as usize] as usize;
    let ar = v.prog[(v.ip + 1) as usize] as usize;
    let br = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ar];
    let b = &v.cs[v.active_frame].locals[br];

    let output = f64::from_bits(a.bits as u64) / f64::from_bits(b.bits as u64);

    v.cs[v.active_frame].locals[dst] = Register::new(output.to_bits() as u64, ValueType::F64);
    v.ip += 3;
}

/////////////////////////
// Conditional Opcodes //
/////////////////////////

#[inline(always)]
pub fn equal(v: &mut Vm) {
    let jump_to = v.prog[v.ip as usize];
    let ra = v.prog[(v.ip + 1) as usize] as usize;
    let rb = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ra];
    let b = &v.cs[v.active_frame].locals[rb];
    if a.bits == b.bits {
        v.ip = (v.ip as i64 + jump_to as i64) as u64;
    } else {
        v.ip += 3;
    }
}

#[inline(always)]
pub fn greater_than(v: &mut Vm) {
    let jump_to = v.prog[v.ip as usize];
    let ra = v.prog[(v.ip + 1) as usize] as usize;
    let rb = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ra];
    let b = &v.cs[v.active_frame].locals[rb];
    if a.bits >= b.bits {
        v.ip = (v.ip as i64 + jump_to as i64) as u64;
    } else {
        v.ip += 3;
    }
}

#[inline(always)]
pub fn less_than(v: &mut Vm) {
    let jump_to = v.prog[v.ip as usize];
    let ra = v.prog[(v.ip + 1) as usize] as usize;
    let rb = v.prog[(v.ip + 2) as usize] as usize;
    let a = &v.cs[v.active_frame].locals[ra];
    let b = &v.cs[v.active_frame].locals[rb];
    if a.bits <= b.bits {
        v.ip = (v.ip as i64 + jump_to as i64) as u64;
    } else {
        v.ip += 3;
    }
}

/////////////////////
// Control OpCodes //
/////////////////////

#[inline(always)]
pub fn call(v: &mut Vm) {
    let target_fn = v.prog[v.ip as usize] as usize;
    let ret_dst = v.prog[v.ip as usize];
    let fn_def = &v.fn_table[target_fn];
    let frame = Frame::new(v.ip + 2, Some(ret_dst));
    v.active_frame += 1;
    v.ip = v.fn_table[target_fn].entry;
}

#[inline(always)]
pub fn jmp(v: &mut Vm) {
    let offset = v.prog[v.ip as usize];
    v.ip = (v.ip as isize + offset as isize) as u64;
}

#[inline(always)]
pub fn jmpif(v: &mut Vm) {
    let offset = v.prog[v.ip as usize] as isize;
    let peek = v.prog[(v.ip + 1) as usize] as usize;
    if v.cs[v.active_frame].locals[peek].zero() {
        v.ip = (v.ip as isize + offset) as u64;
    } else {
        v.ip += 2;
    }
}

#[inline(always)]
pub fn jmpnif(v: &mut Vm) {
    let offset = v.prog[v.ip as usize] as isize;
    let peek = v.prog[(v.ip + 1) as usize] as usize;
    if !v.cs[v.active_frame].locals[peek].zero() {
        v.ip = (v.ip as isize + offset) as u64;
    } else {
        v.ip += 2;
    }
}

#[inline(always)]
pub fn print(v: &mut Vm) {
    let src = v.prog[v.ip as usize] as usize;
    let reg = &v.cs[v.active_frame].locals[src];
    println!("Reg {}: Val: {}, Type: {:?}", src, reg.bits, reg.kind);
    v.ip += 1;
}

#[inline(always)]
pub fn addi32_ptr(v: &mut Vm) {
    let src = v.prog[v.ip as usize] as usize;
    let d = v.prog[(v.ip + 1) as usize] as u32;
    let ws = v.prog[(v.ip + 2) as usize] as u32;
    let reg = &mut v.cs[v.active_frame].locals[src];
    reg.bits = ((reg.bits as u32) + (d * ws)) as u64;
    v.ip += 3;
}

#[inline(always)]
pub fn addi64_ptr(v: &mut Vm) {
    let src = v.prog[v.ip as usize] as usize;
    let d = v.prog[(v.ip + 1) as usize] as u64;
    let ws = v.prog[(v.ip + 2) as usize] as u64;
    let reg = &mut v.cs[v.active_frame].locals[src];
    reg.bits = reg.bits + d * ws;
    v.ip += 3;
}

#[inline(always)]
pub fn ret(v: &mut Vm) {
    let finished = v.cs.pop().unwrap();
    if let Some(caller) = v.cs.last_mut() {
        if let Some(dst) = finished.ret_dst {
            caller.locals[dst as usize] = finished.locals[0];
        }
        v.ip = finished.ret_ip;
        v.active_frame -= 1;
    } else {
        // returned from main function
        v.ip = u64::MAX;
        v.halted = true;
    }
}

#[inline(always)]
pub fn halt(v: &mut Vm) {
    v.halted = true;
}
