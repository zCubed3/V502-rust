pub mod internal;
pub mod vm;

#[cfg(test)]
mod tests {
    use crate::internal;
    use crate::internal::consts;
    use crate::internal::types;

    use crate::vm;

    use std::ptr;
    use crate::internal::consts::BINARY_SIZE;
    use crate::vm::CPUModel;

    #[test]
    fn test_asm() {

    }

    #[test]
    fn test_vm_raw() {
        unsafe {
            let createinfo = internal::vm::VMCreateInfo { hunk_size: consts::BINARY_SIZE, feature_set: types::Featureset::MOS6502 };

            let vm_instance = internal::vm::v502_create_vm(&createinfo);
            assert_ne!(vm_instance, ptr::null_mut());

            // SUPER unsafe but loads a program manually
            //*(*vm_instance).hunk.offset(consts::RESET_VECTOR as isize) = 0x00;
            //*(*vm_instance).hunk.offset(consts::RESET_VECTOR as isize + 1) = 0x40;

            internal::vm::v502_reset_vm(vm_instance);

            internal::vm::v502_cycle_vm(vm_instance);

            internal::vm::v502_free_vm(vm_instance);
        }
    }

    #[test]
    fn test_vm_safe() {
        let mut vm = vm::V502VM::builder()
            .model(CPUModel::MOS6502)
            .hunk_size(BINARY_SIZE)
            .build();

        vm.simulation_reset();
        vm.simulate();
    }
}