use gpui_kit::component::*;
use gpui_kit::*;
use lucide_gpui::{LucideAssets, LucideIcon};

struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(Icon::new(LucideIcon::Play).text_color(cx.theme().red))
            .child(Icon::new(LucideIcon::Armchair))
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(LucideAssets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
