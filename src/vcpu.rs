use crate::types::*;
use crate::vm::VM;

#[repr(C)]
pub enum OPState {
    Failed,
    Success,
    SuccessNoCount
}

pub type OPFunc = unsafe extern "C" fn(*mut VM, Byte) -> OPState;

#[repr(C)]
pub struct VCPU {
    pub program_counter : UShort,

    pub stack_ptr : Byte,
    pub accumulator : Byte,
    pub index_x : Byte,
    pub index_y : Byte,
    pub flags : Byte,

    pub v502_opfunc_t : *mut OPFunc,
    pub feature_set : Featureset
}

extern "C" {

}