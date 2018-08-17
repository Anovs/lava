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
use vk::ext::vk_attachment_sample_locations::*;
use vk::ext::vk_subpass_sample_locations::*;

#[repr(C)]
pub struct RawVkRenderPassSampleLocationsBeginInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    attachment_initial_sample_locations_count: u32,
    attachment_initial_sample_locations: *mut RawVkAttachmentSampleLocations,
    post_subpass_sample_locations_count: u32,
    post_subpass_sample_locations: *mut RawVkSubpassSampleLocations,
}

#[derive(Debug, Clone)]
pub struct VkRenderPassSampleLocationsBeginInfo<'a, 'b, 'c, 'd>
    where
        'b: 'a,
        'd: 'c,
{
    pub attachment_initial_sample_locations: &'a [VkAttachmentSampleLocations<'b>],
    pub post_subpass_sample_locations: &'c [VkSubpassSampleLocations<'d>],
}

impl<'a, 'b, 'c, 'd> VkWrappedType<RawVkRenderPassSampleLocationsBeginInfo> for VkRenderPassSampleLocationsBeginInfo<'a, 'b, 'c, 'd>
    where
        'b: 'a,
        'd: 'c,
{
    fn vk_to_raw(src: &VkRenderPassSampleLocationsBeginInfo, dst: &mut RawVkRenderPassSampleLocationsBeginInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::RenderPassSampleLocationsBeginInfoExt);
        dst.next = ptr::null();
        dst.attachment_initial_sample_locations_count = src.attachment_initial_sample_locations.len() as u32;
        dst.attachment_initial_sample_locations = new_ptr_vk_array(src.attachment_initial_sample_locations);
        dst.post_subpass_sample_locations_count = src.post_subpass_sample_locations.len() as u32;
        dst.post_subpass_sample_locations = new_ptr_vk_array(src.post_subpass_sample_locations);
    }
}

impl Default for VkRenderPassSampleLocationsBeginInfo<'static, 'static, 'static, 'static> {
    fn default() -> VkRenderPassSampleLocationsBeginInfo<'static, 'static, 'static, 'static> {
        VkRenderPassSampleLocationsBeginInfo {
            attachment_initial_sample_locations: &[],
            post_subpass_sample_locations: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd> VkSetup for VkRenderPassSampleLocationsBeginInfo<'a, 'b, 'c, 'd>
    where
        'b: 'a,
        'd: 'c,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkRenderPassSampleLocationsBeginInfo {
    fn vk_free(&mut self) {
        free_vk_ptr_array(self.attachment_initial_sample_locations_count as usize, self.attachment_initial_sample_locations);
        free_vk_ptr_array(self.post_subpass_sample_locations_count as usize, self.post_subpass_sample_locations);
    }
}