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
use vulkan::khr::{VkPresentRegion,RawVkPresentRegion};

/// Wrapper for [VkPresentRegionsKHR](https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkPresentRegionsKHR.html).
#[derive(Debug, Clone)]
pub struct VkPresentRegions<'a, 'b>
    where
        'b: 'a,
{
    pub regions: Option<&'a [VkPresentRegion<'b>]>,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkPresentRegions {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub swapchain_count: u32,
    pub regions: *mut RawVkPresentRegion,
}

impl<'a, 'b> VkWrappedType<RawVkPresentRegions> for VkPresentRegions<'a, 'b>
    where
        'b: 'a,
{
    fn vk_to_raw(src: &VkPresentRegions, dst: &mut RawVkPresentRegions) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PresentRegionsKhr);
        dst.next = ptr::null();
        dst.swapchain_count = get_array_option_len(src.regions) as u32;
        dst.regions = new_ptr_vk_array_checked(src.regions);
    }
}

impl Default for VkPresentRegions<'static, 'static> {
    fn default() -> VkPresentRegions<'static, 'static> {
        VkPresentRegions {
            regions: None,
        }
    }
}

impl<'a, 'b> VkSetup for VkPresentRegions<'a, 'b>
    where
        'b: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPresentRegions {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.swapchain_count as usize, self.regions);
    }
}