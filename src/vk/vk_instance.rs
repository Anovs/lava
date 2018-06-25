use std::string::String;
use std::ffi::CString;
use std::os::raw::c_char;
use std::*;
use vk::*;
use utils::*;

pub struct VkInstance {
    _handler: VkHandler
}

#[repr(C)]
struct RawVkInstanceCreateInfo {
    s_type: VkStructureType,
    p_next: *const u8,
    flags: u32,
    p_application_info: *const RawVkApplicationInfo,
    enabled_layer_count: u32,
    pp_enabled_layer_names: *const *const c_char,
    enabled_extension_count: u32,
    pp_enabled_extension_names: *const *const c_char,
}

#[repr(C)]
struct RawVkApplicationInfo {
    s_type: VkStructureType,
    p_next: *const u8,
    p_application_name: *const c_char,
    application_version: u32,
    p_engine_name: *const c_char,
    engine_version: u32,
    api_version: u32
}

impl VkInstance {
    pub fn create(create_info: &VkInstanceCreateInfo) -> Result<VkInstance, VkResult> {
        unsafe {
            let app_info = &create_info.application_info;
            let api_version = [app_info.api_version[0], app_info.api_version[1], 0];
            let app_name = CString::new(app_info.application_name).unwrap();
            let engine_name = CString::new(app_info.engine_name).unwrap();

            let raw_app_info = RawVkApplicationInfo {
                s_type: VkStructureType::ApplicationInfo,
                p_next: ptr::null(),
                p_application_name: app_name.as_ptr(),
                application_version: vk_make_version(&app_info.application_version),
                p_engine_name: engine_name.as_ptr(),
                engine_version: vk_make_version(&app_info.engine_version),
                api_version: vk_make_version(&api_version)
            };

            let enabled_layers = CharPP::new(&create_info.enabled_layers);
            let enabled_extensions = CharPP::new(&create_info.enabled_extensions);

            let raw_instance_create_info = RawVkInstanceCreateInfo {
                s_type: VkStructureType::InstanceCreateInfo,
                p_next: ptr::null(),
                flags: 0,
                p_application_info: &raw_app_info as *const RawVkApplicationInfo,
                enabled_layer_count: enabled_layers.len(),
                pp_enabled_layer_names: enabled_layers.as_ptr(),
                enabled_extension_count: enabled_extensions.len(),
                pp_enabled_extension_names: enabled_extensions.as_ptr()
            };

            let mut handler : VkHandler = 0;

            let handler_ptr = &mut handler as *mut VkHandler;
            let create_info_ptr = &raw_instance_create_info as *const RawVkInstanceCreateInfo;

            let result = vkCreateInstance(create_info_ptr, VkAllocator::null(), handler_ptr);

            match result {
                VkResult::Success => Ok(VkInstance {
                    _handler: handler
                }),
                _ => Err(result)
            }
        }
    }

    pub fn get_physical_devices(&self) -> Vec<VkPhysicalDevice> {
        unsafe {
            let mut count : u32 = 0;
            let count_ptr = &mut count as *mut u32;
            let mut handler_vec : Vec<VkHandler> = Vec::new();

            vkEnumeratePhysicalDevices(self._handler, count_ptr, ptr::null_mut());
            handler_vec.reserve(count as usize);
            handler_vec.set_len(count as usize);
            vkEnumeratePhysicalDevices(self._handler, count_ptr, handler_vec.as_mut_ptr());

            handler_vec.into_iter().map(|handler| VkPhysicalDevice::from_handler(handler)).collect()
        }
    }
}

impl ops::Drop for VkInstance {
    fn drop(&mut self) {
        unsafe {
            vkDestroyInstance(self._handler, VkAllocator::null());
        }
    }
}

extern {
    fn vkCreateInstance(create_info: *const RawVkInstanceCreateInfo, allocator: *const VkAllocator, instance: *mut VkHandler) -> VkResult;
    fn vkDestroyInstance(instance: VkHandler, allocator: *const VkAllocator);
    fn vkEnumeratePhysicalDevices(instance: VkHandler, count: *mut u32, ptr: *mut VkHandler);
}