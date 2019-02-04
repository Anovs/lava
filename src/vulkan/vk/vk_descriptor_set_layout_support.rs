// Generated by `scripts/generate.js`

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
use vulkan::vk::*;
use vulkan::vk::{VkStructureType,RawVkStructureType};

/// Wrapper for [VkDescriptorSetLayoutSupport](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDescriptorSetLayoutSupport.html).
#[derive(Debug, Clone)]
pub struct VkDescriptorSetLayoutSupport {
    pub supported: bool,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkDescriptorSetLayoutSupport {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub supported: u32,
}

impl VkWrappedType<RawVkDescriptorSetLayoutSupport> for VkDescriptorSetLayoutSupport {
    fn vk_to_raw(src: &VkDescriptorSetLayoutSupport, dst: &mut RawVkDescriptorSetLayoutSupport) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DescriptorSetLayoutSupport);
        dst.next = ptr::null();
        dst.supported = vk_to_raw_value(&src.supported);
    }
}

impl VkRawType<VkDescriptorSetLayoutSupport> for RawVkDescriptorSetLayoutSupport {
    fn vk_to_wrapped(src: &RawVkDescriptorSetLayoutSupport) -> VkDescriptorSetLayoutSupport {
        VkDescriptorSetLayoutSupport {
            supported: u32::vk_to_wrapped(&src.supported),
        }
    }
}

impl Default for VkDescriptorSetLayoutSupport {
    fn default() -> VkDescriptorSetLayoutSupport {
        VkDescriptorSetLayoutSupport {
            supported: false,
        }
    }
}

impl VkSetup for VkDescriptorSetLayoutSupport {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorSetLayoutSupport {
    fn vk_free(&mut self) {
        
    }
}