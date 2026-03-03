use std::arch::asm;

use s86::Simulator;

#[test]
fn mov_negative() {
    let expected: i64;
    unsafe {
        asm!("mov rax, -8", out("rax") expected);
    }

    let source = "
    mov rax, -8
";
    let mut simulator = Simulator::new(source);
    simulator.run();
    assert_eq!(i64::from_le_bytes(simulator.registers.rax), expected);
}

#[test]
fn add_negative() {
    let mut expected: i64;

    unsafe {
        asm!(
            "mov rcx, 8",
            "mov rax, -8",
            "add rcx, rax", in("rax") -8, out("rcx") expected);
    }

    let source = "
    mov rcx, 8
    mov rax, -8
    add rcx, rax
";
    let mut simulator = Simulator::new(source);
    simulator.run();

    assert_eq!(i64::from_le_bytes(simulator.registers.rcx), expected);
}
