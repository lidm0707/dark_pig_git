use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, Render, StatefulInteractiveElement,
    Styled, Window, div, px,
};

use crate::entities::commit::CommitNode;
#[derive(Debug, Clone)]
pub struct Garph {
    pub nodes: Vec<CommitNode>,
    // pub edges: Vec<Edge>,
}

impl Garph {
    pub fn new(nodes: Vec<CommitNode>) -> Self {
        Garph { nodes }
    }

    pub fn create_node(&self, node: CommitNode) -> impl IntoElement {
        // div().absolute().bottom(px(node.position.0)).children([
        //     div()
        //         .bg(gpui::green())
        //         .border_color(gpui::black())
        //         .rounded(px(20.0))
        //         .size(px(10.0))
        //         .left(px(node.position.1)),
        //     div()
        //         .bg(gpui::red())
        //         .child(format!("{:?}", node.timestamp.seconds()))
        //         .left(px(node.position.1 + 10.0)),
        // ])
        div()
            .absolute()
            .left(px(node.position.1)) // 🔑 จุดเริ่มเดียวกัน
            .bottom(px(node.position.0))
            .flex_row()
            .gap(px(4.0))
            .children([
                // ===== box 1 : node =====
                div()
                    .w(px(80.0))
                    .bg(gpui::green())
                    .flex_col()
                    .border_color(gpui::black())
                    .rounded(px(20.0))
                    .size(px(10.0)),
                // ===== box 2 : time =====
            ])
    }

    fn create_row_node(&self, node: CommitNode) -> impl IntoElement {
        div()
            .bg(gpui::red())
            .flex_col()
            .flex_grow()
            .px(px(6.0))
            .py(px(2.0))
            .rounded(px(4.0))
            .child(format!("{:?}", node.timestamp.seconds()))
    }
}

impl Render for Garph {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let meta = self.clone();
        let space = 500.0;
        let rows = self
            .nodes
            .clone()
            .into_iter()
            .map(|node| {
                div()
                    .h(px(32.0)) // ความสูงต่อแถว
                    .flex()
                    .flex_row()
                    .children([
                        // ===== LEFT : node column =====
                        div()
                            .w(px(space / 3.0))
                            .mr(px(10.0))
                            .flex()
                            .items_center()
                            .child(meta.create_node(node.clone())),
                        // ===== RIGHT : time column =====
                        div()
                            .w(px(space / 1.0))
                            .flex()
                            .items_center()
                            .child(meta.create_row_node(node)),
                    ])
            })
            .collect::<Vec<_>>();

        div()
            .size(px(800.0))
            .bg(gpui::rgb(0x282828))
            .id("dag")
            .overflow_scroll()
            .child(
                div()
                    .w(px(480.0))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .children(rows),
            )
    }
}
