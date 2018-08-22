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
use vk::vk_point_clipping_behavior::*;

#[repr(C)]
#[derive(Debug)]
pub struct RawVkPhysicalDevicePointClippingProperties {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub point_clipping_behavior: RawVkPointClippingBehavior,
}

#[derive(Debug, Clone)]
pub struct VkPhysicalDevicePointClippingProperties {
    pub point_clipping_behavior: VkPointClippingBehavior,
}

impl VkRawType<VkPhysicalDevicePointClippingProperties> for RawVkPhysicalDevicePointClippingProperties {
    fn vk_to_wrapped(src: &RawVkPhysicalDevicePointClippingProperties) -> VkPhysicalDevicePointClippingProperties {
        VkPhysicalDevicePointClippingProperties {
            point_clipping_behavior: RawVkPointClippingBehavior::vk_to_wrapped(&src.point_clipping_behavior),
        }
    }
}

impl VkWrappedType<RawVkPhysicalDevicePointClippingProperties> for VkPhysicalDevicePointClippingProperties {
    fn vk_to_raw(src: &VkPhysicalDevicePointClippingProperties, dst: &mut RawVkPhysicalDevicePointClippingProperties) {
        dst.s_type = vk_to_raw_value(&VkStructureType::PhysicalDevicePointClippingProperties);
        dst.next = ptr::null();
        dst.point_clipping_behavior = vk_to_raw_value(&src.point_clipping_behavior);
    }
}

impl Default for VkPhysicalDevicePointClippingProperties {
    fn default() -> VkPhysicalDevicePointClippingProperties {
        VkPhysicalDevicePointClippingProperties {
            point_clipping_behavior: VkPointClippingBehavior::default(),
        }
    }
}

impl VkSetup for VkPhysicalDevicePointClippingProperties {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkPhysicalDevicePointClippingProperties {
    fn vk_free(&mut self) {
        
    }
}