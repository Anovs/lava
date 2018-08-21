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
use vk::vk_access_flags::*;

#[repr(C)]
pub struct RawVkMemoryBarrier {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub src_access_mask: RawVkAccessFlags,
    pub dst_access_mask: RawVkAccessFlags,
}

#[derive(Debug, Clone)]
pub struct VkMemoryBarrier {
    pub src_access_mask: VkAccessFlags,
    pub dst_access_mask: VkAccessFlags,
}

impl VkRawType<VkMemoryBarrier> for RawVkMemoryBarrier {
    fn vk_to_wrapped(src: &RawVkMemoryBarrier) -> VkMemoryBarrier {
        VkMemoryBarrier {
            src_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.src_access_mask),
            dst_access_mask: RawVkAccessFlags::vk_to_wrapped(&src.dst_access_mask),
        }
    }
}

impl VkWrappedType<RawVkMemoryBarrier> for VkMemoryBarrier {
    fn vk_to_raw(src: &VkMemoryBarrier, dst: &mut RawVkMemoryBarrier) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MemoryBarrier);
        dst.next = ptr::null();
        dst.src_access_mask = vk_to_raw_value(&src.src_access_mask);
        dst.dst_access_mask = vk_to_raw_value(&src.dst_access_mask);
    }
}

impl Default for VkMemoryBarrier {
    fn default() -> VkMemoryBarrier {
        VkMemoryBarrier {
            src_access_mask: VkAccessFlags::default(),
            dst_access_mask: VkAccessFlags::default(),
        }
    }
}

impl VkSetup for VkMemoryBarrier {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkMemoryBarrier {
    fn vk_free(&mut self) {
        
    }
}