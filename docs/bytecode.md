# Bytecode Specifications

## Metadata
### Header
 - `u32` Magic (a file signature so the VM know if it should keep going) `b'DVM1`
 - `u32` Version number
 - `u32` function table count
 - `u32` Entry point index
 - `u32` Const table count
 - `u64` Word Count (How many bytecodes are in the code secion in total)

### Function Table

 - `u64` entry point (ip value)
 - `u16` registers needed / used
 - `u16` arg count
 - `Vec<ValueType>` arg type
 - `ValueType` retrun type
 - `u32` flags (Extentions for later as of right now)
 - `u16` Padding 

### Const Table

 - `u32` size of data
 - ... number of bytes is the value based on the first number

## Opcodes

 - LoadI32
 - LoadI64
 - LoadF32
 - LoadF64
 - Mov
 - AddI32
 - AddI64
 - AddF32
 - AddF64
 - SubI32
 - SubI64
 - SubF32
 - SubF64
 - MulI32
 - MulI64
 - MulF32
 - MulF64
 - DivI32
 - DivI64
 - DivF32
 - DivF64
 - Jmp
 - Jmpif
 - Jmpnif
 - Print
 - AddI32Ptr
 - AddI64Ptr
 - Ret
 - Halt

## Representation

Each Bytecode Opcode will be the first byte in any instruction the number of
bytes consumed following the opcode will depend on the opcode in question for
example

```asm

; Loading Data into register 0x01
LoadI32 0x01 #50

``` 
This will sume 12 bytes becuase the first 8  bytes are assumed to be the
register that is to be loaded to `0x01 as u64` in this case and then 4 bytes for the
i32 value to put into the register.

## Notes

There are probably better way to encode this but this is just V1 and I am going to stick with it as I could take each part of the program and overegineer it for 10 years but then we would just have a bad V1 on in 10 years instead of a good v10.
