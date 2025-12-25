pub const LOAD: u8 = 0x01;
pub const STORE: u8 = 0x02;
pub const ADD: u8 = 0x03;
pub const SUB: u8 = 0x04;
pub const HALT: u8 = 0xff;
pub const ADDI: u8 = 0x05;
pub const SUBI: u8 = 0x06;
pub const JUMP: u8 = 0x07;
pub const BEQZ: u8 = 0x08;

const PC: usize = 0;
const R1: usize = 1;
const R2: usize = 2;

const MEMORY_SHIFT: usize = 8;

pub fn compute(memory: &mut [u8; 256]) {
    let mut registers: [usize; 3] = [MEMORY_SHIFT, 0, 0];

    loop {
        let pc = registers[PC];
        let op = memory[pc];

        registers[PC] = match op {
            HALT => break,
            LOAD => load(memory, &mut registers, pc),
            STORE => store(memory, &registers, pc),
            ADD => arithmetic_op(memory, &mut registers, pc, u8::wrapping_add),
            SUB => arithmetic_op(memory, &mut registers, pc, u8::wrapping_sub),
            ADDI => arithmetic_immediate_op(memory, &mut registers, pc, u8::wrapping_add),
            SUBI => arithmetic_immediate_op(memory, &mut registers, pc, u8::wrapping_sub),
            JUMP => jump(memory, pc),
            BEQZ => beqz(memory, &registers, pc),
            _ => panic!("Unknown opcode: {:#04x}", op),
        };
    }
}

fn get_mem(memory: &[u8; 256], idx: usize) -> u8 {
    assert!(idx < MEMORY_SHIFT, "Memory index out of data region");
    memory[idx]
}

fn set_mem(memory: &mut [u8; 256], idx: usize, value: u8) {
    assert!(idx < MEMORY_SHIFT, "Memory index out of data region");
    memory[idx] = value;
}

fn get_cmd(memory: &[u8; 256], idx: usize) -> u8 {
    assert!(
        idx >= MEMORY_SHIFT && idx < memory.len(),
        "Command index out of instruction region"
    );
    memory[idx]
}

fn get_idx(memory: &[u8; 256], idx: usize) -> usize {
    get_cmd(memory, idx) as usize
}

fn load(memory: &[u8; 256], registers: &mut [usize; 3], cmd_idx: usize) -> usize {
    const CMD_LEN: usize = 3;
    let reg_idx = get_idx(memory, cmd_idx + 1);
    let mem_addr = get_idx(memory, cmd_idx + 2);
    assert!(reg_idx == R1 || reg_idx == R2);
    registers[reg_idx] = get_mem(memory, mem_addr) as usize;
    cmd_idx + CMD_LEN
}

fn store(memory: &mut [u8; 256], registers: &[usize; 3], cmd_idx: usize) -> usize {
    const CMD_LEN: usize = 3;
    let reg_idx = get_idx(memory, cmd_idx + 1);
    let mem_addr = get_idx(memory, cmd_idx + 2);
    assert!(reg_idx == R1 || reg_idx == R2);
    set_mem(memory, mem_addr, registers[reg_idx] as u8);
    cmd_idx + CMD_LEN
}

fn arithmetic_op(
    memory: &[u8; 256],
    registers: &mut [usize; 3],
    cmd_idx: usize,
    op: fn(u8, u8) -> u8,
) -> usize {
    const CMD_LEN: usize = 3;
    let reg1 = get_idx(memory, cmd_idx + 1);
    let reg2 = get_idx(memory, cmd_idx + 2);
    assert!(reg1 == R1 || reg1 == R2);
    assert!(reg2 == R1 || reg2 == R2);
    registers[reg1] = op(registers[reg1] as u8, registers[reg2] as u8) as usize;
    cmd_idx + CMD_LEN
}

fn arithmetic_immediate_op(
    memory: &[u8; 256],
    registers: &mut [usize; 3],
    cmd_idx: usize,
    op: fn(u8, u8) -> u8,
) -> usize {
    const CMD_LEN: usize = 3;
    let reg1 = get_idx(memory, cmd_idx + 1);
    let value = get_cmd(memory, cmd_idx + 2);
    assert!(reg1 == R1 || reg1 == R2);
    registers[reg1] = op(registers[reg1] as u8, value) as usize;
    cmd_idx + CMD_LEN
}

fn jump(memory: &[u8; 256], cmd_idx: usize) -> usize {
    let jump_to = get_idx(memory, cmd_idx + 1);
    assert!(jump_to >= MEMORY_SHIFT && jump_to < memory.len());
    jump_to
}

fn beqz(memory: &[u8; 256], registers: &[usize; 3], cmd_idx: usize) -> usize {
    const CMD_LEN: usize = 3;
    let reg1 = get_idx(memory, cmd_idx + 1);
    let offset = get_idx(memory, cmd_idx + 2);
    assert!(reg1 == R1 || reg1 == R2);
    if registers[reg1] == 0 {
        let jump_to = cmd_idx + CMD_LEN + offset;
        assert!(jump_to >= MEMORY_SHIFT && jump_to < memory.len());
        jump_to
    } else {
        cmd_idx + CMD_LEN
    }
}
