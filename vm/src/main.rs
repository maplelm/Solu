mod vm;
use std::{alloc::System, time::SystemTime};
use vm::bytecode::{Instruction, Opcode};

/*
* 0x0000000000000000 0x0000000000000000 0x0000000000000001
* 0x0000000000000000 0x0000000000000001 0x00000000000003E8
* 0x0000000000000000 0x0000000000000002 0x0000000000000000
* 0x0000000000000019 0x0000000000000000
* 0x0000000000000019 0x0000000000000001
* 0x0000000000000019 0x0000000000000002
* 0x0000000000000006 0x0000000000000002 0x0000000000000002 0x0000000000000000
* 0x000000000000000A 0x0000000000000001 0x0000000000000001 0x0000000000000000
* 0x0000000000000019 0x0000000000000000
* 0x0000000000000019 0x0000000000000001
* 0x0000000000000019 0x0000000000000002
* 0x0000000000000018 0x0000000000000001 x0000000000000006 0xFFFFFFFFFFFFFFFA
* 0x000000000000001D
*/

use crate::vm::bytecode::Operand;
fn main() {
    let prog: Vec<vm::bytecode::Instruction> = vec![
        Instruction::new(Opcode::LoadI32, &[Operand::Reg(0), Operand::Int(1)]),
        Instruction::new(Opcode::LoadI32, &[Operand::Reg(1), Operand::Int(1_000)]),
        Instruction::new(Opcode::LoadI32, &[Operand::Reg(2), Operand::Int(0)]),
        Instruction::new(Opcode::Print, &[Operand::Reg(0)]),
        Instruction::new(Opcode::Print, &[Operand::Reg(1)]),
        Instruction::new(Opcode::Print, &[Operand::Reg(2)]),
        Instruction::new(
            Opcode::AddI32,
            &[Operand::Reg(2), Operand::Reg(2), Operand::Reg(0)],
        ),
        Instruction::new(
            Opcode::SubI32,
            &[Operand::Reg(1), Operand::Reg(1), Operand::Reg(0)],
        ),
        Instruction::new(Opcode::Print, &[Operand::Reg(0)]),
        Instruction::new(Opcode::Print, &[Operand::Reg(1)]),
        Instruction::new(Opcode::Print, &[Operand::Reg(2)]),
        Instruction::new(Opcode::Jmpnif, &[Operand::Reg(1), Operand::Int(-6)]),
        Instruction::new(Opcode::Halt, &[]),
    ];
    let mut vm = vm::cpu::Vm::new(prog);
    let vm_start = SystemTime::now();
    let _ = vm.start();
    let vm_end = SystemTime::now();

    let core_start = SystemTime::now();
    let mut x = 1_000_000;
    for i in 0..1_000_000 {
        x -= 1;
    }
    let core_end = SystemTime::now();

    println!("VM Time: {:?}", vm_end.duration_since(vm_start).unwrap());
    println!(
        "Core Time {:?}",
        core_end.duration_since(core_start).unwrap()
    );
}
