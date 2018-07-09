// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ops::Drop;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;
use glfw::*;
use std::mem::ManuallyDrop;

pub type RawVkInstance = RawVkHandle;

#[derive(Debug)]
pub struct VkInstance {
    _handle: RawVkInstance,
    _debug_report_callback_ext_list: ManuallyDrop<Vec<VkDebugReportCallbackEXT>>,
}

impl VkInstance {
    
    pub fn handle(&self) -> RawVkInstance {
        self._handle
    }
    
    pub fn new(create_info: &VkInstanceCreateInfo) -> Result<VkInstance, VkResult> {
        unsafe {
            let mut raw_create_info = RawVkInstanceCreateInfo::vk_from(create_info);
            let raw_create_info_ptr = &mut raw_create_info as *mut RawVkInstanceCreateInfo;
            vk_call_retrieve_single(
                |ptr| vkCreateInstance(raw_create_info_ptr, null(), ptr),
                |instance : &mut VkInstance| {  }
            )
        }
    }
    
    pub fn get_supported_extensions(&self) -> Result<Vec<VkExtensionProperties>, VkResult> {
        unsafe {
            vk_call_retrieve_list(
                |count, ptr| vkEnumerateInstanceExtensionProperties(null(), count, ptr),
                |extension_properties : &mut VkExtensionProperties| {  }
            )
        }
    }
    
    pub fn get_physical_devices(&self) -> Result<Vec<VkPhysicalDevice>, VkResult> {
        VkPhysicalDevice::get_list(self)
    }
    
    pub fn create_surface_from_glfw(&self, glfw_window: &GlfwWindow) -> Result<VkSurfaceKHR, VkResult> {
        VkSurfaceKHR::from_glfw(self, glfw_window)
    }
    
    pub fn get_layer_properties(&self) -> Result<Vec<VkLayerProperties>, VkResult> {
        unsafe {
            vk_call_retrieve_list(
                |count, ptr| vkEnumerateInstanceLayerProperties(count, ptr),
                |layer_properties : &mut VkLayerProperties| {  }
            )
        }
    }
    
    pub fn create_debug_callback(&mut self, create_info: &VkDebugReportCallbackCreateInfo) -> Result<(), VkResult> {
        let result = VkDebugReportCallbackEXT::new(self, create_info);
        match result {
            Ok(value) =>  {
                self._debug_report_callback_ext_list.push(value);
                Ok(())
            }
            Err(error) => Err(error)
        }
    }
}

impl VkFrom<VkInstance> for RawVkInstance {
    
    fn vk_from(value: &VkInstance) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkInstance> for VkInstance {
    
    fn vk_from(value: &RawVkInstance) -> Self {
        Self {
            _handle: *value,
            _debug_report_callback_ext_list: ManuallyDrop::new(Vec::new()),
        }
    }
}

impl Drop for VkInstance {
    
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self._debug_report_callback_ext_list);
            vkDestroyInstance(self._handle, null());
        }
    }
}

extern {
    fn vkDestroyInstance(instance: RawVkInstance, p_allocator: *const c_void);
    fn vkCreateInstance(p_create_info: *const RawVkInstanceCreateInfo, p_allocator: *const c_void, p_instance: *mut RawVkInstance)-> RawVkResult;
    fn vkEnumerateInstanceExtensionProperties(p_layer_name: *const c_char, p_property_count: *mut u32, p_properties: *mut RawVkExtensionProperties)-> RawVkResult;
    fn vkEnumerateInstanceLayerProperties(p_property_count: *mut u32, p_properties: *mut RawVkLayerProperties)-> RawVkResult;
}