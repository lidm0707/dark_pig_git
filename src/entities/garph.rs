use std::mem::offset_of;

use gpui::{
    Context, InteractiveElement, IntoElement, ParentElement, PathBuilder, Render,
    StatefulInteractiveElement, Styled, Window, canvas, div, px,
};

use crate::entities::commit::CommitNode;
use crate::entities::edge::EdgeManager;

#[derive(Debug, Clone)]
pub struct Garph {
    pub nodes: Vec<CommitNode>,
    pub edge_manager: EdgeManager,
}

impl Garph {
    pub fn new(nodes: Vec<CommitNode>, edge_manager: EdgeManager) -> Self {
        Garph {
            nodes,
            edge_manager,
        }
    }

    pub fn create_node(&self, node: CommitNode) -> impl IntoElement {
        // Adjust positioning to match edge coordinates
        let x = node.position.0; // X position (from START_X minus commit height)
        let y = node.position.1; // Y position (based on lane)

        div()
            .absolute()
            .left(px(x)) // Scale lane position for better visibility
            .top(px(y)) // Adjusted Y position (inverted for proper display)
            .w(px(10.0))
            .h(px(10.0))
            .bg(gpui::green())
            .border_color(gpui::black())
            .rounded(px(5.0))
    }

    pub fn create_row_with_node(&self, node: CommitNode, index: usize) -> impl IntoElement {
        // Calculate the Y position to match the node position
        let y_pos = 800.0 - (index as f32 * 20.0); // Match the node Y position

        div()
            .absolute()
            .top(px(y_pos))
            .left(px(220.0)) // Position to the right of the graph
            .flex_row()
            .gap(px(10.0))
            .children([
                // Commit details
                div()
                    .bg(gpui::rgb(0x383838))
                    .min_w(px(600.0))
                    .px(px(10.0))
                    .py(px(5.0))
                    .rounded(px(4.0))
                    .child(
                        div().gap_1().children([
                            div()
                                .text_color(gpui::white())
                                .text_size(px(12.0))
                                .child(format!(
                                    "{}",
                                    node.message.split('\n').next().unwrap_or_default()
                                )),
                            div()
                                .text_color(gpui::rgb(0x969696))
                                .text_size(px(10.0))
                                .child(format!("{} - {}", node.author, node.timestamp.seconds())),
                        ]),
                    ),
            ])
    }
}

impl Render for Garph {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let edges = self.edge_manager.edges.clone();

        // Create a container that will handle scrolling for everything
        div()
            .size_full()
            .bg(gpui::rgb(0x282828))
            .id("dag")
            .overflow_scroll()
            .relative()
            .children([
                // Container that's larger than viewport to allow scrolling
                div().relative().w(px(2000.0)).h(px(2000.0)).children([
                    // Canvas for edges (same size as container)
                    div()
                        .relative()
                        .top(px(0.))
                        .left(px(0.))
                        .size_full()
                        .child(canvas(
                            move |_, _, _| {},
                            move |bounds, _, window, _| {
                                for edge in &edges {
                                    let offset = bounds.origin;
                                    let mut path = PathBuilder::stroke(px(1.5));
                                    path.move_to(edge.from + offset);
                                    path.line_to(edge.to + offset);

                                    if let Ok(p) = path.build() {
                                        window.paint_path(p, gpui::white());
                                    }
                                }
                            },
                        )),
                    // Nodes positioned absolutely within the container
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(0.))
                        .size_full()
                        .children(self.nodes.iter().map(|node| self.create_node(node.clone()))),
                    // Commit details in rows
                    div()
                        .absolute()
                        .top(px(0.))
                        .left(px(220.0))
                        .size_full()
                        .children(
                            self.nodes
                                .iter()
                                .enumerate()
                                .map(|(i, node)| self.create_row_with_node(node.clone(), i)),
                        ),
                ]),
            ])
    }
}
