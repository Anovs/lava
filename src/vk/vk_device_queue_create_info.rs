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
use vk::vk_device_queue_create_flags::*;

#[repr(C)]
pub struct RawVkDeviceQueueCreateInfo {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDeviceQueueCreateFlags,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub queue_priorities: *const f32,
}

#[derive(Debug, Clone)]
pub struct VkDeviceQueueCreateInfo<'a> {
    pub flags: VkDeviceQueueCreateFlags,
    pub queue_family_index: usize,
    pub queue_priorities: &'a [f32],
}

impl<'a> VkWrappedType<RawVkDeviceQueueCreateInfo> for VkDeviceQueueCreateInfo<'a> {
    fn vk_to_raw(src: &VkDeviceQueueCreateInfo, dst: &mut RawVkDeviceQueueCreateInfo) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DeviceQueueCreateInfo);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.queue_family_index = vk_to_raw_value(&src.queue_family_index);
        dst.queue_count = src.queue_priorities.len() as u32;
        dst.queue_priorities = src.queue_priorities.as_ptr();
    }
}

impl Default for VkDeviceQueueCreateInfo<'static> {
    fn default() -> VkDeviceQueueCreateInfo<'static> {
        VkDeviceQueueCreateInfo {
            flags: VkDeviceQueueCreateFlags::default(),
            queue_family_index: 0,
            queue_priorities: &[],
        }
    }
}

impl<'a> VkSetup for VkDeviceQueueCreateInfo<'a> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDeviceQueueCreateInfo {
    fn vk_free(&mut self) {
        
    }
}