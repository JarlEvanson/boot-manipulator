//! Definitions of `x86_64` control registers.

use core::fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Cr0(u64);

impl Cr0 {
    pub fn get() -> Self {
        let cr0: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, cr0",
                out(reg) cr0
            )
        }
        Self(cr0)
    }

    pub fn pe(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn mp(&self) -> bool {
        self.0 & (1 << 1) == (1 << 1)
    }

    pub fn em(&self) -> bool {
        self.0 & (1 << 2) == (1 << 2)
    }

    pub fn ts(&self) -> bool {
        self.0 & (1 << 3) == (1 << 3)
    }

    pub fn et(&self) -> bool {
        self.0 & (1 << 4) == (1 << 4)
    }

    pub fn ne(&self) -> bool {
        self.0 & (1 << 5) == (1 << 5)
    }

    pub fn wp(&self) -> bool {
        self.0 & (1 << 16) == (1 << 16)
    }

    pub fn am(&self) -> bool {
        self.0 & (1 << 18) == (1 << 18)
    }

    pub fn nw(&self) -> bool {
        self.0 & (1 << 29) == (1 << 29)
    }

    pub fn cd(&self) -> bool {
        self.0 & (1 << 30) == (1 << 30)
    }

    pub fn pg(&self) -> bool {
        self.0 & (1 << 31) == (1 << 31)
    }
}

impl fmt::Display for Cr0 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Cr0Display(self.0).fmt(f)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct Cr0Display(pub u64);

impl fmt::Display for Cr0Display {
    #[allow(unused_assignments)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cr0 = Cr0(self.0);
        let mut prev = false;

        macro_rules! flag {
            ($flag_enabled:expr, $name:expr) => {
                if $flag_enabled {
                    if prev {
                        write!(f, " | ")?;
                    }
                    write!(f, $name)?;
                    prev = true;
                }
            };
        }

        flag!(cr0.pe(), "PE");
        flag!(cr0.mp(), "MP");
        flag!(cr0.em(), "EM");
        flag!(cr0.ts(), "TS");
        flag!(cr0.et(), "ET");
        flag!(cr0.ne(), "NE");
        flag!(cr0.wp(), "WP");
        flag!(cr0.am(), "AM");
        flag!(cr0.nw(), "NW");
        flag!(cr0.cd(), "CD");
        flag!(cr0.pg(), "PG");

        Ok(())
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Cr2(u64);

impl Cr2 {
    pub fn get() -> Cr2 {
        let cr2: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, cr2",
                out(reg) cr2
            )
        }
        Self(cr2)
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Cr3(u64);

impl Cr3 {
    pub fn get() -> Cr3 {
        let cr3: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, cr3",
                out(reg) cr3
            )
        }
        Self(cr3)
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Cr4(u64);

impl Cr4 {
    pub fn get() -> Cr4 {
        let cr4: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, cr4",
                out(reg) cr4
            )
        }
        Self(cr4)
    }

    pub fn vme(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn pvi(&self) -> bool {
        self.0 & (1 << 1) == (1 << 1)
    }

    pub fn tsd(&self) -> bool {
        self.0 & (1 << 2) == (1 << 2)
    }

    pub fn de(&self) -> bool {
        self.0 & (1 << 3) == (1 << 3)
    }

    pub fn pse(&self) -> bool {
        self.0 & (1 << 4) == (1 << 4)
    }

    pub fn pae(&self) -> bool {
        self.0 & (1 << 5) == (1 << 5)
    }

    pub fn mce(&self) -> bool {
        self.0 & (1 << 6) == (1 << 6)
    }

    pub fn pge(&self) -> bool {
        self.0 & (1 << 7) == (1 << 7)
    }

    pub fn pce(&self) -> bool {
        self.0 & (1 << 8) == (1 << 8)
    }

    pub fn osfxsr(&self) -> bool {
        self.0 & (1 << 9) == (1 << 9)
    }

    pub fn osxmmexcpt(&self) -> bool {
        self.0 & (1 << 10) == (1 << 10)
    }

    pub fn umip(&self) -> bool {
        self.0 & (1 << 11) == (1 << 11)
    }

    pub fn la57(&self) -> bool {
        self.0 & (1 << 12) == (1 << 12)
    }

    pub fn vmxe(&self) -> bool {
        self.0 & (1 << 13) == (1 << 13)
    }

    pub fn smxe(&self) -> bool {
        self.0 & (1 << 14) == (1 << 14)
    }

    pub fn fsgsbase(&self) -> bool {
        self.0 & (1 << 16) == (1 << 16)
    }

    pub fn pcide(&self) -> bool {
        self.0 & (1 << 17) == (1 << 17)
    }

    pub fn osxsave(&self) -> bool {
        self.0 & (1 << 18) == (1 << 18)
    }

    pub fn kl(&self) -> bool {
        self.0 & (1 << 19) == (1 << 19)
    }

    pub fn smep(&self) -> bool {
        self.0 & (1 << 20) == (1 << 20)
    }

    pub fn smap(&self) -> bool {
        self.0 & (1 << 21) == (1 << 21)
    }

    pub fn pke(&self) -> bool {
        self.0 & (1 << 22) == (1 << 22)
    }

    pub fn cet(&self) -> bool {
        self.0 & (1 << 23) == (1 << 23)
    }

    pub fn pks(&self) -> bool {
        self.0 & (1 << 24) == (1 << 24)
    }

    pub fn uintr(&self) -> bool {
        self.0 & (1 << 25) == (1 << 25)
    }
}

impl fmt::Display for Cr4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Cr4Display(self.0).fmt(f)
    }
}

#[derive(Clone, Copy, Default, Hash, PartialEq, Eq)]
pub struct Cr4Display(pub u64);

impl fmt::Display for Cr4Display {
    #[allow(unused_assignments)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cr4 = Cr4(self.0);
        let mut prev = false;

        macro_rules! flag {
            ($flag_enabled:expr, $name:expr) => {
                if $flag_enabled {
                    if prev {
                        write!(f, " | ")?;
                    }
                    write!(f, $name)?;
                    prev = true;
                }
            };
        }

        flag!(cr4.vme(), "VME");
        flag!(cr4.pvi(), "PVI");
        flag!(cr4.tsd(), "TSD");
        flag!(cr4.de(), "DE");
        flag!(cr4.pse(), "PSE");
        flag!(cr4.pae(), "PAE");
        flag!(cr4.mce(), "MCE");
        flag!(cr4.pge(), "PGE");
        flag!(cr4.pce(), "PCE");
        flag!(cr4.osfxsr(), "OSFXSR");
        flag!(cr4.osxmmexcpt(), "OSXMMEXCPT");
        flag!(cr4.umip(), "UMIP");
        flag!(cr4.la57(), "LA57");
        flag!(cr4.vmxe(), "VMXE");
        flag!(cr4.smxe(), "SMXE");
        flag!(cr4.fsgsbase(), "FSGSBASE");
        flag!(cr4.pcide(), "PCIDE");
        flag!(cr4.osxsave(), "OSXSAVE");
        flag!(cr4.kl(), "KL");
        flag!(cr4.smep(), "SMEP");
        flag!(cr4.smap(), "SMAP");
        flag!(cr4.pke(), "PKE");
        flag!(cr4.cet(), "CET");
        flag!(cr4.pks(), "PKS");
        flag!(cr4.uintr(), "UINTR");

        Ok(())
    }
}
