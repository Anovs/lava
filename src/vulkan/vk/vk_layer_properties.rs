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

/// Wrapper for [VkLayerProperties](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkLayerProperties.html)
#[derive(Debug, Clone)]
pub struct VkLayerProperties {
    pub layer_name: String,
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: String,
}

#[doc(hidden)]
#[repr(C)]
pub struct RawVkLayerProperties {
    pub layer_name: [c_char; 256],
    pub spec_version: u32,
    pub implementation_version: u32,
    pub description: [c_char; 256],
}

impl VkRawType<VkLayerProperties> for RawVkLayerProperties {
    fn vk_to_wrapped(src: &RawVkLayerProperties) -> VkLayerProperties {
        VkLayerProperties {
            layer_name: new_string(&src.layer_name[0] as *const c_char),
            spec_version: src.spec_version,
            implementation_version: src.implementation_version,
            description: new_string(&src.description[0] as *const c_char),
        }
    }
}

impl VkSetup for VkLayerProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkLayerProperties {
    fn vk_free(&mut self) {
        
    }
}