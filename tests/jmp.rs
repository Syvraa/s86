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
    let mut simulator = Simulator::new(source);
    simulator.registers.rax = u64::to_ne_bytes(10);
    simulator.run();
    assert_eq!(u64::from_ne_bytes(simulator.registers.rax), expected);
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
    let mut simulator = Simulator::new(source);
    simulator.run();
    assert_eq!(u64::from_ne_bytes(simulator.registers.rax), 9);
}
