pub mod internal;

#[cfg(test)]
mod tests {
    use crate::internal::*;
    use std::ptr;

    #[test]
    fn test_asm() {

    }

    #[test]
    fn test_vm() {
        unsafe {
            let createinfo = vm::VMCreateInfo { hunk_size: consts::BINARY_SIZE, feature_set: types::Featureset::MOS6502 };

            let vm_instance = vm::v502_create_vm(&createinfo);
            assert_ne!(vm_instance, ptr::null_mut());

            // SUPER unsafe but loads a program manually
            //*(*vm_instance).hunk.offset(consts::RESET_VECTOR as isize) = 0x00;
            //*(*vm_instance).hunk.offset(consts::RESET_VECTOR as isize + 1) = 0x40;

            vm::v502_reset_vm(vm_instance);

            vm::v502_cycle_vm(vm_instance);

            vm::v502_free_vm(vm_instance);
        }
    }
}