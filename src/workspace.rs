use gpui::{AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div};

use crate::garph::Garph;
use crate::title::TitleBar;

pub struct Dock;
pub struct Pane;
pub struct Workspace {
    dock: Option<Entity<Garph>>,
    title_bar: Entity<TitleBar>,
    // pane: Vec<Entity<AnyElement>>,
}

impl Workspace {
    pub fn new(dock: Option<Entity<Garph>>, cx: &mut Context<Self>) -> Self {
        Self {
            dock,
            title_bar: cx.new(|_| TitleBar::new("Dark Pig Git")),
        }
    }

    pub fn set_title(&mut self, title: &str, cx: &mut Context<Self>) {
        let title = title.to_string();
        self.title_bar
            .update(cx, |title_bar, _| title_bar.set_title(title));
    }

    // pub fn add_pane(&mut self, pane: Entity<AnyElement>) {
    //     self.pane.push(pane);
    // }

    // pub fn remove_pane(&mut self, index: usize) {
    //     self.pane.remove(index);
    // }

    // pub fn remove_all_panes(&mut self) {
    //     self.pane.clear();
    // }
}

impl Render for Workspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let dock = self.dock.clone().unwrap();
        let title_bar = self.title_bar.clone();
        let result = div()
            .size_full()
            .relative()
            .flex()
            .flex_col()
            .child(title_bar)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .child(div().w(gpui::px(300.0)).h_full().child(dock))
                    .child(div().flex_1().bg(gpui::white()).text_2xl().child(
                        "test Workspace \n test Workspace \n test Workspace \n test Workspace \n ",
                    )),
            );

        result
    }
}
