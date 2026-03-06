use std::arch::asm;

use s86::Simulator;

#[test]
fn je() {
    let expected: u64;
    unsafe {
        asm!(
            "mov rax, 10",
            "cmp rax, 10",
            "je 3f",
            "mov rax, 8",
            "3:",
            out("rax") expected);
    }

    let source = "
    mov rax, 10
    cmp rax, 10
    je end
    mov rax, 8
    end:
";
    let mut simulator = Simulator::new(source);
    simulator.run();
    assert_eq!(u64::from_ne_bytes(simulator.registers.rax), expected);
}

#[test]
fn ja() {
    let expected: u64;
    unsafe {
        asm!(
            "mov rax, 10",
            "cmp rax, 10",
            "ja 3f",
            "mov rax, 8",
            "3:",
            out("rax") expected);
    }

    let source = "
    mov rax, 10
    cmp rax, 10
    ja end
    mov rax, 8
    end:
";
    let mut simulator = Simulator::new(source);
    simulator.run();
    assert_eq!(u64::from_ne_bytes(simulator.registers.rax), expected);
}
