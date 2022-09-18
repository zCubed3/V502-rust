// Only typedef'ed here because in C it's typedef'ed

pub type Byte = u8;
pub type Char = i8;

pub type UShort = u16;
pub type Short = i16;

pub type UInt = u32;
pub type Int = i32;

pub type FileLen = UInt;
pub type VPRGIdent = UInt;

#[repr(C)]
pub enum Featureset {
    MOS6502,
    W65C02S
}