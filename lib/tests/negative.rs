use std::arch::asm;

use s86::Simulator;

#[test]
fn mov_negative() {
    let expected: u64;
    unsafe {
        asm!("mov rax, -8", out("rax") expected);
    }

    let source = "
    mov rax, -8
";
    let mut simulator = Simulator::new(source, 0);
    simulator.run();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn add_negative() {
    let mut expected: u64;

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
    let mut simulator = Simulator::new(source, 0);
    simulator.run();

    assert_eq!(simulator.registers.rcx, expected);
}
