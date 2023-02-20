use std::collections::HashMap;
use turtle::{Point, Turtle, Drawing};
use crate::TagData;

pub struct Triangulator {
    point: Point,
    drawing: Drawing,
    turtle: Turtle,
    anchors: HashMap<String, TagData>,
    anchor_names: Vec<String>,
    anchor_dist: f64,
}

impl Triangulator {
    pub fn init(&mut self) {
        self.turtle.pen_up();
        self.turtle.go_to(self.point);
        
        self.turtle.pen_down();
        
        self.turtle.go_to(Point { x: self.point.x * -1.0, y: self.point.y });
        self.turtle.go_to(Point { x: self.point.x * -1.0, y: self.point.y * -1.0 });
        self.turtle.go_to(Point { x: self.point.x, y: self.point.y * -1.0 });
        self.turtle.go_to(self.point);
        
        self.turtle.pen_up();
    }
    
    fn triangulate_coordinates(&self) -> Option<Point> {
        let anchor_1 = self.anchors.get(self.anchor_names.get(0).unwrap()).unwrap();
        let anchor_2 = self.anchors.get(self.anchor_names.get(1).unwrap()).unwrap();
        
        if anchor_1.range < 0.0 || anchor_2.range < 0.0 || anchor_1.range + anchor_2.range <= self.anchor_dist {
            return None;
        }
 
        let a = self.anchor_dist;
        let b = anchor_1.range;
        let c = anchor_2.range;
        
        let cos_a = ((b * b) + (c * c) - (a * a)) / (2.0 * b * c);
        let x = b * cos_a;
        let y = b * f64::sqrt(1.0 - (cos_a * cos_a));
        
        Some(Point { x, y })
    }
    
    pub fn update_anchor_info(&mut self, tag_data: TagData) {
        if self.anchor_names.len() >= 2 {
            println!("Cannot use more than 2 anchors");
            return;
        }
        
        let tag_data_source = tag_data.source.clone();
        if let None = self.anchors.insert(tag_data.source.clone(), tag_data) {
            println!("Added New Anchor: {}", &tag_data_source);
            self.anchor_names.push(tag_data_source);
        }
        
        self.update_display();
    }
    
    fn update_display(&mut self) {
        if self.anchor_names.len() != 2 {
            return;
        }
        
        self.turtle.go_to(self.point);
        
        let position = self.triangulate_coordinates();
        let canvas_size = self.drawing.size();
        let scale = (canvas_size.width - 200) as f64 / self.anchor_dist;
        
        match position {
            Some(pos) => {
                let x_offset = ((canvas_size.width - 200) as f64 / 2.0) * -1.0;
                let y_offset = (canvas_size.height - 200) as f64 / 2.0;
                
                let scaled_pos = Point {
                    x: x_offset + pos.x * scale,
                    y: y_offset - pos.y * scale,
                };
                
                self.turtle.go_to(scaled_pos);
                self.point = scaled_pos;
            },
            None => (),
        };
         
        println!("{:#?}", position);
    }
    
}

impl From<f64> for Triangulator {
    fn from(item: f64) -> Self {
        let mut drawing = Drawing::new();
        let mut turtle = drawing.add_turtle();
        turtle.set_speed("instant");
        
        let canvas_size = drawing.size();
        let x = ((canvas_size.width - 200) as f64 / 2.0) * -1.0;
        let y = (canvas_size.height - 200) as f64 / 2.0;
        let initial_point = Point { x, y };
        
        Triangulator {
            point: initial_point,
            drawing,
            turtle,
            anchors: HashMap::new(),
            anchor_names: Vec::new(),
            anchor_dist: item
        }
    }
}