pub mod types;
pub mod consts;
pub mod vm;
pub mod vcpu;

#[cfg(test)]
mod tests {
    use crate::*;
    use std::ptr;

    #[test]
    fn test_vm() {
        unsafe {
            let createinfo = vm::VMCreateInfo { hunk_size: consts::BINARY_SIZE, feature_set: types::Featureset::MOS6502 };

            let vm_instance = vm::v502_create_vm(&createinfo);
            
            assert_ne!(vm_instance, ptr::null_mut());
        }
    }
}