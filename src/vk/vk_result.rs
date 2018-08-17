// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkResult = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkResult {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorSurfaceLostKhr = -1000000000,
    ErrorNativeWindowInUseKhr = -1000000001,
    SuboptimalKhr = 1000001003,
    ErrorOutOfDateKhr = -1000001004,
    ErrorIncompatibleDisplayKhr = -1000003001,
    ErrorValidationFailedExt = -1000011001,
    ErrorInvalidShaderNv = -1000012000,
    ErrorFragmentationExt = -1000161000,
    ErrorNotPermittedExt = -1000174001,
}

impl VkRawType<VkResult> for RawVkResult {
    fn vk_to_wrapped(src: &RawVkResult) -> VkResult {
        unsafe {
            *((src as *const i32) as *const VkResult)
        }
    }
}

impl VkWrappedType<RawVkResult> for VkResult {
    fn vk_to_raw(src: &VkResult, dst: &mut RawVkResult) {
        *dst = *src as i32
    }
}

impl Default for VkResult {
    fn default() -> VkResult {
        VkResult::Success
    }
}