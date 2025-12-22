import operator

# Constants for opcodes
LOAD = 0x01
STORE = 0x02
ADD = 0x03
SUB = 0x04
HALT = 0xff

# Stretch goals
ADDI = 0x05
SUBI = 0x06
JUMP = 0x07
BEQZ = 0x08

MEMORY_SHIFT = 8

PC = 0
R1 = 1
R2 = 2


def compute(memory):
    """
    Given a 256 byte array of "memory", run the stored program
    to completion, modifying the data in place to reflect the result

    The memory format is:

    00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f ... ff
    **        **... __
    ^==DATA===============^ ^==INSTRUCTIONS==============^
    """
    registers = [MEMORY_SHIFT, 0, 0]  # PC, R1 and R2

    handlers = {
        LOAD: load,
        STORE: store,
        ADD: lambda m, r, idx: arithmetic_op(m, r, idx, operator.add),
        SUB: lambda m, r, idx: arithmetic_op(m, r, idx, operator.sub),
    }

    while True:
        command_idx = registers[PC]
        command = get_cmd(memory, command_idx)
        if command == HALT:
            break
        handler = handlers.get(command)
        if handler:
            registers[PC] = handler(memory, registers, command_idx)
        else:
            raise ValueError(f"Unknown command {command}")


def set_mem(memory, idx, value):
    assert 0 <= idx < MEMORY_SHIFT
    memory[idx] = value


def get_mem(memory, idx):
    assert 0 <= idx < MEMORY_SHIFT
    return memory[idx]


def get_cmd(memory, idx):
    assert MEMORY_SHIFT <= idx < len(memory)
    return memory[idx]


def load(memory, registers, command_idx) -> int:
    CMD_LEN = 3
    reg_idx, mem_addr = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert reg_idx in [0x01, 0x02] 
    registers[reg_idx] = get_mem(memory, mem_addr)
    return command_idx + CMD_LEN


def store(memory, registers, command_idx) -> int:
    CMD_LEN = 3
    reg_idx, mem_addr = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert reg_idx in [0x01, 0x02] 
    set_mem(memory, mem_addr, registers[reg_idx])
    return command_idx + CMD_LEN


def arithmetic_op(memory, registers, command_idx, op) -> int:
    CMD_LEN = 3
    reg1, reg2 = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert reg1 in [0x01, 0x02] 
    assert reg2 in [0x01, 0x02]
    registers[reg1] = op(registers[reg1], registers[reg2]) & 0xFF
    return command_idx + CMD_LEN
