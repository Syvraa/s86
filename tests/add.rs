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
    simulator.registers.rax = 8;
    simulator.run();
    assert_eq!(simulator.registers.rax, expected);
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
    simulator.registers.rax = 8;
    simulator.registers.rcx = 8;
    simulator.run();

    assert_eq!(simulator.registers.rcx, expected);
}
