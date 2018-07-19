// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;
use glfw::*;

pub type RawVkSurface = RawVkHandle;

#[derive(Debug)]
pub struct VkSurface {
    _handle: RawVkSurface,
    _instance: RawVkInstance,
}

impl VkSurface {
    
    pub fn handle(&self) -> RawVkSurface {
        self._handle
    }
    
    pub fn from_glfw(instance: &VkInstance, glfw_window: &GlfwWindow) -> Result<VkSurface, VkResult> {
        unsafe {
            let instance_handle = instance.handle();
            vk_call_retrieve_single(
                |ptr| glfwCreateWindowSurface(instance_handle, glfw_window.handle(), null(), ptr),
                |surface : &mut VkSurface| { surface._instance = instance_handle; }
            )
        }
    }
}

impl VkFrom<VkSurface> for RawVkSurface {
    
    fn vk_from(value: &VkSurface) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkSurface> for VkSurface {
    
    fn vk_from(value: &RawVkSurface) -> Self {
        Self {
            _handle: *value,
            _instance: VK_NULL_HANDLE,
        }
    }
}

impl Drop for VkSurface {
    
    fn drop(&mut self) {
        unsafe {
            vkDestroySurfaceKHR(self._instance, self._handle, null());
        }
    }
}

extern {
    fn vkDestroySurfaceKHR(instance: RawVkInstance, surface: RawVkSurface, p_allocator: *const c_void);
    fn glfwCreateWindowSurface(instance: RawVkInstance, window: *mut RawGlfwWindow, allocator: *const c_void, surface: *mut RawVkSurface)-> RawVkResult;
}