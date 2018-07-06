// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;

pub type RawVkBuffer = RawVkHandle;

#[derive(Debug)]
pub struct VkBuffer {
    _handle: RawVkBuffer,
    _device: RawVkDevice,
}

impl VkBuffer {
    
    pub fn handle(&self) -> RawVkBuffer {
        self._handle
    }
    
    pub fn new(device: &VkDevice, create_info: &VkBufferCreateInfo) -> Result<VkBuffer, VkResult> {
        unsafe {
            let device_handle = device.handle();
            let mut raw_create_info = RawVkBufferCreateInfo::vk_from(create_info);
            let raw_create_info_ptr = &mut raw_create_info as *mut RawVkBufferCreateInfo;
            vk_call_retrieve_single(
                |ptr| vkCreateBuffer(device_handle, raw_create_info_ptr, null(), ptr),
                |buffer : &mut VkBuffer| { buffer._device = device_handle; }
            )
        }
    }
}

impl VkFrom<VkBuffer> for RawVkBuffer {
    
    fn vk_from(value: &VkBuffer) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkBuffer> for VkBuffer {
    
    fn vk_from(value: &RawVkBuffer) -> Self {
        Self {
            _handle: *value,
            _device: VK_NULL_HANDLE,
        }
    }
}

impl Drop for VkBuffer {
    
    fn drop(&mut self) {
        unsafe {
            vkDestroyBuffer(self._device, self._handle, null());
        }
    }
}

extern {
    fn vkDestroyBuffer(device: RawVkDevice, buffer: RawVkBuffer, p_allocator: *const c_void);
    fn vkCreateBuffer(device: RawVkDevice, p_create_info: *const RawVkBufferCreateInfo, p_allocator: *const c_void, p_buffer: *mut RawVkBuffer)-> RawVkResult;
}