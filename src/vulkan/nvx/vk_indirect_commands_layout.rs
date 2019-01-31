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
pub type RawVkIndirectCommandsLayout = u64;

/// Wrapper for [VkIndirectCommandsLayoutNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkIndirectCommandsLayoutNVX.html)
#[derive(Debug, Clone)]
pub struct VkIndirectCommandsLayout {
    _handle: RawVkIndirectCommandsLayout,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkIndirectCommandsLayout> for RawVkIndirectCommandsLayout {
    fn vk_to_wrapped(src: &RawVkIndirectCommandsLayout) -> VkIndirectCommandsLayout {
        VkIndirectCommandsLayout {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkIndirectCommandsLayout> for VkIndirectCommandsLayout {
    fn vk_to_raw(src: &VkIndirectCommandsLayout, dst: &mut RawVkIndirectCommandsLayout) {
        *dst = src._handle
    }
}

impl Default for VkIndirectCommandsLayout {
    fn default() -> VkIndirectCommandsLayout {
        VkIndirectCommandsLayout {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkIndirectCommandsLayout {
    fn eq(&self, other: &VkIndirectCommandsLayout) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkIndirectCommandsLayout {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkIndirectCommandsLayout {
    
    /// Returns the internal Vulkan handle for the object.
    pub fn vk_handle(&self) -> u64 {
        self._handle
    }
    
    /// Wrapper for [vkDestroyIndirectCommandsLayoutNVX](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/vkDestroyIndirectCommandsLayoutNVX.html)
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyIndirectCommandsLayoutNVX)(self._parent_device, self._handle, ptr::null());
        }
    }
}