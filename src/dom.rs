use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::event::Event;

pub struct FpsCounter {
    inner: web_sys::Element,
}

impl FpsCounter {
    pub fn update(&self, fps: i32) {
        self.inner
            .set_text_content(Some(&format!("FPS: {:?}", fps)));
    }
}

pub fn get_fps_counter() -> FpsCounter {
    FpsCounter {
        inner: web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("f"))
            .expect("FPS counter not found"),
    }
}

pub fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("c"))
        .map(|e| e.unchecked_into::<HtmlCanvasElement>())
        .expect("Canvas not found")
}

pub struct LogList {
    inner: web_sys::Element,
}

impl LogList {
    pub fn log_event(&self, event: &Event<()>) {
        if let Event::WindowEvent { event, .. } = &event {
            let log = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.create_element("li").ok())
                .expect("Log could not be created");
            log.set_text_content(Some(&format!("{:?}", event)));

            self.inner
                .insert_before(&log, self.inner.first_child().as_ref())
                .expect("Could not append log");
        }
    }

    pub fn log_message(&self, message: &str) {
        let log = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.create_element("li").ok())
            .expect("Log could not be created");
        log.set_text_content(Some(message));

        self.inner
            .insert_before(&log, self.inner.first_child().as_ref())
            .expect("Could not append log");
    }
}

pub fn get_log_list() -> LogList {
    LogList {
        inner: web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("l"))
            .expect("Log list not found"),
    }
}
