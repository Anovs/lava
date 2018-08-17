// Generated by `scripts/generate_vk.js`

use utils::vk_traits::*;

pub type RawVkObjectType = i32;

#[repr(i32)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum VkObjectType {
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    SamplerYcbcrConversion = 1000156000,
    DescriptorUpdateTemplate = 1000085000,
    SurfaceKhr = 1000000000,
    SwapchainKhr = 1000001000,
    DisplayKhr = 1000002000,
    DisplayModeKhr = 1000002001,
    DebugReportCallbackExt = 1000011000,
    ObjectTableNvx = 1000086000,
    IndirectCommandsLayoutNvx = 1000086001,
    DebugUtilsMessengerExt = 1000128000,
    ValidationCacheExt = 1000160000,
}

impl VkRawType<VkObjectType> for RawVkObjectType {
    fn vk_to_wrapped(src: &RawVkObjectType) -> VkObjectType {
        unsafe {
            *((src as *const i32) as *const VkObjectType)
        }
    }
}

impl VkWrappedType<RawVkObjectType> for VkObjectType {
    fn vk_to_raw(src: &VkObjectType, dst: &mut RawVkObjectType) {
        *dst = *src as i32
    }
}

impl Default for VkObjectType {
    fn default() -> VkObjectType {
        VkObjectType::Unknown
    }
}