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
use vk::vk_descriptor_type::*;
use vk::vk_shader_stage_flags::*;
use vk::vk_sampler::*;

#[repr(C)]
pub struct RawVkDescriptorSetLayoutBinding {
    binding: u32,
    descriptor_type: RawVkDescriptorType,
    descriptor_count: u32,
    stage_flags: RawVkShaderStageFlags,
    immutable_samplers: *mut RawVkSampler,
}

#[derive(Debug, Clone)]
pub struct VkDescriptorSetLayoutBinding<'a, 'b>
    where
        'b: 'a,
{
    pub binding: usize,
    pub descriptor_type: VkDescriptorType,
    pub stage_flags: VkShaderStageFlags,
    pub immutable_samplers: Option<&'a [&'b VkSampler]>,
}

impl<'a, 'b> VkWrappedType<RawVkDescriptorSetLayoutBinding> for VkDescriptorSetLayoutBinding<'a, 'b>
    where
        'b: 'a,
{
    fn vk_to_raw(src: &VkDescriptorSetLayoutBinding, dst: &mut RawVkDescriptorSetLayoutBinding) {
        dst.binding = vk_to_raw_value(&src.binding);
        dst.descriptor_type = vk_to_raw_value(&src.descriptor_type);
        dst.descriptor_count = get_array_option_len(src.immutable_samplers) as u32;
        dst.stage_flags = vk_to_raw_value(&src.stage_flags);
        dst.immutable_samplers = new_ptr_vk_array_checked_from_ref(src.immutable_samplers);
    }
}

impl Default for VkDescriptorSetLayoutBinding<'static, 'static> {
    fn default() -> VkDescriptorSetLayoutBinding<'static, 'static> {
        VkDescriptorSetLayoutBinding {
            binding: 0,
            descriptor_type: VkDescriptorType::default(),
            stage_flags: VkShaderStageFlags::default(),
            immutable_samplers: None,
        }
    }
}

impl<'a, 'b> VkSetup for VkDescriptorSetLayoutBinding<'a, 'b>
    where
        'b: 'a,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDescriptorSetLayoutBinding {
    fn vk_free(&mut self) {
        free_ptr(self.immutable_samplers);
    }
}