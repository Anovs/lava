// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::ptr::null;
use libc::*;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RawVkSwapchainCreateInfoKHR {
    s_type: RawVkStructureType,
    next: *const c_void,
    flags: RawVkSwapchainCreateFlagsKHR,
    surface: RawVkSurfaceKHR,
    min_image_count: u32,
    image_format: RawVkFormat,
    image_color_space: RawVkColorSpaceKHR,
    image_extent: RawVkExtent2D,
    image_array_layers: u32,
    image_usage: RawVkImageUsageFlags,
    image_sharing_mode: RawVkSharingMode,
    queue_family_index_count: u32,
    queue_family_indices: *mut u32,
    pre_transform: RawVkSurfaceTransformFlagBitsKHR,
    composite_alpha: RawVkCompositeAlphaFlagBitsKHR,
    present_mode: RawVkPresentModeKHR,
    clipped: RawVkBool32,
    old_swapchain: RawVkSwapchainKHR,
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct VkSwapchainCreateInfoKHR {
    pub flags: VkSwapchainCreateFlagsKHR,
    pub surface: &'a VkSurfaceKHR,
    pub min_image_count: u32,
    pub image_format: VkFormat,
    pub image_color_space: VkColorSpaceKHR,
    pub image_extent: VkExtent2D,
    pub image_array_layers: u32,
    pub image_usage: VkImageUsageFlags,
    pub image_sharing_mode: VkSharingMode,
    pub queue_family_indices: Vec<usize>,
    pub pre_transform: VkSurfaceTransformFlagsKHR,
    pub composite_alpha: VkCompositeAlphaFlagsKHR,
    pub present_mode: VkPresentModeKHR,
    pub clipped: bool,
    pub old_swapchain: &'b VkSwapchainKHR,
}

impl VkFrom<VkSwapchainCreateInfoKHR> for RawVkSwapchainCreateInfoKHR {
    
    fn vk_from(value: &VkSwapchainCreateInfoKHR) -> Self {
        unsafe {
            Self {
                s_type: VkFrom::vk_from(&VkStructureType::SwapchainCreateInfoKHR),
                next: null(),
                flags: VkFrom::vk_from(&value.flags),
                surface: VkFrom::vk_from(value.surface),
                min_image_count: value.min_image_count,
                image_format: VkFrom::vk_from(&value.image_format),
                image_color_space: VkFrom::vk_from(&value.image_color_space),
                image_extent: VkFrom::vk_from(&value.image_extent),
                image_array_layers: value.image_array_layers,
                image_usage: VkFrom::vk_from(&value.image_usage),
                image_sharing_mode: VkFrom::vk_from(&value.image_sharing_mode),
                queue_family_index_count: value.queue_family_indices.len() as u32,
                queue_family_indices: copy_as_c_array(&value.queue_family_indices.iter().map(|x| *x as u32).collect()),
                pre_transform: VkFrom::vk_from(&value.pre_transform),
                composite_alpha: VkFrom::vk_from(&value.composite_alpha),
                present_mode: VkFrom::vk_from(&value.present_mode),
                clipped: VkFrom::vk_from(&value.clipped),
                old_swapchain: VkFrom::vk_from(value.old_swapchain),
            }
        }
    }
}

impl VkFrom<RawVkSwapchainCreateInfoKHR> for VkSwapchainCreateInfoKHR {
    
    fn vk_from(value: &RawVkSwapchainCreateInfoKHR) -> Self {
        unsafe {
            Self {
                flags: VkFrom::vk_from(&value.flags),
                surface: VkFrom::vk_from(&value.surface),
                min_image_count: value.min_image_count,
                image_format: VkFrom::vk_from(&value.image_format),
                image_color_space: VkFrom::vk_from(&value.image_color_space),
                image_extent: VkFrom::vk_from(&value.image_extent),
                image_array_layers: value.image_array_layers,
                image_usage: VkFrom::vk_from(&value.image_usage),
                image_sharing_mode: VkFrom::vk_from(&value.image_sharing_mode),
                queue_family_indices: vec_from_c_ptr(value.queue_family_index_count, value.queue_family_indices).iter().map(|x| *x as usize).collect(),
                pre_transform: VkFrom::vk_from(&value.pre_transform),
                composite_alpha: VkFrom::vk_from(&value.composite_alpha),
                present_mode: VkFrom::vk_from(&value.present_mode),
                clipped: VkFrom::vk_from(&value.clipped),
                old_swapchain: VkFrom::vk_from(&value.old_swapchain),
            }
        }
    }
}