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
use vk::ext::vk_debug_utils_messenger_callback_data_flags::*;
use vk::ext::vk_debug_utils_label::*;
use vk::ext::vk_debug_utils_object_name_info::*;

#[repr(C)]
pub struct RawVkDebugUtilsMessengerCallbackData {
    pub s_type: RawVkStructureType,
    pub next: *const c_void,
    pub flags: RawVkDebugUtilsMessengerCallbackDataFlags,
    pub message_id_name: *mut c_char,
    pub message_id_number: i32,
    pub message: *mut c_char,
    pub queue_label_count: u32,
    pub queue_labels: *mut RawVkDebugUtilsLabel,
    pub cmd_buf_label_count: u32,
    pub cmd_buf_labels: *mut RawVkDebugUtilsLabel,
    pub object_count: u32,
    pub objects: *mut RawVkDebugUtilsObjectNameInfo,
}

#[derive(Debug, Clone)]
pub struct VkDebugUtilsMessengerCallbackData<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'd: 'c,
        'f: 'e,
        'h: 'g,
{
    pub flags: VkDebugUtilsMessengerCallbackDataFlags,
    pub message_id_name: Option<&'a str>,
    pub message_id_number: isize,
    pub message: &'b str,
    pub queue_labels: Option<&'c [VkDebugUtilsLabel<'d>]>,
    pub cmd_buf_labels: Option<&'e [VkDebugUtilsLabel<'f>]>,
    pub objects: &'g [VkDebugUtilsObjectNameInfo<'h>],
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> VkWrappedType<RawVkDebugUtilsMessengerCallbackData> for VkDebugUtilsMessengerCallbackData<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'd: 'c,
        'f: 'e,
        'h: 'g,
{
    fn vk_to_raw(src: &VkDebugUtilsMessengerCallbackData, dst: &mut RawVkDebugUtilsMessengerCallbackData) {
        dst.s_type = vk_to_raw_value(&VkStructureType::DebugUtilsMessengerCallbackDataExt);
        dst.next = ptr::null();
        dst.flags = vk_to_raw_value(&src.flags);
        dst.message_id_name = new_ptr_string_checked(src.message_id_name);
        dst.message_id_number = vk_to_raw_value(&src.message_id_number);
        dst.message = new_ptr_string(src.message);
        dst.queue_label_count = get_array_option_len(src.queue_labels) as u32;
        dst.queue_labels = new_ptr_vk_array_checked(src.queue_labels);
        dst.cmd_buf_label_count = get_array_option_len(src.cmd_buf_labels) as u32;
        dst.cmd_buf_labels = new_ptr_vk_array_checked(src.cmd_buf_labels);
        dst.object_count = src.objects.len() as u32;
        dst.objects = new_ptr_vk_array(src.objects);
    }
}

impl Default for VkDebugUtilsMessengerCallbackData<'static, 'static, 'static, 'static, 'static, 'static, 'static, 'static> {
    fn default() -> VkDebugUtilsMessengerCallbackData<'static, 'static, 'static, 'static, 'static, 'static, 'static, 'static> {
        VkDebugUtilsMessengerCallbackData {
            flags: VkDebugUtilsMessengerCallbackDataFlags::default(),
            message_id_name: None,
            message_id_number: 0,
            message: "",
            queue_labels: None,
            cmd_buf_labels: None,
            objects: &[],
        }
    }
}

impl<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h> VkSetup for VkDebugUtilsMessengerCallbackData<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h>
    where
        'd: 'c,
        'f: 'e,
        'h: 'g,
{
    fn vk_setup(&mut self, fn_table: *mut VkInstanceFunctionTable, instance: RawVkInstance, device: RawVkDevice) {
        
    }
}

impl VkFree for RawVkDebugUtilsMessengerCallbackData {
    fn vk_free(&mut self) {
        free_ptr(self.message_id_name);
        free_ptr(self.message);
        free_vk_ptr_array(self.queue_label_count as usize, self.queue_labels);
        free_vk_ptr_array(self.cmd_buf_label_count as usize, self.cmd_buf_labels);
        free_vk_ptr_array(self.object_count as usize, self.objects);
    }
}