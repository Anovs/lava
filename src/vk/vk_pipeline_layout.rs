// Generated by `scripts/generate_vk.js`

use utils::c_bindings::*;
use utils::vk_traits::*;
use utils::vk_ptr::*;
use utils::vk_convert::*;
use std::os::raw::c_char;
use std::ptr;
use std::mem;
use std::cmp;
use vk::*;

pub type RawVkPipelineLayout = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkPipelineLayout {
    _handle: RawVkPipelineLayout,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkPipelineLayout> for RawVkPipelineLayout {
    fn vk_to_wrapped(src: &RawVkPipelineLayout) -> VkPipelineLayout {
        VkPipelineLayout {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkPipelineLayout> for VkPipelineLayout {
    fn vk_to_raw(src: &VkPipelineLayout, dst: &mut RawVkPipelineLayout) {
        *dst = src._handle
    }
}

impl Default for VkPipelineLayout {
    fn default() -> VkPipelineLayout {
        VkPipelineLayout {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl PartialEq for VkPipelineLayout {
    fn eq(&self, other: &VkPipelineLayout) -> bool {
        self._handle == other._handle
    }
}

impl VkSetup for VkPipelineLayout {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkPipelineLayout {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyPipelineLayout)(self._parent_device, self._handle, ptr::null());
        }
    }
}