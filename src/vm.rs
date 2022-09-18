use crate::internal::vm::{v502_free_vm, VM as CVM};
use crate::internal::vm;
use crate::internal::types::*;
use crate::internal::consts::*;

pub use crate::internal::types::Featureset as CPUModel;

// Reference: https://rust-unofficial.github.io/patterns/patterns/creational/builder.html

pub struct V502VM {
    raw_vm : Option<*mut CVM>
}

impl V502VM {
    pub fn builder() -> V502VMBuilder {
        V502VMBuilder::default()
    }

    pub fn new(size : UInt, model : CPUModel) -> V502VM {
        let mut vm = V502VM { raw_vm: None };

        unsafe {
            let create_info = vm::VMCreateInfo { feature_set: model, hunk_size: size };
            vm.raw_vm = Some(vm::v502_create_vm(&create_info));
        }

        vm
    }

    unsafe fn get_cvm_ptr(&mut self) -> Option<*mut CVM> {
        return match self.raw_vm.as_mut() {
            Some(x) =>  Some(*x),
            None => None
        };
    }

    pub fn simulation_reset(&mut self) {
        unsafe {
            let cvm = self.get_cvm_ptr().expect("CVM was invalid!");
            vm::v502_reset_vm(cvm);
        }
    }

    pub fn simulate(&mut self) {
        unsafe {
            let cvm = self.get_cvm_ptr().expect("CVM was invalid!");
            vm::v502_cycle_vm(cvm);
        }
    }
}

impl Drop for V502VM {
    fn drop(&mut self) {
        unsafe {
            let cvm = self.raw_vm.unwrap();
            v502_free_vm(cvm);
        }
    }
}

pub struct V502VMBuilder {
    size : UInt,
    model : CPUModel
}

impl V502VMBuilder {
    pub fn model(mut self, model : CPUModel) -> Self {
        self.model = model;
        self
    }

    pub fn hunk_size(mut self, size : UInt) -> Self {
        self.size = size;
        self
    }

    pub fn build(self) -> V502VM {
        V502VM::new(self.size, self.model)
    }
}

impl Default for V502VMBuilder {
    fn default() -> Self {
        V502VMBuilder { size: BINARY_SIZE, model: CPUModel::MOS6502 }
    }
}