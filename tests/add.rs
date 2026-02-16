use std::arch::asm;

use s86::Simulator;

#[test]
fn add_single() {
    let expected: u64;
    unsafe {
        asm!("add rax, 8", in("rax") 8, lateout("rax") expected );
    }

    let source = "
    add rax, 8
";
    let mut simulator = Simulator::new(source);
    simulator.registers.rax = u64::to_ne_bytes(8);
    simulator.run();
    assert_eq!(u64::from_ne_bytes(simulator.registers.rax), expected);
}

#[test]
fn add_reg() {
    let expected: u64;
    unsafe {
        asm!(
            "add rcx, rax", in("rax") 8, in("rcx") 8, lateout("rcx") expected);
    }

    let source = "
    add rcx, rax
";
    let mut simulator = Simulator::new(source);
    simulator.registers.rax = u64::to_ne_bytes(8);
    simulator.registers.rcx = u64::to_ne_bytes(8);
    simulator.run();

    assert_eq!(u64::from_ne_bytes(simulator.registers.rcx), expected);
}
