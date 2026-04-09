use std::arch::asm;

use s86::Simulator;

#[test]
fn mov_single() {
    let expected: u64;
    unsafe {
        asm!("mov rax, 8", out("rax") expected);
    }

    let source = "
    mov rax, 8
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn mov_reg() {
    let expected: u64;
    unsafe {
        asm!(
            "mov rax, 8",
            "mov rcx, rax", out("rcx") expected);
    }

    let source = "
    mov rax, 8
    mov rcx, rax
";
    let mut simulator = Simulator::new(source, 0).unwrap();
    simulator.run().unwrap();

    assert_eq!(simulator.registers.rcx, expected);
}
