// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ptr::null;
use libc::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawVkBufferCreateInfo {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkBufferCreateFlags,
    size: RawVkDeviceSize,
    usage: RawVkBufferUsageFlags,
    sharing_mode: RawVkSharingMode,
    queue_family_index_count: u32,
    queue_family_indices: *mut u32,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkBufferCreateInfo {
    pub flags: VkBufferCreateFlags,
    pub size: VkDeviceSize,
    pub usage: VkBufferUsageFlags,
    pub sharing_mode: VkSharingMode,
    pub queue_family_indices: Vec<usize>,
}

impl VkFrom<VkBufferCreateInfo> for RawVkBufferCreateInfo {
    
    fn vk_from(value: &VkBufferCreateInfo) -> Self {
        unsafe {
            Self {
                s_type: VkFrom::vk_from(&VkStructureType::BufferCreateInfo),
                next: null(),
                flags: VkFrom::vk_from(&value.flags),
                size: VkFrom::vk_from(&value.size),
                usage: VkFrom::vk_from(&value.usage),
                sharing_mode: VkFrom::vk_from(&value.sharing_mode),
                queue_family_index_count: value.queue_family_indices.len() as u32,
                queue_family_indices: copy_as_c_array(&value.queue_family_indices.iter().map(|x| *x as u32).collect()),
            }
        }
    }
}

impl VkFrom<RawVkBufferCreateInfo> for VkBufferCreateInfo {
    
    fn vk_from(value: &RawVkBufferCreateInfo) -> Self {
        unsafe {
            Self {
                flags: VkFrom::vk_from(&value.flags),
                size: VkFrom::vk_from(&value.size),
                usage: VkFrom::vk_from(&value.usage),
                sharing_mode: VkFrom::vk_from(&value.sharing_mode),
                queue_family_indices: vec_from_c_ptr(value.queue_family_index_count, value.queue_family_indices).iter().map(|x| *x as usize).collect(),
            }
        }
    }
}