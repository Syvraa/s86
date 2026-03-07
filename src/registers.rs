use bitfield::bitfield;

use crate::operands::Reg;

#[derive(Default)]
pub struct Registers {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub(crate) flags: Flags,
}

impl<'a> Registers {
    pub fn get_mut_reg(&'a mut self, reg: Reg) -> &'a mut u64 {
        type R = Reg;
        match reg {
            R::Rax => &mut self.rax,
            R::Rbx => &mut self.rbx,
            R::Rcx => &mut self.rcx,
            R::Rdx => &mut self.rdx,
            R::Rsi => &mut self.rsi,
            R::Rdi => &mut self.rdi,
            R::Rsp => &mut self.rsp,
            R::Rbp => &mut self.rbp,
            R::R8 => &mut self.r8,
            R::R9 => &mut self.r9,
            R::R10 => &mut self.r10,
            R::R11 => &mut self.r11,
            R::R12 => &mut self.r12,
            R::R13 => &mut self.r13,
            R::R14 => &mut self.r14,
            R::R15 => &mut self.r15,
        }
    }

    pub fn flags(&self) -> &Flags {
        &self.flags
    }
}

bitfield! {
    pub struct Flags(u64);
    bool;
    pub cf, set_cf: 0;
    pub zf, set_zf: 6;
    pub sf, set_sf: 7;
    pub of, set_of: 11;
}

impl Default for Flags {
    fn default() -> Self {
        // Bit 2 is always set in RFLAGS.
        Self(2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flags() {
        let mut regs = Registers::default();
        assert_eq!(regs.flags().0, 2);
        regs.flags.set_zf(true);
        assert_eq!(regs.flags().0, 66);
    }
}
