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
use vk::vk_image::*;

#[repr(C)]
pub struct RawVkImageSparseMemoryRequirementsInfo2 {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub image: RawVkImage,
}

#[derive(Debug, Clone)]
pub struct VkImageSparseMemoryRequirementsInfo2<'a> {
    pub image: &'a VkImage,
}

impl<'a> VkWrappedType<RawVkImageSparseMemoryRequirementsInfo2> for VkImageSparseMemoryRequirementsInfo2<'a> {
    fn vk_to_raw(src: &VkImageSparseMemoryRequirementsInfo2, dst: &mut RawVkImageSparseMemoryRequirementsInfo2) {
        dst.s_type = vk_to_raw_value(&VkStructureType::ImageSparseMemoryRequirementsInfo2);
        dst.next = ptr::null();
        dst.image = vk_to_raw_value(src.image);
    }
}

impl Default for VkImageSparseMemoryRequirementsInfo2<'static> {
    fn default() -> VkImageSparseMemoryRequirementsInfo2<'static> {
        VkImageSparseMemoryRequirementsInfo2 {
            image: vk_null_ref(),
        }
    }
}

impl<'a> VkSetup for VkImageSparseMemoryRequirementsInfo2<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkImageSparseMemoryRequirementsInfo2 {
    fn vk_free(&mut self) {
        
    }
}