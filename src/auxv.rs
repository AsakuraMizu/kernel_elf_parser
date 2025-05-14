use core::fmt;

use alloc::collections::btree_map::BTreeMap;

/// Represents the type of an auxiliary vector entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(non_camel_case_types)]
#[repr(usize)]
pub enum AuxType {
    /// End of vector
    NULL = 0,
    /// Entry should be ignored
    IGNORE = 1,
    /// File descriptor of program
    EXECFD = 2,
    /// Program headers for program
    PHDR = 3,
    /// Size of program header entry
    PHENT = 4,
    /// Number of program headers
    PHNUM = 5,
    /// System page size
    PAGESZ = 6,
    /// Base address of interpreter
    BASE = 7,
    /// Flags
    FLAGS = 8,
    /// Entry point of program
    ENTRY = 9,
    /// Program is not ELF
    NOTELF = 10,
    /// Real UID
    UID = 11,
    /// Effective UID
    EUID = 12,
    /// Real GID
    GID = 13,
    /// Effective GID
    EGID = 14,
    /// String identifying CPU for optimizations
    PLATFORM = 15,
    /// Arch dependent hints at CPU capabilities
    HWCAP = 16,
    /// Frequency at which times() increments
    CLKTCK = 17,
    /// Floating point unit control word
    FPUCW = 18,
    /// Data cache block size
    DCACHEBSIZE = 19,
    /// Instruction cache block size
    ICACHEBSIZE = 20,
    /// Unified cache block size
    UCACHEBSIZE = 21,
    /// Entry should be ignored on PowerPC
    IGNOREPPC = 22,
    /// Secure mode boolean
    SECURE = 23,
    /// String identifying real platform, may differ from AT_PLATFORM
    BASE_PLATFORM = 24,
    /// Address of 16 random bytes
    RANDOM = 25,
    /// Extension of AT_HWCAP
    HWCAP2 = 26,
    /// Filename of program
    EXECFN = 31,
    /// Address of the VDSO
    SYSINFO = 32,
    /// Address of the ELF header of the VDSO
    SYSINFO_EHDR = 33,
    /// Shape of level 1 instruction cache
    L1I_CACHESHAPE = 34,
    /// Shape of level 1 data cache
    L1D_CACHESHAPE = 35,
    /// Shape of level 2 cache
    L2_CACHESHAPE = 36,
    /// Shape of level 3 cache
    L3_CACHESHAPE = 37,
    /// Size of level 1 instruction cache
    L1I_CACHESIZE = 40,
    /// Geometry of level 1 instruction cache
    L1I_CACHEGEOMETRY = 41,
    /// Size of level 1 data cache
    L1D_CACHESIZE = 42,
    /// Geometry of level 1 data cache
    L1D_CACHEGEOMETRY = 43,
    /// Size of level 2 cache
    L2_CACHESIZE = 44,
    /// Geometry of level 2 cache
    L2_CACHEGEOMETRY = 45,
    /// Size of level 3 cache
    L3_CACHESIZE = 46,
    /// Geometry of level 3 cache
    L3_CACHEGEOMETRY = 47,
    /// Minimal stack size for signal delivery
    MINSIGSTKSZ = 51,
}

#[derive(Clone, Default)]
pub struct AuxVec(BTreeMap<AuxType, usize>);

impl fmt::Debug for AuxVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.0.iter()).finish()
    }
}

impl AuxVec {
    pub const fn new() -> Self {
        AuxVec(BTreeMap::new())
    }

    pub fn set(&mut self, ty: AuxType, val: usize) {
        if matches!(ty, AuxType::NULL | AuxType::IGNORE) {
            return;
        }
        self.0.insert(ty, val);
    }
}
