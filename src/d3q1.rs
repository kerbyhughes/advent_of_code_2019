use std::fs::File;
use std::io::prelude::*;

struct Wire<'a> {
    curr: Point, 
    path: &'a mut Vec<&'a str>,
    on: &'a mut Vec<Point>, 
}

#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn calc_distance(&mut self) -> i32 {
        return self.x.abs() + self.y.abs()
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}


pub fn process(input_file: &str) -> Result<i32, &str> {
    let mut file = File::open(input_file)
        .expect("Couldn't open file");
    
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    let mut wire_1 = Wire{
        curr: Point{x: 0, y: 0}, 
        path: &mut Vec::new(),
        on: &mut Vec::new()
    };

    let mut wire_2 = Wire{
        curr: Point{x: 0, y: 0},
        path: &mut Vec::new(),
        on: &mut Vec::new()
    };

    let mut lines = Vec::new();
    for line in content.trim().split("\n") {
        lines.push(line);
    }

    for step in lines[0].split(",") {
       wire_1.path.push(step); 
    }

    for step in lines[1].split(",") {
       wire_2.path.push(step); 
    }

    for i in 0..wire_1.path.len() {
        update_location(&mut wire_1, i);
    }

    for i in 0..wire_2.path.len() {
        update_location(&mut wire_2, i);
    }
    
    let mut closest = -1;
    for mut p in wire_1.on.iter_mut() {
        if wire_2.on.contains(p) {
            let distance = p.calc_distance();

            if distance == 0 {
                continue;
            }

            println!("hit! at {}.{} distance:{}", p.x, p.y, distance);
            
            if closest < 0 {
                closest = distance
            } else if distance < closest {
                closest = distance
            }
        }
    }

    Ok(closest)
}

fn update_location(w: &mut Wire, index: usize) {
    let step = w.path[index];
    let direction = step.chars().nth(0).unwrap();
    let magnitude = step[1..].parse::<i32>().unwrap();
       
    match direction {
        // up
        'U' => {
            //println!("up");
            for y in w.curr.y..(w.curr.y + magnitude) {
                let point = Point{
                    x: w.curr.x,
                    y: y,
                };

                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
            }

            w.curr.y = w.curr.y + magnitude;
        },

        // down
        'D' => {
            //println!("down");
            for y in (w.curr.y - magnitude)..w.curr.y {
                let point = Point{
                    x: w.curr.x,
                    y: y,
                };
               
                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
            }

            w.curr.y = w.curr.y - magnitude;
        },

        // left
        'L' => {
            //println!("left");
            for x in (w.curr.x - magnitude)..w.curr.x {
                let point = Point{
                    x: x,
                    y: w.curr.y,
                };

                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
            }

            w.curr.x = w.curr.x - magnitude;
        },

        // right
        'R' => {
            //println!("right");
            for x in w.curr.x..(w.curr.x + magnitude) {
                let point = Point{
                    x: x,
                    y: w.curr.y,
                };

                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
            }

            w.curr.x = w.curr.x + magnitude;
        },

        _ => panic!("Can't parse direction! Got: {}", direction)
    }
}

