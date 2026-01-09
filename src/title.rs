use gpui::{
    Context, EventEmitter, InteractiveElement, IntoElement, MouseButton, ParentElement, Render,
    SharedString, Styled, Window, actions, div, px,
};

actions!(work, [Quit]);

pub struct TitleBar {
    title: SharedString,
}

#[derive(Clone, Copy)]
pub struct Event;

impl TitleBar {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
        }
    }

    pub fn set_title(&mut self, title: impl Into<SharedString>) {
        self.title = title.into();
    }
}

impl EventEmitter<Event> for TitleBar {}

impl Render for TitleBar {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let quit_text = "Quit".to_string();

        div()
            .w_full()
            .h(px(32.0))
            .flex()
            .items_center()
            .justify_between()
            .px_4()
            .bg(gpui::rgb(0x1e1e1e))
            .text_color(gpui::rgb(0xffffff))
            .text_sm()
            .font_weight(gpui::FontWeight::MEDIUM)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(self.title.clone()),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(gpui::rgb(0xff5f57)),
                            )
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(gpui::rgb(0xfebc2e)),
                            )
                            .child(
                                div()
                                    .w(px(12.0))
                                    .h(px(12.0))
                                    .rounded_full()
                                    .bg(gpui::rgb(0x28c840)),
                            ),
                    )
                    .child(
                        div()
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(gpui::rgb(0xff5f57))
                            .text_color(gpui::rgb(0xffffff))
                            .cursor_pointer()
                            .child(quit_text)
                            .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                                cx.quit();
                            }),
                    ),
            )
    }
}
