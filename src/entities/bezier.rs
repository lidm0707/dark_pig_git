use gpui::{Path, PathBuilder, PathStyle, Pixels, Point, StrokeOptions, point, px};
use lyon::path::LineCap;

/// Create a bezier curve path for connecting two commit nodes
pub fn create_bezier_edge(
    start: Point<Pixels>,
    end: Point<Pixels>,
    stroke_width: f32,
) -> Path<Pixels> {
    // Calculate the horizontal distance to determine curve shape
    let dx = (end.x - start.x) * 0.5;

    // Control points for cubic bezier curve
    // First control point: starts horizontally from the start point
    let c1 = point(start.x + dx, start.y);

    // Second control point: approaches the end point horizontally
    let c2 = point(end.x - dx, end.y);

    // Create stroke options
    let options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_line_cap(LineCap::Round)
        .with_line_join(lyon::path::LineJoin::Round);

    // Build the path
    let mut builder = PathBuilder::stroke(px(stroke_width)).with_style(PathStyle::Stroke(options));

    // Start at the source point
    builder.move_to(start);

    // Create cubic bezier curve to the destination
    builder.cubic_bezier_to(c1, c2, end);

    // Build and return the path
    builder.build().unwrap()
}

/// Create a more complex bezier curve for merges and branches
pub fn create_complex_bezier_edge(
    start: Point<Pixels>,
    end: Point<Pixels>,
    control_points: (Point<Pixels>, Point<Pixels>),
    stroke_width: f32,
) -> Path<Pixels> {
    // Create stroke options
    let options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_line_cap(LineCap::Round)
        .with_line_join(lyon::path::LineJoin::Round);

    // Build the path
    let mut builder = PathBuilder::stroke(px(stroke_width)).with_style(PathStyle::Stroke(options));

    // Start at the source point
    builder.move_to(start);

    // Create cubic bezier curve using the provided control points
    builder.cubic_bezier_to(control_points.0, control_points.1, end);

    // Build and return the path
    builder.build().unwrap()
}

/// Create a vertical straight edge (for commits in the same lane)
pub fn create_vertical_edge(
    start: Point<Pixels>,
    end: Point<Pixels>,
    stroke_width: f32,
) -> Path<Pixels> {
    // Create stroke options
    let options = StrokeOptions::default()
        .with_line_width(stroke_width)
        .with_line_cap(LineCap::Round)
        .with_line_join(lyon::path::LineJoin::Round);

    // Build the path
    let mut builder = PathBuilder::stroke(px(stroke_width)).with_style(PathStyle::Stroke(options));

    // Start at the source point
    builder.move_to(start);

    // Create a straight line to the destination
    builder.line_to(end);

    // Build and return the path
    builder.build().unwrap()
}

/// Helper function to calculate the center position of a commit node
pub fn calculate_node_center(node_x: f32, node_y: f32, node_size: f32) -> Point<Pixels> {
    point(px(node_x + node_size / 2.0), px(node_y + node_size / 2.0))
}

/// Helper function to calculate the connection point on a node's edge
/// Angle is in radians, with 0 being to the right, PI/2 being down, etc.
pub fn calculate_connection_point(
    node_x: f32,
    node_y: f32,
    node_size: f32,
    angle: f32,
) -> Point<Pixels> {
    // Calculate the center of the node
    let center_x = node_x + node_size / 2.0;
    let center_y = node_y + node_size / 2.0;

    // Calculate the point on the edge based on angle
    let radius = node_size / 2.0;
    let x = center_x + radius * angle.cos();
    let y = center_y + radius * angle.sin();

    point(px(x), px(y))
}
