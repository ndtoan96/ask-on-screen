use dioxus::{
    desktop::{Config, LogicalPosition, LogicalSize, WindowBuilder},
    prelude::*,
};
use xcap::Monitor;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
// const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    let monitors = Monitor::all().unwrap();
    let x = monitors.iter().map(|m| m.x()).min().unwrap();
    let y = monitors.iter().map(|m| m.y()).min().unwrap();
    let width = monitors
        .iter()
        .map(|m| m.x() + m.width() as i32 - x)
        .max()
        .unwrap() as u32;
    let height = monitors
        .iter()
        .map(|m| m.y() + m.height() as i32 - y)
        .max()
        .unwrap() as u32;
    dbg!(x, y, width, height);
    let config = Config::new().with_window(
        WindowBuilder::new()
            .with_transparent(true)
            .with_background_color((0, 0, 0, 0))
            .with_always_on_top(true)
            .with_position(LogicalPosition::new(x, y))
            .with_inner_size(LogicalSize::new(width, height))
            .with_decorations(false),
    );
    dioxus::LaunchBuilder::desktop()
        .with_cfg(config)
        .launch(App);
}

#[component]
fn App() -> Element {
    let mut cursor = use_signal(|| None);
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        div {
            onmousemove: move |event| {
                cursor.set(Some(event.client_coordinates()));
            },
            "Cursor position: {cursor:?}"
        }
    }
}
