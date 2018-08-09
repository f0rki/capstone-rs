//! Contains EVM-specific types

//pub use arch::arch_builder::arm::*;
//use arch::DetailsArchInsn;
//use capstone_sys::{arm_op_mem, arm_op_type, cs_arm, cs_arm_op};
//use std::convert::From;
//use std::os::raw::c_uint;
//use std::{cmp, fmt, slice};
//
use capstone_sys::cs_evm;

pub use capstone_sys::evm_insn as EVMInsn;
pub use capstone_sys::evm_insn_group as EVMInsnGroup;

/// Contains ARM-specific details for an instruction
pub struct EVMInsnDetail<'a>(pub(crate) &'a cs_evm);

impl<'a> EVMInsnDetail<'a> {
    //...
}

//impl_PartialEq_repr_fields!(ArmInsnDetail<'a> [ 'a ];
//    usermode, vector_size, vector_data, cps_mode, cps_flag, cc, update_flags, writeback,
//    mem_barrier, operands
//);

//impl_PartialEq_repr_fields!(ArmOpMem;
//    base, index, scale, disp
//);

//def_arch_details_struct!(

//);

#[cfg(test)]
mod test {
    use super::*;
    use capstone_sys::*;

    #[test]
    fn test_basic_evm() {}

}
