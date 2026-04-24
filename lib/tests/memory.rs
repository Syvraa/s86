use s86::Simulator;
use s86::diff::DiffReg;
use s86::diff::MemDiff;
use s86::diff::RegDiff;
use s86::diff::StateDiff;

#[test]
fn read_dword() {
    let source = "
    mov rax, qword [0]
";
    let mut simulator = Simulator::new(source, 16).unwrap();
    let diff = simulator.step().unwrap();

    assert_eq!(
        diff.reg_diffs,
        vec![RegDiff {
            reg: DiffReg::Rax,
            value: 0
        }]
    );
    assert_eq!(simulator.registers.rax, 0);
}

#[test]
fn write_byte() {
    let source = "
    mov byte [1], 8
    mov ah, byte [1]
";
    let mut simulator = Simulator::new(source, 16).unwrap();
    let diffs = vec![simulator.step().unwrap(), simulator.step().unwrap()];

    assert_eq!(
        diffs,
        vec![
            StateDiff {
                reg_diffs: vec![],
                mem_diffs: vec![MemDiff {
                    address: 1,
                    value: 8
                }],
            },
            StateDiff {
                reg_diffs: vec![RegDiff {
                    reg: DiffReg::Rax,
                    value: 8
                }],
                mem_diffs: vec![]
            }
        ]
    );
    assert_eq!(simulator.registers.rax, 8 << 8);
}

#[test]
fn write_dword() {
    let source = "
    mov dword [1], 8
    mov eax, dword [1]
";
    let mut simulator = Simulator::new(source, 16).unwrap();
    simulator.run().unwrap();

    assert_eq!(simulator.registers.rax, 8);
}

#[test]
fn dynamic_address() {
    let source = "
    mov rax, 1
    mov qword [rax*8], 16
    mov rbx, qword [rax*8]
";
    let mut simulator = Simulator::new(source, 16).unwrap();
    simulator.run().unwrap();

    assert_eq!(simulator.registers.rbx, 16);
}

#[test]
fn offset() {
    let source = "
    mov byte [1+2], 3
    mov al, byte [1+2]
";
    let mut simulator = Simulator::new(source, 16).unwrap();
    simulator.run().unwrap();

    assert_eq!(simulator.registers.rax, 3);
}
