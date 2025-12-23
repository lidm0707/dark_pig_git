use std::collections::HashMap;

use git2::Oid;
use gpui::{
    Context, IntoElement, ParentElement, PathBuilder, Pixels, Point, Render, Styled, Window,
    canvas, div, px,
};

#[derive(Debug, Clone)]
pub struct Edge {
    pub from: Point<Pixels>,
    pub to: Point<Pixels>,
}

impl Edge {
    pub fn new(x: Pixels, y: Pixels) -> Self {
        Self {
            from: Point::new(x, y),
            to: Point::new(0.0.into(), 0.0.into()),
        }
    }
}
#[derive(Debug, Clone, Default)]
pub struct EdgeManager {
    pub edges: Vec<Edge>,
}

impl EdgeManager {
    pub fn new() -> Self {
        Self { edges: Vec::new() }
    }

    pub fn add(&mut self, from: Point<Pixels>, to: Point<Pixels>) {
        self.edges.push(Edge { from, to });
    }
}

impl Render for EdgeManager {
    fn render(&mut self, window: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        window.request_animation_frame();

        let mut lines = Vec::new();
        for edge in &self.edges {
            let mut builder = PathBuilder::stroke(px(1.5));

            builder.move_to(edge.from);
            builder.line_to(edge.to);

            let line = builder.build().unwrap();
            lines.push(line);
        }
        div().size_full().child(
            canvas(
                move |_, _, _| {},
                move |_, _, window, _| {
                    for path in lines {
                        window.paint_path(path, gpui::white());
                    }
                },
            )
            .size_full(),
        )
    }
}
