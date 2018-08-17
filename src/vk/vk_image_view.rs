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

pub type RawVkImageView = u64;

#[derive(Debug, Copy, Clone)]
pub struct VkImageView {
    _handle: RawVkImageView,
    _parent_instance: RawVkInstance,
    _parent_device: RawVkDevice,
    _fn_table: *mut VkInstanceFunctionTable
}

impl VkRawType<VkImageView> for RawVkImageView {
    fn vk_to_wrapped(src: &RawVkImageView) -> VkImageView {
        VkImageView {
            _handle: *src,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkWrappedType<RawVkImageView> for VkImageView {
    fn vk_to_raw(src: &VkImageView, dst: &mut RawVkImageView) {
        *dst = src._handle
    }
}

impl Default for VkImageView {
    fn default() -> VkImageView {
        VkImageView {
            _handle: 0,
            _parent_instance: 0,
            _parent_device: 0,
            _fn_table: ptr::null_mut()
        }
    }
}

impl VkSetup for VkImageView {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        self._parent_instance = instance;
        self._parent_device = device;
        self._fn_table = fn_table;
    }
}

impl VkImageView {
    
    pub fn handle(&self) -> u64 {
        self._handle
    }
    
    pub fn destroy(&self) {
        unsafe {
            ((&*self._fn_table).vkDestroyImageView)(self._parent_device, self._handle, ptr::null());
        }
    }
}