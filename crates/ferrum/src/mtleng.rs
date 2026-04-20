// ; MTLEngine is where is stashed all the rendering engine logic
use std::ptr::null_mut;
use glfw::ffi::{GLFW_CLIENT_API, GLFW_NO_API, GLFWwindow,
    glfwCreateWindow, glfwGetCocoaWindow, glfwInit,
    glfwPollEvents, glfwTerminate, glfwWindowHint, glfwWindowShouldClose};
use objc2_app_kit::NSWindow;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice, MTLPixelFormat};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_quartz_core::CAMetalLayer;

// Only OBJ-C objects need Retained

pub struct MTLEngine {
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    glfw_window: *mut GLFWwindow,
    metal_window: Retained<NSWindow>,
    metal_layer: Retained<CAMetalLayer>
}

impl MTLEngine {

    pub unsafe fn new() -> Self {
        glfwInit();
        // Tell GLFW to not create OPENGL graphics context
        glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
        // HARDCODED WIDHT AND LENGTH FOR NOW
        let glfw_window = glfwCreateWindow(
            800,
            600,
            c"Metal Window".as_ptr(),
            null_mut(),
            null_mut()
        );

        let device =
            MTLCreateSystemDefaultDevice()
            .expect("Failed to find a Metal device.");

        let metal_window =
            Retained::retain(glfwGetCocoaWindow(glfw_window) as *mut NSWindow)
                .expect("Unable to find Objc Handle");

        let metal_layer = CAMetalLayer::new();
        metal_layer.setDevice(Some(&device));
        metal_layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
        metal_window.contentView().unwrap().setLayer(Some(&metal_layer));
        metal_window.contentView().unwrap().setWantsLayer(true);

        Self
        {
            device,
            glfw_window,
            metal_window,
            metal_layer,
        }
    }

    pub unsafe fn run(&self) {
        while glfwWindowShouldClose(self.glfw_window) == 0 {
            glfwPollEvents();
        }
    }

    pub unsafe fn cleanup(&self) {
        glfwTerminate();
    }

}
