use gpui::prelude::FluentBuilder;
use gpui::{
    AnyElement, Context, EventEmitter, InteractiveElement, IntoElement, MouseButton, ParentElement,
    Render, StatefulInteractiveElement, Styled, Window, div, px,
};

pub struct DiffPaneClosed;

pub struct DiffPane {
    diff_content: String,
    title: String,
}

impl DiffPane {
    pub fn new(title: String, diff_content: String) -> Self {
        Self {
            diff_content,
            title,
        }
    }

    pub fn set_diff(&mut self, diff_content: String) {
        self.diff_content = diff_content;
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    fn on_close_clicked(
        &mut self,
        _event: &MouseButton,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.emit(DiffPaneClosed);
    }
}

impl EventEmitter<DiffPaneClosed> for DiffPane {}

impl Render for DiffPane {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let title = self.title.clone();

        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(gpui::rgb(0x1E1E1E))
            // Header
            .child(
                div()
                    .w_full()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .px(px(12.0))
                    .py(px(8.0))
                    .border_b_1()
                    .border_color(gpui::rgb(0x333333))
                    .bg(gpui::rgb(0x252525))
                    .child(
                        div()
                            .text_color(gpui::white())
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_size(px(14.0))
                            .child(title),
                    )
                    .child(
                        div()
                            .text_color(gpui::rgb(0x888888))
                            .text_size(px(16.0))
                            .px(px(8.0))
                            .py(px(4.0))
                            .cursor_pointer()
                            .hover(|style| style.bg(gpui::rgb(0x444444)))
                            .rounded(px(4.0))
                            .child("✕")
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    this.on_close_clicked(&MouseButton::Left, _window, cx);
                                }),
                            ),
                    ),
            )
            // Diff content
            .child(
                div()
                    .flex_1()
                    .id("diff_content")
                    .overflow_scroll()
                    .bg(gpui::rgb(0x1E1E1E))
                    .flex()
                    .flex_col()
                    .px(px(8.0))
                    .py(px(4.0))
                    .child(
                        div()
                            .text_color(gpui::rgb(0xCCCCCC))
                            .font_family("monospace")
                            .text_size(px(12.0))
                            .child(self.diff_content.clone()),
                    ),
            )
    }
}
