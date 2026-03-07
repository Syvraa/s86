use std::arch::asm;

use s86::Simulator;

#[test]
fn xor_single() {
    let expected: u64;
    unsafe {
        asm!("xor rax, 8", in("rax") 342, lateout("rax") expected );
    }

    let source = "
    xor rax, 8
";
    let mut simulator = Simulator::new(source);
    simulator.registers.rax = 342;
    simulator.run();
    assert_eq!(simulator.registers.rax, expected);
}

#[test]
fn xor_reg() {
    let expected: u64;
    unsafe {
        asm!(
            "xor rcx, rax", in("rax") 8, in("rcx") 342, lateout("rcx") expected);
    }

    let source = "
    xor rcx, rax
";
    let mut simulator = Simulator::new(source);
    simulator.registers.rax = 8;
    simulator.registers.rcx = 342;
    simulator.run();

    assert_eq!(simulator.registers.rcx, expected);
}
