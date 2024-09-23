//! Definitions of interfaces for architectural registers.

use core::mem::MaybeUninit;

pub mod control;
pub mod msr;

#[repr(C)]
pub struct Idtr {
    _padding: [MaybeUninit<u8>; 6],
    limit: u16,
    address: u64,
}

impl Idtr {
    pub fn get() -> Self {
        let mut reg = Self::new(0, 0);

        unsafe { core::arch::asm!("sidt [{}]", in(reg) &mut reg) }

        reg
    }

    pub fn new(address: u64, limit: u16) -> Self {
        Self {
            _padding: [MaybeUninit::uninit(); 6],
            limit,
            address,
        }
    }

    pub fn limit(&self) -> u16 {
        self.limit
    }

    pub fn address(&self) -> u64 {
        self.address
    }
}

#[repr(C)]
pub struct Gdtr {
    _padding: [MaybeUninit<u8>; 6],
    limit: u16,
    address: u64,
}

impl Gdtr {
    pub fn get() -> Self {
        let mut reg = Self::new(0, 0);

        unsafe { core::arch::asm!("sgdt [{}]", in(reg) &mut reg) }

        reg
    }

    pub fn new(address: u64, limit: u16) -> Self {
        Self {
            _padding: [MaybeUninit::uninit(); 6],
            limit,
            address,
        }
    }

    pub fn limit(&self) -> u16 {
        self.limit
    }

    pub fn address(&self) -> u64 {
        self.address
    }
}
