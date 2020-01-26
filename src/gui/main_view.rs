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