from collections import defaultdict

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

    while True:  # keep looping, like a physical computer's clock
        command_idx = registers[PC]
        command = get_cmd(memory, command_idx)
        if command == HALT:
            break
        elif command == LOAD:
            registers[PC] = load(memory, registers, command_idx)
        elif command == STORE:
            registers[PC] = store(memory, registers, command_idx)
        elif command == ADD:
            registers[PC] = add(memory, registers, command_idx)
        elif command == SUB:
            registers[PC] = subtract(memory, registers, command_idx)
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
    load_to, load_from = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert load_to in [0x01, 0x02]
    match load_to:
        case 0x01:
            registers[R1] = get_mem(memory, load_from)
        case 0x02:
            registers[R2] = get_mem(memory, load_from)
    return command_idx + CMD_LEN

def store(memory, registers, command_idx) -> int:
    CMD_LEN = 3
    store_from, store_to = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert store_from in [0x01, 0x02]
    match store_from:
        case 0x01:
            set_mem(memory, store_to, registers[R1])
        case 0x02:
            set_mem(memory, store_to, registers[R2])
    return command_idx + CMD_LEN

def add(memory, registers, command_idx) -> int:
    CMD_LEN = 3
    add_from_1, add_from_2 = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert add_from_1 in [0x01, 0x02]
    assert add_from_2 in [0x01, 0x02]
    registers[add_from_1] = (registers[add_from_1] + registers[add_from_2]) & 0xFF
    return command_idx + CMD_LEN

def subtract(memory, registers, command_idx) -> int:
    CMD_LEN = 3
    min, sub = get_cmd(memory, command_idx + 1), get_cmd(memory, command_idx + 2)
    assert min in [0x01, 0x02]
    assert sub in [0x01, 0x02]
    registers[min] = (registers[min] - registers[sub]) & 0xFF
    return command_idx + CMD_LEN


    





        

