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
use vulkan::vk::{VkInstanceCreateFlags,RawVkInstanceCreateFlags};
use vulkan::vk::{VkApplicationInfo,RawVkApplicationInfo};

/// Wrapper for [VkInstanceCreateInfo](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkInstanceCreateInfo.html)
#[derive(Debug, Clone)]
pub struct VkInstanceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'c: 'a,
        'e: 'd,
        'g: 'f,
{
    pub flags: VkInstanceCreateFlags,
    pub application_info: Option<&'a VkApplicationInfo<'b, 'c>>,
    pub enabled_layer_names: &'d [&'e str],
    pub enabled_extension_names: &'f [&'g str],
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkInstanceCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkInstanceCreateFlags,
    pub application_info: *mut RawVkApplicationInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *mut *mut c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *mut *mut c_char,
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkWrappedType<RawVkInstanceCreateInfo> for VkInstanceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'c: 'a,
        'e: 'd,
        'g: 'f,
{
    fn vk_to_raw(src: &VkInstanceCreateInfo, dst: &mut RawVkInstanceCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::InstanceCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.application_info = new_ptr_vk_value_checked(src.application_info);
        dst.enabled_layer_count = src.enabled_layer_names.len() as u32;
        dst.enabled_layer_names = new_ptr_string_array(src.enabled_layer_names);
        dst.enabled_extension_count = src.enabled_extension_names.len() as u32;
        dst.enabled_extension_names = new_ptr_string_array(src.enabled_extension_names);
    }
}

impl Default for VkInstanceCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkInstanceCreateInfo<'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkInstanceCreateInfo {
            flags: VkInstanceCreateFlags::default(),
            application_info: None,
            enabled_layer_names: &[],
            enabled_extension_names: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g> VkSetup for VkInstanceCreateInfo<'a, 'b, 'c, 'd, 'e, 'f, 'g>
    where
        'b: 'a,
        'c: 'a,
        'e: 'd,
        'g: 'f,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkInstanceCreateInfo {
    fn vk_free(&mut self) {
        free_vk_ptr(self.application_info);
        free_ptr(self.enabled_layer_names);
        free_ptr(self.enabled_extension_names);
    }
}