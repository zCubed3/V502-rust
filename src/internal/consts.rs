use super::types::*;

pub const BINARY_SIZE : UInt = 64 * 1024; // 64 KiB

pub const NMI_VECTOR : UShort = 0xFFFA;
pub const RESET_VECTOR : UShort = 0xFFFC;
pub const IRQ_VECTOR : UShort = 0xFFFE;
