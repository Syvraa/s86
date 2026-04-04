use std::arch::asm;

use s86::Simulator;

#[test]
fn jmp() {
    let expected: u64;
    unsafe {
        asm!(
            "mov rax, 10",
            "jmp 2f",
            "mov rax, 8",
            "2:",
            "mov rax, 9",
            out("rax") expected);
    }

    let source = "
    jmp label
    mov rax, 8
    label:
    mov rax, 9
";
    let mut simulator = Simulator::new(source, 0);
    simulator.registers.rax = 10;
    simulator.run();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn jmp_sublabel() {
    let source = "
    label:
    jmp .sub
    mov rax, 8
    .sub:
    mov rax, 9
";
    let mut simulator = Simulator::new(source, 0);
    simulator.run();
    assert_eq!(simulator.registers.rax, 9);
}
