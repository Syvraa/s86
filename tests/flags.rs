use std::arch::asm;

use s86::Simulator;

#[test]
fn add_max() {
    let input: u64 = u64::MAX;
    let expected: u64;
    unsafe {
        asm!(
            "add rax, 1",
            "pushfq",
            "pop rcx",
            in("rax") input, out("rcx") expected);
    }

    let source = "
    add rax, 1
";
    let mut simulator = Simulator::new(source, 0);
    simulator.registers.rax = input;
    simulator.run();
    assert_eq!(simulator.registers.flags().0, extract_flags(expected));
}

#[test]
fn sub_max() {
    let input: u64 = u64::MAX;
    let expected: u64;
    unsafe {
        asm!(
            "sub rax, 1",
            "pushfq",
            "pop rcx",
            in("rax") input, out("rcx") expected);
    }

    let source = "
    sub rax, 1
";
    let mut simulator = Simulator::new(source, 0);
    simulator.registers.rax = input;
    simulator.run();
    assert_eq!(simulator.registers.flags().0, extract_flags(expected));
}

#[test]
fn add_min() {
    let input: u64 = u64::MIN;
    let expected: u64;
    unsafe {
        asm!(
            "add rax, 1",
            "pushfq",
            "pop rcx",
            in("rax") input, out("rcx") expected);
    }

    let source = "
    add rax, 1
";
    let mut simulator = Simulator::new(source, 0);
    simulator.registers.rax = input;
    simulator.run();
    assert_eq!(simulator.registers.flags().0, extract_flags(expected));
}

#[test]
fn sub_min() {
    let input: u64 = u64::MIN;
    let expected: u64;
    unsafe {
        asm!(
            "sub rax, 1",
            "pushfq",
            "pop rcx",
            in("rax") input, out("rcx") expected);
    }

    let source = "
    sub rax, 1
";
    let mut simulator = Simulator::new(source, 0);
    simulator.registers.rax = input;
    simulator.run();
    assert_eq!(simulator.registers.flags().0, extract_flags(expected));
}

fn extract_flags(flags: u64) -> u64 {
    flags & 0b10011101011
}
