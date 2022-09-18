use crate::internal::types::*;
use crate::internal::vm::VM;

#[repr(C)]
pub enum OPState {
    Failed,
    Success,
    SuccessNoCount
}

pub type OPFunc = unsafe extern "C" fn(*mut VM, Byte) -> OPState;

#[repr(C)]
pub enum StateFlags {
    FlagCarry = 1,
    FlagZero = 2,
    FlagInterrupt = 4,
    FlagDecimal = 8,
    FlagBreak = 16,
    FlagUnused = 32,
    FlagOverflow = 64,
    FlagNegative = 128
}

#[repr(C)]
pub struct VCPU {
    pub program_counter : UShort,

    pub stack_ptr : Byte,
    pub accumulator : Byte,
    pub index_x : Byte,
    pub index_y : Byte,
    pub flags : Byte,

    pub opfuncs : *mut OPFunc,
    pub feature_set : Featureset
}

extern "C" {
    pub fn v502_get_fallback_func() -> OPFunc;

    pub fn v502_create_vcpu(feature_set : Featureset) -> *mut VCPU;
    pub fn v502_free_vcpu(vcpu : *mut VCPU);
    pub fn v502_reset_vcpu(vcpu : *mut VCPU, org : UShort);

    pub fn v502_safe_add_vcpu(vcpu : *mut VCPU, val : Byte);
    pub fn v502_safe_sub_vcpu(vcpu : *mut VCPU, val : Byte);
    pub fn v502_compare_vcpu(vcpu : *mut VCPU, lhs : Byte, rhs : Byte);
}