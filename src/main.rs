// #![windows_subsystem = "windows"]

// use druid::widget::{Controller, CrossAxisAlignment, Painter};
// use druid::widget::{Flex, Label};
// use druid::{
//     theme, AppLauncher, Color, Data, Env, Event, EventCtx, Lens, RenderContext, Widget, WidgetExt,
//     WindowDesc,
// };

// use chrono::Utc;
// use chrono_tz::America::New_York;

// use druid::widget::ControllerHost;
// use std::time::Duration;

// #[derive(Clone, Data, Lens)]
// struct TimeState {
//     time: String,
// }

// #[derive(Clone, Data, Lens)]
// struct TimeState2 {
//     time: String,
// }


// #[derive(Clone, Data)]
// struct TimeController;

// #[derive(Clone, Data)]
// struct TimeController2;

// impl<W: Widget<TimeState>> Controller<TimeState, W> for TimeController {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut EventCtx,
//         event: &Event,
//         data: &mut TimeState,
//         env: &Env,
//     ) {
//         match event {
//             Event::WindowConnected => {
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             Event::Timer(timer_id) => {
//                 data.time = data.get_time();
//                 ctx.request_paint();
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             _ => {}
//         }
//         child.event(ctx, event, data, env);
//     }
// }

// impl<W: Widget<TimeState2>> Controller<TimeState2, W> for TimeController2 {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut EventCtx,
//         event: &Event,
//         data: &mut TimeState2,
//         env: &Env,
//     ) {
//         match event {
//             Event::WindowConnected => {
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             Event::Timer(timer_id) => {
//                 data.time = data.get_time();
//                 ctx.request_paint();
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             _ => {}
//         }
//         child.event(ctx, event, data, env);
//     }
// }

// fn flex_row<T: Data>(
//     w1: impl Widget<T> + 'static,
//     w2: impl Widget<T> + 'static,
//     // w3: impl Widget<T> + 'static,
//     // w4: impl Widget<T> + 'static,
// ) -> impl Widget<T> {
//     Flex::row()
//         .with_flex_child(w1, 1.0)
//         .with_spacer(1.0)
//         .with_flex_child(w2, 1.0)
//     // .with_spacer(1.0)
//     // .with_flex_child(w3, 1.0)
//     // .with_spacer(1.0)
//     // .with_flex_child(w4, 1.0)
// }

// fn build_root_widget() -> impl Widget<TimeState> {
//     let display = Label::new(|data: &String, _env: &_| data.clone())
//         .with_text_size(24.0)
//         .lens(TimeState::time)
//         .padding(5.0);

//     let display = ControllerHost::new(display, TimeController);
    
//     let display2 = Label::new(|data: &String, _env: &_| data.clone())
//         .with_text_size(24.0)
//         .lens(TimeState2::time)
//         .padding(5.0);

//     let display2 = ControllerHost::new(display2, TimeController2);

//     Flex::row()
//         .with_flex_spacer(0.2)
//         .with_child(display)
//         .with_flex_spacer(0.2)
//         .with_child(display2)
// }

// impl TimeState {
//     fn get_time(&mut self) -> String {
//         Utc::now().with_timezone(&New_York).to_string()
//     }
// }


// impl TimeState2 {
//     fn get_time(&mut self) -> String {
//         Utc::now().with_timezone(&New_York).to_string()
//     }
// }

// pub fn main() {
//     // describe the main window
//     let main_window = WindowDesc::new(build_root_widget())
//         .title("NewYork_Time: EST!")
//         .window_size((349.0, 200.0))
//         .resizable(false);

//     // create the initial app state
//     let initial_state: TimeState = TimeState {
//         time: "".to_string(),
//         // }

//         // let TimeState2 = TimeState2 {
//         // time: "".to_string(),
//         // };
//     };

//     // start the application. Here we pass in the application state.
//     AppLauncher::with_window(main_window)
//         .log_to_console()
//         .launch(initial_state)
//         .expect("Failed to launch application");
// }



#![windows_subsystem = "windows"]

use druid::widget::{Controller, CrossAxisAlignment, Painter};
use druid::widget::{Flex, Label};
use druid::{
    theme, AppLauncher, Color, Data, Env, Event, EventCtx, Lens, RenderContext, Widget, WidgetExt,
    WindowDesc,
};

use chrono::Utc;
use chrono_tz::America::New_York;
use chrono_tz::Asia::Seoul;

use druid::widget::ControllerHost;
use std::time::Duration;

#[derive(Clone, Data, Lens)]
struct TimeState {
    time_ny: String,
    time_kr: String,
}

impl TimeState {
    fn get_time(&mut self) -> String {
        let raw_ny_time = Utc::now().with_timezone(&New_York).to_string();
        return format!("NewYork_Time: \n {:.19}", raw_ny_time);
    }

    fn get_time_kr(&mut self) -> String {
        let raw_kr_time = Utc::now().with_timezone(&Seoul).to_string();
        return format!("_Korea_Time: \n {:.19}", raw_kr_time);
    }
}

fn flex_row<T: Data>(
    w1: impl Widget<T> + 'static,
    w2: impl Widget<T> + 'static,
) -> impl Widget<T> {
    Flex::row()
        .with_flex_child(w1, 1.0)
        .with_spacer(1.0)
        .with_flex_child(w2, 1.0)
}

fn build_root_widget() -> impl Widget<TimeState> {
    let display_ny = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(24.0)
        .lens(TimeState::time_ny)
        .padding(5.0);

    let display_kr = Label::new(|data: &String, _env: &_| data.clone())
        .with_text_size(24.0)
        .lens(TimeState::time_kr)
        .padding(5.0);

    Flex::row()
        .with_flex_spacer(0.2)
        .with_child(display_ny)
        .with_flex_spacer(0.2)
        .with_child(display_kr)
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST! Korean_Time: KST!")
        .window_size((349.0, 200.0))
        .resizable(false);

    // create the initial app state
    let initial_state = TimeState {
        time_ny: "".to_string(),
        time_kr: "".to_string(),
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}