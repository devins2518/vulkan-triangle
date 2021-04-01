use vulkano::{
    self,
    device::{Device, DeviceExtensions, Features},
    image::ImageUsage,
    instance::{Instance, PhysicalDevice},
    swapchain::{ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain},
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Fullscreen, WindowBuilder},
};

fn main() {
    // Create Vulkan logical device for rendering
    let instance = Instance::new(None, &vulkano_win::required_extensions(), None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("Failed to enumerate devices!");
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };

        Device::new(
            physical,
            physical.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    let event_loop = EventLoop::new();

    let surface = WindowBuilder::new()
        .with_title("Vulkan Triangle")
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

    // Set up swap chain
    let capabilities = surface
        .capabilities(physical)
        .expect("Failed to get capabilities of surface layer!");
    let dimensions = capabilities.current_extent.unwrap_or([1280, 1024]);
    let alpha = capabilities
        .supported_composite_alpha
        .iter()
        .next()
        .unwrap();
    let format = capabilities.supported_formats[0].0;

    let (swapchain, images) = Swapchain::new(
        device.clone(),
        surface.clone(),
        capabilities.min_image_count,
        format,
        dimensions,
        1,
        ImageUsage::color_attachment(),
        &queue,
        SurfaceTransform::Identity,
        alpha,
        PresentMode::Fifo,
        FullscreenExclusive::Default,
        true,
        ColorSpace::SrgbNonLinear,
    )
    .expect("Failed to create swapchain");

    event_loop.run(|event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => *control_flow = ControlFlow::Exit,
        _ => (),
    });
}
