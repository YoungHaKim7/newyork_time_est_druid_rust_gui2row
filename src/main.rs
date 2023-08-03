#![windows_subsystem = "windows"]

use druid::widget::{Controller, CrossAxisAlignment, Painter};
use druid::widget::{Flex, Label};
use druid::{
    theme, AppLauncher, Color, Data, Env, Event, EventCtx, Lens, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use chrono::Utc;
use chrono_tz::America::New_York;

use druid::widget::ControllerHost;
use std::time::Duration;

#[derive(Clone, Data, Lens)]
struct TimeState {
    time: String,
}

#[derive(Clone, Data)]
struct TimeController;

impl<W: Widget<TimeState>> Controller<TimeState, W> for TimeController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut TimeState,
        env: &Env,
    ) {
        match event {
            Event::WindowConnected => {
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            Event::Timer(timer_id) => {
                data.time = data.get_time();
                ctx.request_paint();
                let timer_id = ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            _ => {}
        }
        child.event(ctx, event, data, env);
    }
}

fn op_button_label(op: char, label: String) -> impl Widget<TimeState> {
    let painter = Painter::new(|ctx, _, env| {
        let bounds = ctx.size().to_rect();

        ctx.fill(bounds, &env.get(theme::PRIMARY_DARK));

        if ctx.is_hot() {
            ctx.stroke(bounds.inset(-0.5), &Color::WHITE, 1.0);
        }

        if ctx.is_active() {
            ctx.fill(bounds, &env.get(theme::PRIMARY_LIGHT));
        }
    });

    Label::new(label)
        .with_text_size(24.)
        .center()
        .background(painter)
        .expand()
}

fn op_button(op: char) -> impl Widget<TimeState> {
    op_button_label(op.clone(), op.to_string())
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
    // w3: impl Widget<T> + 'static,
    // w4: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
    // .with_spacer(1.0)
    // .with_flex_child(w3, 1.0)
    // .with_spacer(1.0)
    // .with_flex_child(w4, 1.0)
}

fn build_root_widget() -> impl Widget<TimeState> {
    let display = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(24.0)
        .lens(TimeState::time)
        .padding(5.0);

    let display = ControllerHost::new(display, TimeController);

    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display)
        .with_flex_spacer(0.2)
        .cross_axis_alignment(CrossAxisAlignment::End)
        .with_flex_child(
            flex_row(op_button_label('c', "CE".to_string()), op_button('C')),
            1.0,
        )
}

impl TimeState {
    fn get_time(&mut self) -> String {
        Utc::now().with_timezone(&New_York).to_string()
    }
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((349.0, 200.0))
        .resizable(false);

    // create the initial app state
    let initial_state: TimeState = TimeState {
        time: "".to_string(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
