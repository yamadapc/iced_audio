use super::PrimitiveCache;
use crate::native::text_marks;
use crate::style::text_marks::{Align, Placement, Style};

use iced_graphics::{alignment, Primitive, Rectangle};

fn draw_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    y: f32,
    text_marks: &text_marks::Group,
    style: &Style,
    inverse: bool,
    align: alignment::Vertical,
) {
    let color = style.color;
    let font = style.font;
    let text_size = f32::from(style.text_size);
    let text_bounds_width = f32::from(style.bounds_width);
    let text_bounds_height = f32::from(style.bounds_height);

    if inverse {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x: (bounds.x + (text_mark.0.scale_inv(bounds.width)))
                        .round(),
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: align,
            });
        }
    } else {
        for text_mark in &text_marks.group {
            primitives.push(Primitive::Text {
                content: text_mark.1.clone(),
                size: text_size,
                bounds: Rectangle {
                    x: (bounds.x + (text_mark.0.scale(bounds.width))).round(),
                    y,
                    width: text_bounds_width,
                    height: text_bounds_height,
                },
                color,
                font,
                horizontal_alignment: alignment::Horizontal::Center,
                vertical_alignment: align,
            });
        }
    }
}

/// Draws text marks on a horizontal axis.
///
/// * `bounds` - The bounds of the widget to place the text marks in/outside of.
/// * `text_marks` - The group of text marks.
/// * `style` - The text marks style.
/// * `placement` - The placement of the text marks relative to the bounds.
/// * `inverse` - Whether to inverse the positions of the text marks (true) or
/// not (false).
pub fn draw_horizontal_text_marks(
    bounds: &Rectangle,
    text_marks: &text_marks::Group,
    style: &Style,
    placement: &Placement,
    inverse: bool,
    cache: &PrimitiveCache,
) -> Primitive {
    cache.cached_linear(
        *bounds,
        text_marks,
        *style,
        *placement,
        inverse,
        || {
            let primitives = match placement {
                Placement::BothSides { inside, offset } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(text_marks.group.len() * 2);

                    if *inside {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Top,
                        );
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y + bounds.height,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Bottom,
                        );
                    } else {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Bottom,
                        );
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y + bounds.height,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Top,
                        );
                    }

                    primitives
                }
                Placement::LeftOrTop { inside, offset } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(text_marks.group.len());

                    if *inside {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Top,
                        );
                    } else {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Bottom,
                        );
                    }

                    primitives
                }
                Placement::RightOrBottom { inside, offset } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(text_marks.group.len());

                    if *inside {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y + bounds.height,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Bottom,
                        );
                    } else {
                        draw_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.y + bounds.height,
                            text_marks,
                            style,
                            inverse,
                            alignment::Vertical::Top,
                        );
                    }

                    primitives
                }
                Placement::Center { align, offset } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(text_marks.group.len());

                    match align {
                        Align::Start => {
                            draw_aligned(
                                &mut primitives,
                                &bounds,
                                bounds.center_y(),
                                text_marks,
                                style,
                                inverse,
                                alignment::Vertical::Top,
                            );
                        }
                        Align::End => {
                            draw_aligned(
                                &mut primitives,
                                &bounds,
                                bounds.center_y(),
                                text_marks,
                                style,
                                inverse,
                                alignment::Vertical::Bottom,
                            );
                        }
                        Align::Center => {
                            draw_aligned(
                                &mut primitives,
                                &bounds,
                                bounds.center_y(),
                                text_marks,
                                style,
                                inverse,
                                alignment::Vertical::Center,
                            );
                        }
                    }

                    primitives
                }
            };

            Primitive::Group { primitives }
        },
    )
}
