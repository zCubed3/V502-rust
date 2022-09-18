use crate::types::*;
use crate::vcpu::VCPU;

#[repr(C)]
pub struct VMCreateInfo {
    pub hunk_size : u32,
    pub feature_set : Featureset
}

#[repr(C)]
pub struct VM {
    pub vcpu : *mut VCPU,

    pub hunk : *mut Byte,
    pub hunk_length : UInt
}

extern "C" {
    pub fn v502_create_vm(createinfo : *const VMCreateInfo) -> *mut VM;
}