use orbtk::{prelude::*, shell::ShellRequest};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::thread;
use crate::virtspace::VirtSpacePipeline;

// ---------------------------------------------------------------------------
// ORBTK GUI STATE
// ---------------------------------------------------------------------------

#[derive(AsAny)]
pub struct GuiState {
    updater_thread: Option<thread::JoinHandle<()>>,
    exit_updater: Arc<Mutex<bool>>,
    current_time_utc: DateTime<Utc>,
    frame_counter: u64
}

impl Default for GuiState {
    fn default() -> Self {
        GuiState {
            updater_thread: None,
            exit_updater: Arc::new(Mutex::new(false)),
            current_time_utc: Utc::now(),
            frame_counter: 0
        }
    }
}

impl State for GuiState {

    /// Setup the update requester thread which will trigger an update every 
    /// 1/30th of a second. This is needed since currently OrbTK doesn't 
    /// support auto redraw, so we need some event in the window to update all
    /// the data in the GUI (like clicking in the window). To avoid this we can
    /// call the ctx.request_sender().send(Update) function to request an 
    /// update (as if we were an event happening!).
    fn init(&mut self, _: &mut Registry, ctx: &mut Context) {
        let sender = ctx.request_sender();
        let exit_thread = Arc::clone(&self.exit_updater);

        self.updater_thread = Some(thread::spawn(move || {
            loop {
                sender.send(ShellRequest::Update).unwrap();
                thread::sleep(Duration::from_millis(33));

                // Check for exit request
                // TODO: Find some way of making this true...
                if *exit_thread.lock().unwrap() {
                    break;
                }
            }
        }));
    }

    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        self.current_time_utc = Utc::now();
        self.frame_counter += 1;

        // Update the virtspace's pipeline data
        if let Some(virt_space) = ctx
            .widget()
            .get_mut::<RenderPipeline>("render_pipeline")
            .0
            .as_any()
            .downcast_ref::<VirtSpacePipeline>() {
            
            virt_space.frame_counter.set(self.frame_counter);
        }

        // Update time value
        ctx.widget().set("current_time_text", String16::from(format!(
            "{} UTC, frame {}", 
            self.current_time_utc.format("%Y-%m-%d %H:%M:%S").to_string(),
            self.frame_counter)));
    }
}