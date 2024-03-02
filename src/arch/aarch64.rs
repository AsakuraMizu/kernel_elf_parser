//! Relocate .rela.dyn sections
//! R_TYPE 与处理器架构有关，相关文档详见
//! aarch: unimplemented

extern crate alloc;
use super::RelocatePair;
use alloc::vec::Vec;

/// To parse the elf file and get the relocate pairs
///
/// # Arguments
///
/// * `elf` - The elf file
/// * `elf_base_addr` - The base address of the elf file if the file will be loaded to the memory
pub fn get_relocate_pairs(
    _elf: &xmas_elf::ElfFile,
    _elf_base_addr: Option<usize>,
) -> Vec<RelocatePair> {
    panic!("Unimplemented");
}
