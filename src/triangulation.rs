use std::collections::HashMap;

use turtle::Point;

use crate::TagData;

pub fn triangulate_coordinate(distance_between_anchors: f64, tag_to_anchor_1: f64, tag_to_anchor_2: f64) -> Point {
    let a = distance_between_anchors;
    let b = tag_to_anchor_1;
    let c = tag_to_anchor_2;
    
    let cos_a = ((b * b) + (c * c) - (a * a)) / (2.0 * b * c);
    let x = b * cos_a;
    let y = b * f64::sqrt(1.0 - (cos_a * cos_a));
    
    Point { x, y }
}

pub fn update_position(anchors: &HashMap<String, TagData>) {}