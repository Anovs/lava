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
use vk::vk_extent_2d::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkMultisampleProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub max_sample_location_grid_size: RawVkExtent2D,
}

#[derive(Debug, Clone)]
pub struct VkMultisampleProperties {
    pub max_sample_location_grid_size: VkExtent2D,
}

impl VkRawType<VkMultisampleProperties> for RawVkMultisampleProperties {
    fn vk_to_wrapped(src: &RawVkMultisampleProperties) -> VkMultisampleProperties {
        VkMultisampleProperties {
            max_sample_location_grid_size: RawVkExtent2D::vk_to_wrapped(&src.max_sample_location_grid_size),
        }
    }
}

impl VkWrappedType<RawVkMultisampleProperties> for VkMultisampleProperties {
    fn vk_to_raw(src: &VkMultisampleProperties, dst: &mut RawVkMultisampleProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::MultisamplePropertiesExt);
        dst.next = ptr::null();
        dst.max_sample_location_grid_size = vk_to_raw_value(&src.max_sample_location_grid_size);
    }
}

impl Default for VkMultisampleProperties {
    fn default() -> VkMultisampleProperties {
        VkMultisampleProperties {
            max_sample_location_grid_size: VkExtent2D::default(),
        }
    }
}

impl VkSetup for VkMultisampleProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        VkSetup::vk_setup(&mut self.max_sample_location_grid_size, fn_table, instance, device);
    }
}

impl VkFree for RawVkMultisampleProperties {
    fn vk_free(&mut self) {
        RawVkExtent2D::vk_free(&mut self.max_sample_location_grid_size);
    }
}