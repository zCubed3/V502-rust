use crate::types::*;

#[repr(C)]
struct VM {
    vcpu : VCPU,

    hunk : *Byte,
    hunk_length : UInt
}

extern "C" {

}