use std::string::String;
use std::ffi::CStr;
use std::vec::Vec;
use std::cmp;
use std::mem;
use std::os::raw::c_char;
use utils::vk_type::VkRawType;
use utils::vk_type::VkWrappedType;

pub fn vk_to_raw_value<W : VkWrappedType<R>, R>(value: &W) -> R {
    unsafe {
        let mut dst = mem::uninitialized();
        W::vk_to_raw(value, &mut dst);
        dst
    }
}

pub fn vk_to_raw_array<W : VkWrappedType<R>, R>(array: &[W], dst: &mut[R]) {
    let len = cmp::min(array.len(), dst.len());

    for i in 0..len {
        W::vk_to_raw(&array[i], &mut dst[i])
    }
}

pub fn to_array<T : Copy>(src: &[T], dst: &mut[T]) {
    let len = cmp::min(src.len(), dst.len());

    for i in 0..len {
        dst[i] = src[i];
    }
}

pub fn string_to_byte_array(string: &str, dst: &mut[c_char]) {
    let bytes = string.as_bytes();
    let len = cmp::min(bytes.len(), dst.len() - 1);

    for i in 0..len {
        dst[i] = bytes[i] as c_char;
    }

    dst[len] = 0;
}

pub fn new_vk_array<R : VkRawType<W>, W>(length: u32, ptr: *const R) -> Vec<W>
{
    unsafe {
        let len = length as usize;
        let mut vector : Vec<W> = Vec::with_capacity(len);

        for i in 0..len {
            vector.push(R::vk_to_wrapped(&*ptr.add(i)));
        }

        vector
    }
}

pub fn new_array<T : Copy>(length: u32, ptr: *const T) -> Vec<T> {
    unsafe {
        let len = length as usize;
        let mut vector : Vec<T> = Vec::with_capacity(len);

        for i in 0..len {
            vector.push(*ptr.add(i));
        }

        vector
    }
}

pub fn new_string(ptr: *const c_char) -> String {
    unsafe {
        String::from_utf8_unchecked(CStr::from_ptr(ptr).to_bytes().to_vec())
    }
}