use orbtk::{prelude::*, shell::ShellRequest, theme::DEFAULT_THEME_CSS};
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Utc};
use std::time::Duration;
use std::thread;
use crate::virtspace::pipeline::VirtSpacePipeline;

// ---------------------------------------------------------------------------
// THEME IMPORTS
// ---------------------------------------------------------------------------

static THEME_EXT: &'static str = include_str!("../res/theme.css");

// TODO: Figure out how to use this
#[allow(dead_code)]
const FONT_ROBOTO_BOLD: &[u8] = include_bytes!("../res/Roboto-Bold.ttf");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(THEME_EXT)
        .build()
}

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

        if let Some(virt_space) = ctx
            .widget()
            .get_mut::<RenderPipeline>("render_pipeline")
            .0
            .as_any()
            .downcast_ref::<VirtSpacePipeline>() {
            
            virt_space.frame_counter.set(self.frame_counter);
        }

        ctx.widget().set("current_time_text", String16::from(format!(
            "{} UTC, frame {}", 
            self.current_time_utc.format("%Y-%m-%d %H:%M:%S").to_string(),
            self.frame_counter)));
    }
}


// ---------------------------------------------------------------------------
// ORBTK MAIN WIDGET
// ---------------------------------------------------------------------------

widget!(
    MainView<GuiState> {
        current_time_text: String16,
        render_pipeline: RenderPipeline
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .render_pipeline(RenderPipeline(Box::new(VirtSpacePipeline::default())))
            .child(
                Grid::create()
                    .columns(
                        Columns::create()
                            .repeat(350.0, 2)
                            .column(700.0)
                            .build()
                    )
                    .rows(
                        Rows::create()
                            .row(600.0)
                            .row(200.0)
                            .build()
                    )
                    .child(Stack::create()
                        .selector("container")
                        .orientation("vertical")
                        .margin((8.0, 8.0, 8.0, 8.0))
                        .attach(Grid::column(0))
                        .child(TextBlock::create()
                            .selector(Selector::from("text-block")
                                .class("time"))
                            .text(("current_time_text", id))
                            .build(ctx))
                        .child(TextBlock::create()
                            .selector(Selector::from("text-block")
                                .class("header"))
                            .text("TM: Telemetry")
                            .build(ctx))
                        .build(ctx))
                    .child(Canvas::create()
                        .selector("virtspace")
                        .attach(Grid::column(1))
                        .attach(Grid::column_span(2))
                        .render_pipeline(id)
                        .build(ctx))
                    .build(ctx)
            )
    }
}

pub fn start() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("AutonomyControl")
                .position((200.0, 200.0))
                .size(1400.0, 800.0)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}