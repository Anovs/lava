// Generated by `scripts/generate_vk.js`

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
use vk::vk_instance_function_table::*;
use vk::vk_instance::*;
use vk::vk_device::*;
use vk::vk_structure_type::*;
use vk::khr::vk_swapchain::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkImageSwapchainCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub swapchain: RawVkSwapchain,
}

#[derive(Debug, Clone)]
pub struct VkImageSwapchainCreateInfo<'a> {
    pub swapchain: Option<&'a VkSwapchain>,
}

impl<'a> VkWrappedType<RawVkImageSwapchainCreateInfo> for VkImageSwapchainCreateInfo<'a> {
    fn vk_to_raw(src: &VkImageSwapchainCreateInfo, dst: &mut RawVkImageSwapchainCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageSwapchainCreateInfoKhr);
        dst.next = ptr::null();
        dst.swapchain = if src.swapchain.is_some() { vk_to_raw_value(src.swapchain.unwrap()) } else { 0 };
    }
}

impl Default for VkImageSwapchainCreateInfo<'static> {
    fn default() -> VkImageSwapchainCreateInfo<'static> {
        VkImageSwapchainCreateInfo {
            swapchain: None,
        }
    }
}

impl<'a> VkSetup for VkImageSwapchainCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImageSwapchainCreateInfo {
    fn vk_free(&mut self) {
        
    }
}