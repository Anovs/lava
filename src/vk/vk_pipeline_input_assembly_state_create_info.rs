// Generated by `scripts/generate_vk.js`

use std::os::raw::c_char;
use std::ops::Deref;
use std::ptr;
use std::cmp;
use std::mem;
use utils::c_bindings::*;
use utils::vk_convert::*;
use utils::vk_null::*;
use utils::vk_ptr::*;
use utils::vk_traits::*;
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::vk_pipeline_input_assembly_state_create_flags::*;
use vk::vk_primitive_topology::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPipelineInputAssemblyStateCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkPipelineInputAssemblyStateCreateFlags,
    pub topology: RawVkPrimitiveTopology,
    pub primitive_restart_enable: u32,
}

#[derive(Debug, Clone)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub flags: VkPipelineInputAssemblyStateCreateFlags,
    pub topology: VkPrimitiveTopology,
    pub primitive_restart_enable: bool,
}

impl VkRawType<VkPipelineInputAssemblyStateCreateInfo> for RawVkPipelineInputAssemblyStateCreateInfo {
    fn vk_to_wrapped(src: &RawVkPipelineInputAssemblyStateCreateInfo) -> VkPipelineInputAssemblyStateCreateInfo {
        VkPipelineInputAssemblyStateCreateInfo {
            flags: RawVkPipelineInputAssemblyStateCreateFlags::vk_to_wrapped(&src.flags),
            topology: RawVkPrimitiveTopology::vk_to_wrapped(&src.topology),
            primitive_restart_enable: u32::vk_to_wrapped(&src.primitive_restart_enable),
        }
    }
}

impl VkWrappedType<RawVkPipelineInputAssemblyStateCreateInfo> for VkPipelineInputAssemblyStateCreateInfo {
    fn vk_to_raw(src: &VkPipelineInputAssemblyStateCreateInfo, dst: &mut RawVkPipelineInputAssemblyStateCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PipelineInputAssemblyStateCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.topology = vk_to_raw_value(&src.topology);
        dst.primitive_restart_enable = vk_to_raw_value(&src.primitive_restart_enable);
    }
}

impl Default for VkPipelineInputAssemblyStateCreateInfo {
    fn default() -> VkPipelineInputAssemblyStateCreateInfo {
        VkPipelineInputAssemblyStateCreateInfo {
            flags: VkPipelineInputAssemblyStateCreateFlags::default(),
            topology: VkPrimitiveTopology::default(),
            primitive_restart_enable: false,
        }
    }
}

impl VkSetup for VkPipelineInputAssemblyStateCreateInfo {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPipelineInputAssemblyStateCreateInfo {
    fn vk_free(&mut self) {
        
    }
}