use instant::Instant;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;
use winit::event::Event;

pub struct Dom {
    pub log_list: LogList,
    pub fps_counter: FpsCounter,
}

impl Dom {
    pub fn new() -> Self {
        Self {
            log_list: get_log_list(),
            fps_counter: get_fps_counter(),
        }
    }
}

pub fn get_canvas() -> HtmlCanvasElement {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("c"))
        .map(|e| e.unchecked_into::<HtmlCanvasElement>())
        .expect("Canvas not found")
}

fn get_fps_counter() -> FpsCounter {
    FpsCounter {
        inner: web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("f"))
            .expect("FPS counter not found"),
        last_frame: None,
        dt_filtered: 0_f32,
    }
}

fn get_log_list() -> LogList {
    LogList {
        inner: web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("l"))
            .expect("Log list not found"),
    }
}

pub struct FpsCounter {
    inner: web_sys::Element,
    last_frame: Option<Instant>,
    dt_filtered: f32,
}

impl FpsCounter {
    pub fn update(&mut self) {
        const FPS_FILTER_PERIOD: f32 = 10.0;

        match self.last_frame {
            Some(last_frame) => {
                // Update FPS
                let now = Instant::now();
                let dt_duration = now - last_frame;
                self.last_frame = Some(now);

                let dt_raw = dt_duration.as_secs_f32();
                self.dt_filtered = self.dt_filtered
                    + (dt_raw - self.dt_filtered) / FPS_FILTER_PERIOD;

                let label = &format!("FPS: {:?}", self.fps());
                self.inner.set_text_content(Some(label));
            }
            None => {
                self.last_frame = Some(Instant::now());
            }
        };
    }

    pub fn fps(&self) -> i32 {
        (1.0 / self.dt_filtered) as i32
    }
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
