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
use vk::nv::vk_geometry_triangles::*;
use vk::nv::vk_geometry_aabb::*;

#[derive(Debug, Clone)]
pub struct VkGeometryData<'a, 'b, 'c, 'd> {
    pub triangles: VkGeometryTriangles<'a, 'b, 'c>,
    pub aabbs: VkGeometryAABB<'d>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct RawVkGeometryData {
    pub triangles: RawVkGeometryTriangles,
    pub aabbs: RawVkGeometryAABB,
}

impl<'a, 'b, 'c, 'd> VkWrappedType<RawVkGeometryData> for VkGeometryData<'a, 'b, 'c, 'd> {
    fn vk_to_raw(src: &VkGeometryData, dst: &mut RawVkGeometryData) {
        dst.triangles = vk_to_raw_value(&src.triangles);
        dst.aabbs = vk_to_raw_value(&src.aabbs);
    }
}

impl Default for VkGeometryData<'static, 'static, 'static, 'static> {
    fn default() -> VkGeometryData<'static, 'static, 'static, 'static> {
        VkGeometryData {
            triangles: VkGeometryTriangles::default(),
            aabbs: VkGeometryAABB::default(),
        }
    }
}

impl<'a, 'b, 'c, 'd> VkSetup for VkGeometryData<'a, 'b, 'c, 'd> {
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkGeometryData {
    fn vk_free(&mut self) {
        RawVkGeometryTriangles::vk_free(&mut self.triangles);
        RawVkGeometryAABB::vk_free(&mut self.aabbs);
    }
}