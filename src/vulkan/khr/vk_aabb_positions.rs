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

/// Wrapper for [VkAabbPositionsKHR](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkAabbPositionsKHR.html).
#[derive(Debug, Clone)]
pub struct VkAabbPositions {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

#[doc(hidden)]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkAabbPositions {
    pub min_x: f32,
    pub min_y: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_y: f32,
    pub max_z: f32,
}

impl VkWrappedType<RawVkAabbPositions> for VkAabbPositions {
    fn vk_to_raw(src: &VkAabbPositions, dst: &mut RawVkAabbPositions) {
        dst.min_x = src.min_x;
        dst.min_y = src.min_y;
        dst.min_z = src.min_z;
        dst.max_x = src.max_x;
        dst.max_y = src.max_y;
        dst.max_z = src.max_z;
    }
}

impl VkRawType<VkAabbPositions> for RawVkAabbPositions {
    fn vk_to_wrapped(src: &RawVkAabbPositions) -> VkAabbPositions {
        VkAabbPositions {
            min_x: src.min_x,
            min_y: src.min_y,
            min_z: src.min_z,
            max_x: src.max_x,
            max_y: src.max_y,
            max_z: src.max_z,
        }
    }
}

impl Default for VkAabbPositions {
    fn default() -> VkAabbPositions {
        VkAabbPositions {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 0.0,
            max_y: 0.0,
            max_z: 0.0,
        }
    }
}

impl VkSetup for VkAabbPositions {
    fn vk_setup(&mut self, fn_table: *mut VkFunctionTable) {
        
    }
}

impl VkFree for RawVkAabbPositions {
    fn vk_free(&self) {
        
    }
}