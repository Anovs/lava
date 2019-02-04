// Generated by `scripts/generate.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::ptr;
use std::mem;
use std::cmp;
use std::slice;
use vulkan::*;
use vulkan::vk::*;

#[doc(hidden)]
pub type RawVkDisplay = u64;

/// Wrapper for [VkDisplayKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkDisplayKHR.html).
#[derive(Debug, Clone)]
pub struct VkDisplay {
    _handle: RawVkDisplay,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkDisplay> for RawVkDisplay {
    fn vk_to_wrapped(src: &RawVkDisplay) -> VkDisplay {
        VkDisplay {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkDisplay> for VkDisplay {
    fn vk_to_raw(src: &VkDisplay, dst: &mut RawVkDisplay) {
        *dst = src._handle
    }
}

impl Default for VkDisplay {
    fn default() -> VkDisplay {
        VkDisplay {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkDisplay {
    fn eq(&self, other: &VkDisplay) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkDisplay {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkDisplay {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
}