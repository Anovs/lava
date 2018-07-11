// Generated by `scripts/generate_vk.js`

use vk::*;
use std::os::raw::c_char;
use std::vec::Vec;
use std::ptr::null;
use libc::c_void;
use glfw::*;

pub type RawVkDescriptorSetLayout = RawVkHandle;

#[derive(Debug)]
pub struct VkDescriptorSetLayout {
    _handle: RawVkDescriptorSetLayout,
}

impl VkDescriptorSetLayout {
    
    pub fn handle(&self) -> RawVkDescriptorSetLayout {
        self._handle
    }
}

impl VkFrom<VkDescriptorSetLayout> for RawVkDescriptorSetLayout {
    
    fn vk_from(value: &VkDescriptorSetLayout) -> Self {
        value._handle
    }
}

impl VkFrom<RawVkDescriptorSetLayout> for VkDescriptorSetLayout {
    
    fn vk_from(value: &RawVkDescriptorSetLayout) -> Self {
        Self {
            _handle: *value,
        }
    }
}