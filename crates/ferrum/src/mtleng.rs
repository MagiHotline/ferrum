use glfw::{GlfwReceiver, PWindow, WindowEvent, WindowMode};
use objc2_app_kit::NSWindow;
use objc2_metal::{MTLCreateSystemDefaultDevice, MTLDevice, MTLPixelFormat};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_quartz_core::CAMetalLayer;

// Only OBJ-C objects need Retained

pub struct MTLEngine {
    glfw: glfw::Glfw,
    events: GlfwReceiver<(f64, WindowEvent)>,
    device: Retained<ProtocolObject<dyn MTLDevice>>,
    glfw_window: PWindow,
    metal_window: Retained<NSWindow>,
    metal_layer: Retained<CAMetalLayer>
}

impl MTLEngine {

    pub fn new() -> Self {
        let mut glfw = glfw::init_no_callbacks().expect("Failed to init glfw");
        // Tell GLFW to not create OPENGL graphics context
        glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi);
        //glfwWindowHint(GLFW_CLIENT_API, GLFW_NO_API);
        // HARDCODED WIDHT AND LENGTH FOR NOW
        /*let glfw_window = */
        let (mut glfw_window, events) = glfw.create_window(
            800, 600,
            "Metal Window",
            WindowMode::Windowed
        ).expect("Failed to create window.");

        let device =
            MTLCreateSystemDefaultDevice()
            .expect("Failed to find a Metal device.");

        // let metal_window =
        //   Retained::retain(glfwGetCocoaWindow(glfw_window) as *mut NSWindow)
        //       .expect("Unable to find Objc Handle");
        //
        let metal_window = unsafe {
                Retained::retain(glfw::Window::get_cocoa_window(&glfw_window) as *mut NSWindow)
                    .expect("Unable to find Objc handle")
        };

        // Link device and layer to the window
        let metal_layer = CAMetalLayer::new();
        metal_layer.setDevice(Some(&device));
        metal_layer.setPixelFormat(MTLPixelFormat::BGRA8Unorm);
        metal_window.contentView().unwrap().setLayer(Some(&metal_layer));
        metal_window.contentView().unwrap().setWantsLayer(true);

        Self
        {
            glfw,
            events,
            device,
            glfw_window,
            metal_window,
            metal_layer,
        }
    }

    pub fn run(&mut self) {
        while !glfw::Window::should_close(&self.glfw_window) {
            self.glfw.poll_events();
        }
    }

}
