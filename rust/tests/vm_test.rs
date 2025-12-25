use vm::compute;

struct VmCase {
    x: u8,
    y: u8,
    out: u8,
}

struct VmTest {
    name: &'static str,
    asm: &'static str,
    cases: Vec<VmCase>,
}

fn main_tests() -> Vec<VmTest> {
    vec![
        VmTest {
            name: "Halt",
            asm: "halt",
            cases: vec![VmCase { x: 0, y: 0, out: 0 }],
        },
        VmTest {
            name: "LoadStore",
            asm: "load r1 1
store r1 0
halt",
            cases: vec![
                VmCase { x: 1, y: 0, out: 1 },
                VmCase {
                    x: 255,
                    y: 0,
                    out: 255,
                },
            ],
        },
        VmTest {
            name: "Add",
            asm: "load r1 1
load r2 2
add r1 r2
store r1 0
halt",
            cases: vec![
                VmCase { x: 1, y: 2, out: 3 }, // 1 + 2 = 3
                VmCase {
                    x: 254,
                    y: 1,
                    out: 255,
                }, // support max int
                VmCase {
                    x: 255,
                    y: 1,
                    out: 0,
                }, // correctly overflow
            ],
        },
        VmTest {
            name: "Subtract",
            asm: "load r1 1
load r2 2
sub r1 r2
store r1 0
halt",
            cases: vec![
                VmCase { x: 5, y: 3, out: 2 },
                VmCase {
                    x: 0,
                    y: 1,
                    out: 255,
                }, // correctly overflow backwards
            ],
        },
    ]
}

fn stretch_goal_tests() -> Vec<VmTest> {
    vec![
        VmTest {
            name: "Jump",
            asm: "load r1 1
jump 16
store r1 0
halt",
            cases: vec![VmCase {
                x: 42,
                y: 0,
                out: 0,
            }],
        },
        VmTest {
            name: "Beqz",
            asm: "load r1 1
load r2 2
beqz r2 3
store r1 0
halt",
            cases: vec![
                VmCase {
                    x: 42,
                    y: 0,
                    out: 0,
                }, // r2 is zero, so should branch over the store
                VmCase {
                    x: 42,
                    y: 1,
                    out: 42,
                }, // r2 is nonzero, so should store back 42
            ],
        },
        VmTest {
            name: "Addi",
            asm: "load r1 1
addi r1 3
addi r1 5
store r1 0
halt",
            cases: vec![
                VmCase { x: 0, y: 0, out: 8 }, // 0 + 3 + 5 = 8
                VmCase {
                    x: 20,
                    y: 0,
                    out: 28,
                }, // 20 + 3 + 5 = 28
            ],
        },
        VmTest {
            name: "Sum to n",
            asm: "load r1 1
beqz r1 8
add r2 r1
subi r1 1
jump 11
store r2 0
halt",
            cases: vec![
                VmCase { x: 0, y: 0, out: 0 },
                VmCase { x: 1, y: 0, out: 1 },
                VmCase {
                    x: 5,
                    y: 0,
                    out: 15,
                },
                VmCase {
                    x: 10,
                    y: 0,
                    out: 55,
                },
            ],
        },
    ]
}

#[test]
fn test_main() {
    for test in main_tests() {
        run_vm_test(&test);
    }
}

#[test]
#[ignore = "Run with `cargo test -- --ignored` or `cargo test -- --include-ignored` to include stretch goals"]
fn test_stretch_goals() {
    for test in stretch_goal_tests() {
        run_vm_test(&test);
    }
}

fn print_data_area(memory: &[u8; 256]) {
    print!("[");
    for i in 0..8 {
        if i > 0 {
            print!(", ");
        }
        print!("{:02x}", memory[i]);
    }
    println!("]");
}

fn run_vm_test(test: &VmTest) {
    let code = assemble(test.asm);

    for case in &test.cases {
        let mut memory = [0u8; 256];
        memory[8..8 + code.len()].copy_from_slice(&code);
        memory[1] = case.x;
        memory[2] = case.y;

        println!("[{}] f({}, {})", test.name, case.x, case.y);
        print!("  before: ");
        print_data_area(&memory);

        compute(&mut memory);

        print!("  after:  ");
        print_data_area(&memory);

        let actual = memory[0];
        assert_eq!(
            actual, case.out,
            "[{}] Expected f({}, {}) to be {}, not {}",
            test.name, case.x, case.y, case.out, actual
        );
    }
}

fn reg(s: &str) -> u8 {
    match s {
        "r1" => 0x01,
        "r2" => 0x02,
        _ => panic!("Unknown register: {}", s),
    }
}

fn mem(s: &str) -> u8 {
    s.parse().expect("Invalid memory address")
}

fn imm(s: &str) -> u8 {
    mem(s)
}

fn assemble(asm: &str) -> Vec<u8> {
    let mut mc = Vec::new();
    for line in asm.trim().lines() {
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "load" => mc.extend_from_slice(&[0x01, reg(parts[1]), mem(parts[2])]),
            "store" => mc.extend_from_slice(&[0x02, reg(parts[1]), mem(parts[2])]),
            "add" => mc.extend_from_slice(&[0x03, reg(parts[1]), reg(parts[2])]),
            "sub" => mc.extend_from_slice(&[0x04, reg(parts[1]), reg(parts[2])]),
            "addi" => mc.extend_from_slice(&[0x05, reg(parts[1]), imm(parts[2])]),
            "subi" => mc.extend_from_slice(&[0x06, reg(parts[1]), imm(parts[2])]),
            "jump" => mc.extend_from_slice(&[0x07, imm(parts[1])]),
            "beqz" => mc.extend_from_slice(&[0x08, reg(parts[1]), imm(parts[2])]),
            "halt" => mc.push(0xff),
            _ => panic!("Invalid operation: {}", parts[0]),
        }
    }
    mc
}
