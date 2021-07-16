//! `iced_graphics` renderer for tick marks

use super::PrimitiveCache;
use crate::core::Normal;
use crate::native::tick_marks;
use crate::style::tick_marks::{Placement, Shape, Style};
use iced_graphics::{Background, Color, Primitive, Rectangle};

fn draw_vertical_lines(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_y: f32,
    bounds_height: f32,
    x: f32,
    width: f32,
    length: f32,
    color: Color,
    inverse: bool,
) {
    let start_y = bounds_y - (f32::from(width) / 2.0);
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: (start_y + tick_mark.scale(bounds_height)),
                    width: f32::from(length),
                    height: f32::from(width),
                },
                background: back_color.clone(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: (start_y + tick_mark.scale_inv(bounds_height)),
                    width: f32::from(length),
                    height: f32::from(width),
                },
                background: back_color.clone(),
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

fn draw_vertical_circles(
    primitives: &mut Vec<Primitive>,
    tick_marks: &[Normal],
    bounds_y: f32,
    bounds_height: f32,
    x: f32,
    diameter: f32,
    color: Color,
    inverse: bool,
) {
    let diameter = f32::from(diameter);
    let radius = diameter / 2.0;
    let start_y = bounds_y - f32::from(radius);
    let back_color = Background::Color(color);

    if inverse {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: (start_y + tick_mark.scale(bounds_height)),
                    width: diameter,
                    height: diameter,
                },
                background: back_color.clone(),
                border_radius: radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    } else {
        for tick_mark in tick_marks {
            primitives.push(Primitive::Quad {
                bounds: Rectangle {
                    x,
                    y: (start_y + tick_mark.scale_inv(bounds_height)),
                    width: diameter,
                    height: diameter,
                },
                background: back_color.clone(),
                border_radius: radius,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            });
        }
    }
}

#[inline]
fn draw_vertical_left_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => return,
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_vertical_lines(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_vertical_circles(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_left_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_vertical_left_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_vertical_left_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_vertical_left_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_vertical_right_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => return,
            Shape::Line {
                length,
                width,
                color,
            } => {
                draw_vertical_lines(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x - f32::from(*length),
                    *width,
                    *length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                draw_vertical_circles(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x - f32::from(*diameter),
                    *diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_right_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    inverse: bool,
) {
    draw_vertical_right_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        inverse,
    );
    draw_vertical_right_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        inverse,
    );
    draw_vertical_right_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        inverse,
    );
}

#[inline]
fn draw_vertical_center_aligned_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => return,
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (x, length) = if fill_length {
                    (
                        bounds.x + f32::from(*length),
                        bounds.width - (f32::from(*length) * 2.0),
                    )
                } else {
                    (x - (*length / 2.0), *length)
                };

                draw_vertical_lines(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (x, diameter) = if fill_length {
                    (
                        bounds.x + f32::from(*diameter),
                        bounds.width - (f32::from(*diameter) * 2.0),
                    )
                } else {
                    (x - (*diameter / 2.0), *diameter)
                };

                draw_vertical_circles(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    x,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_center_aligned(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    inverse: bool,
) {
    draw_vertical_center_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        inverse,
    );
    draw_vertical_center_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        inverse,
    );
    draw_vertical_center_aligned_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        inverse,
    );
}

#[inline]
fn draw_vertical_center_aligned_split_tier(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: Option<&Vec<Normal>>,
    shape: &Shape,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    if let Some(tick_marks) = tick_marks {
        match shape {
            Shape::None => return,
            Shape::Line {
                length,
                width,
                color,
            } => {
                let (left_x, length) = if fill_length {
                    let length = *length + ((bounds.width + gap) / 2.0);
                    (x - length - (gap / 2.0), length)
                } else {
                    (x - *length - (gap / 2.0), *length)
                };

                let right_x = x + (gap / 2.0);

                draw_vertical_lines(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    left_x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
                draw_vertical_lines(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    right_x,
                    *width,
                    length,
                    *color,
                    inverse,
                );
            }
            Shape::Circle { diameter, color } => {
                let (left_x, diameter) = if fill_length {
                    (
                        bounds.x - f32::from(*diameter),
                        *diameter + ((bounds.width + gap) / 2.0),
                    )
                } else {
                    (x - *diameter - (gap / 2.0), *diameter)
                };

                let right_x = x + (gap / 2.0);

                draw_vertical_circles(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    left_x,
                    diameter,
                    *color,
                    inverse,
                );
                draw_vertical_circles(
                    primitives,
                    tick_marks,
                    bounds.y,
                    bounds.height,
                    right_x,
                    diameter,
                    *color,
                    inverse,
                );
            }
        }
    }
}

fn draw_vertical_center_aligned_split(
    primitives: &mut Vec<Primitive>,
    bounds: &Rectangle,
    x: f32,
    tick_marks: &tick_marks::Group,
    style: &Style,
    fill_length: bool,
    gap: f32,
    inverse: bool,
) {
    draw_vertical_center_aligned_split_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_1(),
        &style.tier_1,
        fill_length,
        gap,
        inverse,
    );
    draw_vertical_center_aligned_split_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_2(),
        &style.tier_2,
        fill_length,
        gap,
        inverse,
    );
    draw_vertical_center_aligned_split_tier(
        primitives,
        bounds,
        x,
        tick_marks.tier_3(),
        &style.tier_3,
        fill_length,
        gap,
        inverse,
    );
}

/// Draws tick marks on a vertical axis.
///
/// * bounds - The bounds of the widget to place the tick marks in/outside of.
/// * tick_marks - The group of tick marks.
/// * style - The tick marks style.
/// * placement - The placement of the tick marks relative to the bounds.
/// * inverse - Whether to inverse the positions of the tick marks (true) or
/// not (false).
pub fn draw_vertical_tick_marks(
    bounds: &Rectangle,
    tick_marks: &tick_marks::Group,
    style: &Style,
    placement: &Placement,
    inverse: bool,
    cache: &PrimitiveCache,
) -> Primitive {
    cache.cached_linear(
        *bounds,
        tick_marks,
        *style,
        *placement,
        inverse,
        || {
            let primitives = match placement {
                Placement::BothSides { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len() * 2);

                    if *inside {
                        draw_vertical_left_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                        draw_vertical_right_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_vertical_right_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                        draw_vertical_left_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::LeftOrTop { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    if *inside {
                        draw_vertical_left_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_vertical_right_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::RightOrBottom { offset, inside } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    if *inside {
                        draw_vertical_right_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    } else {
                        draw_vertical_left_aligned(
                            &mut primitives,
                            &bounds,
                            bounds.x + bounds.width,
                            tick_marks,
                            style,
                            inverse,
                        );
                    }

                    primitives
                }
                Placement::Center {
                    offset,
                    fill_length,
                } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len());

                    draw_vertical_center_aligned(
                        &mut primitives,
                        &bounds,
                        bounds.center_x(),
                        tick_marks,
                        style,
                        *fill_length,
                        inverse,
                    );

                    primitives
                }
                Placement::CenterSplit {
                    offset,
                    fill_length,
                    gap,
                } => {
                    let bounds = offset.offset_rect(bounds);

                    let mut primitives: Vec<Primitive> =
                        Vec::with_capacity(tick_marks.len() * 2);

                    draw_vertical_center_aligned_split(
                        &mut primitives,
                        &bounds,
                        bounds.center_x(),
                        tick_marks,
                        style,
                        *fill_length,
                        f32::from(*gap),
                        inverse,
                    );

                    primitives
                }
            };

            Primitive::Group { primitives }
        },
    )
}
