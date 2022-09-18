use libc::c_char;

use crate::internal::types::*;
use crate::internal::cpu::vcpu::VCPU;

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

#[repr(C)]
pub struct LoadProgramOptions {
    pub raw_origin : UShort
}

extern "C" {
    pub fn v502_create_vm(createinfo : *const VMCreateInfo) -> *mut VM;
    pub fn v502_free_vm(vm : *mut VM);
    pub fn v502_reset_vm(vm : *mut VM);
    pub fn v502_cycle_vm(vm : *mut VM) -> bool;

    pub fn v502_default_load_options() -> LoadProgramOptions;
    pub fn v502_load_vprg_vm(vm : *mut VM, bytes : *mut Byte);
    pub fn v502_load_raw_vm(vm : *mut VM, origin : UShort, bytes : *mut Byte, len : FileLen);
    pub fn v502_load_program_file_vm(vm : *mut VM, path : *const c_char, load_options : *const LoadProgramOptions);

    pub fn v502_make_word(a : Byte, b : Byte) -> UShort;
}