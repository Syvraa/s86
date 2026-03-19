#![allow(clippy::cast_possible_truncation)]

use bitfield::bitfield;

use crate::operands::{ByteReg, DwordReg, QwordReg, Reg, WordReg};

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

impl Registers {
    pub fn read(&self, reg: Reg) -> u64 {
        match reg {
            Reg::Qword(reg) => self.read_qword(reg),
            Reg::Dword(reg) => u64::from(self.read_dword(reg)),
            Reg::Word(reg) => u64::from(self.read_word(reg)),
            Reg::Byte(reg) => u64::from(self.read_byte(reg)),
        }
    }

    pub fn write(&mut self, reg: Reg, value: u64) {
        match reg {
            Reg::Qword(reg) => self.write_qword(reg, value),
            Reg::Dword(reg) => self.write_dword(reg, value as u32),
            Reg::Word(reg) => self.write_word(reg, value as u16),
            Reg::Byte(reg) => self.write_byte(reg, value as u8),
        }
    }

    fn read_qword(&self, reg: QwordReg) -> u64 {
        type QR = QwordReg;
        match reg {
            QR::Rax => self.rax,
            QR::Rbx => self.rbx,
            QR::Rcx => self.rcx,
            QR::Rdx => self.rdx,
            QR::Rsi => self.rsi,
            QR::Rdi => self.rdi,
            QR::Rsp => self.rsp,
            QR::Rbp => self.rbp,
            QR::R8 => self.r8,
            QR::R9 => self.r9,
            QR::R10 => self.r10,
            QR::R11 => self.r11,
            QR::R12 => self.r12,
            QR::R13 => self.r13,
            QR::R14 => self.r14,
            QR::R15 => self.r15,
        }
    }

    fn read_dword(&self, reg: DwordReg) -> u32 {
        type DR = DwordReg;
        (match reg {
            DR::Eax => self.rax,
            DR::Ebx => self.rbx,
            DR::Ecx => self.rcx,
            DR::Edx => self.rdx,
            DR::Esi => self.rsi,
            DR::Edi => self.rdi,
            DR::Esp => self.rsp,
            DR::Ebp => self.rbp,
            DR::R8d => self.r8,
            DR::R9d => self.r9,
            DR::R10d => self.r10,
            DR::R11d => self.r11,
            DR::R12d => self.r12,
            DR::R13d => self.r13,
            DR::R14d => self.r14,
            DR::R15d => self.r15,
        }) as u32
    }

    fn read_word(&self, reg: WordReg) -> u16 {
        type WR = WordReg;
        (match reg {
            WR::Ax => self.rax,
            WR::Bx => self.rbx,
            WR::Cx => self.rcx,
            WR::Dx => self.rdx,
            WR::Si => self.rsi,
            WR::Di => self.rdi,
            WR::Sp => self.rsp,
            WR::Bp => self.rbp,
            WR::R8w => self.r8,
            WR::R9w => self.r9,
            WR::R10w => self.r10,
            WR::R11w => self.r11,
            WR::R12w => self.r12,
            WR::R13w => self.r13,
            WR::R14w => self.r14,
            WR::R15w => self.r15,
        }) as u16
    }

    fn read_byte(&self, reg: ByteReg) -> u8 {
        type BR = ByteReg;
        (match reg {
            BR::Ah => self.rax >> 8,
            BR::Al => self.rax,
            BR::Bh => self.rbx >> 8,
            BR::Bl => self.rbx,
            BR::Ch => self.rcx >> 8,
            BR::Cl => self.rcx,
            BR::Dh => self.rdx >> 8,
            BR::Dl => self.rdx,
            BR::Sil => self.rsi,
            BR::Dil => self.rdi,
            BR::Spl => self.rsp,
            BR::Bpl => self.rbp,
            BR::R8b => self.r8,
            BR::R9b => self.r9,
            BR::R10b => self.r10,
            BR::R11b => self.r11,
            BR::R12b => self.r12,
            BR::R13b => self.r13,
            BR::R14b => self.r14,
            BR::R15b => self.r15,
        }) as u8
    }

    fn write_qword(&mut self, reg: QwordReg, value: u64) {
        type QR = QwordReg;
        let reg = match reg {
            QR::Rax => &mut self.rax,
            QR::Rbx => &mut self.rbx,
            QR::Rcx => &mut self.rcx,
            QR::Rdx => &mut self.rdx,
            QR::Rsi => &mut self.rsi,
            QR::Rdi => &mut self.rdi,
            QR::Rsp => &mut self.rsp,
            QR::Rbp => &mut self.rbp,
            QR::R8 => &mut self.r8,
            QR::R9 => &mut self.r9,
            QR::R10 => &mut self.r10,
            QR::R11 => &mut self.r11,
            QR::R12 => &mut self.r12,
            QR::R13 => &mut self.r13,
            QR::R14 => &mut self.r14,
            QR::R15 => &mut self.r15,
        };
        *reg = value;
    }

    fn write_dword(&mut self, reg: DwordReg, value: u32) {
        type DR = DwordReg;
        let reg = match reg {
            DR::Eax => &mut self.rax,
            DR::Ebx => &mut self.rbx,
            DR::Ecx => &mut self.rcx,
            DR::Edx => &mut self.rdx,
            DR::Esi => &mut self.rsi,
            DR::Edi => &mut self.rdi,
            DR::Esp => &mut self.rsp,
            DR::Ebp => &mut self.rbp,
            DR::R8d => &mut self.r8,
            DR::R9d => &mut self.r9,
            DR::R10d => &mut self.r10,
            DR::R11d => &mut self.r11,
            DR::R12d => &mut self.r12,
            DR::R13d => &mut self.r13,
            DR::R14d => &mut self.r14,
            DR::R15d => &mut self.r15,
        };
        *reg = u64::from(value);
    }

    fn write_word(&mut self, reg: WordReg, value: u16) {
        type WR = WordReg;
        let reg = match reg {
            WR::Ax => &mut self.rax,
            WR::Bx => &mut self.rbx,
            WR::Cx => &mut self.rcx,
            WR::Dx => &mut self.rdx,
            WR::Si => &mut self.rsi,
            WR::Di => &mut self.rdi,
            WR::Sp => &mut self.rsp,
            WR::Bp => &mut self.rbp,
            WR::R8w => &mut self.r8,
            WR::R9w => &mut self.r9,
            WR::R10w => &mut self.r10,
            WR::R11w => &mut self.r11,
            WR::R12w => &mut self.r12,
            WR::R13w => &mut self.r13,
            WR::R14w => &mut self.r14,
            WR::R15w => &mut self.r15,
        };
        *reg |= u64::from(value);
    }

    fn write_byte(&mut self, reg: ByteReg, value: u8) {
        type BR = ByteReg;
        let mut upper_byte = false;
        let reg = match reg {
            BR::Al => &mut self.rax,
            BR::Ah => {
                upper_byte = true;
                &mut self.rax
            }
            BR::Bl => &mut self.rbx,
            BR::Bh => {
                upper_byte = true;
                &mut self.rbx
            }
            BR::Cl => &mut self.rcx,
            BR::Ch => {
                upper_byte = true;
                &mut self.rcx
            }
            BR::Dl => &mut self.rdx,
            BR::Dh => {
                upper_byte = true;
                &mut self.rdx
            }
            BR::Sil => &mut self.rsi,
            BR::Dil => &mut self.rdi,
            BR::Spl => &mut self.rsp,
            BR::Bpl => &mut self.rbp,
            BR::R8b => &mut self.r8,
            BR::R9b => &mut self.r9,
            BR::R10b => &mut self.r10,
            BR::R11b => &mut self.r11,
            BR::R12b => &mut self.r12,
            BR::R13b => &mut self.r13,
            BR::R14b => &mut self.r14,
            BR::R15b => &mut self.r15,
        };
        *reg |= if upper_byte {
            u64::from(value) << 8
        } else {
            u64::from(value)
        };
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
