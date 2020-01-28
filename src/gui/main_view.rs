use orbtk::{prelude::*, theme::DEFAULT_THEME_CSS};
use crate::virtspace::VirtSpacePipeline;
use crate::gui::state::GuiState;

// ---------------------------------------------------------------------------
// THEME IMPORTS
// ---------------------------------------------------------------------------

static THEME_EXT: &'static str = include_str!("../../res/theme.css");

// TODO: Figure out how to use this
#[allow(dead_code)]
const FONT_ROBOTO_BOLD: &[u8] = include_bytes!("../../res/Roboto-Bold.ttf");

fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(THEME_EXT)
        .build()
}

// ---------------------------------------------------------------------------
// ORBTK MAIN WIDGET
// ---------------------------------------------------------------------------

const DEFAULT_WINDOW_WIDTH: f64 = 1600.0;
const DEFAULT_WINDOW_HEIGHT: f64 = 900.0;

widget!(
    MainView<GuiState> {
        current_time_text: String16,
        render_pipeline: RenderPipeline,
        window_width: f64,
        window_height: f64
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
                            .repeat(450.0, 2)
                            .column("stretch")
                            .build()
                    )
                    .rows(
                        Rows::create()
                            .row("stretch")
                            .row(300.0)
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
                    .child(Grid::create()
                        .columns(Columns::create()
                            .column("auto")
                            .column("stretch")
                            .build())
                        .rows(Rows::create()
                            .row("auto")
                            .row("stretch")
                            .row("auto")
                            .build())
                        .attach(Grid::column(0))
                        .attach(Grid::row(1))
                        .attach(Grid::column_span(2))
                        .child(TextBlock::create()
                            .attach(Grid::column(0))
                            .attach(Grid::row(0))
                            .selector(Selector::from("text-block").class("header"))
                            .margin((8.0, 8.0, 8.0, 8.0))
                            .text("TC: Telecommand")
                            .build(ctx))
                        .child(Button::create()
                            .selector(Selector::from("button").class("abort"))
                            .attach(Grid::column(0))
                            .attach(Grid::row(2))
                            .margin((8.0, 8.0, 8.0, 8.0))
                            .horizontal_alignment("start")
                            .vertical_alignment("center")
                            .text("Abort")
                            .build(ctx))
                        .child(Button::create()
                            .selector(Selector::from("button").class("send"))
                            .attach(Grid::column(1))
                            .attach(Grid::row(2))
                            .margin((8.0, 8.0, 8.0, 8.0))
                            .horizontal_alignment("end")
                            .vertical_alignment("center")
                            .text("Send")
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
                .size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
                .resizeable(true)
                .theme(get_theme())
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}