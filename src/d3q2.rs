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
    
    let mut fewest: i32 = -1;
    let mut i = 1;
    let mut j = 1;
    for p in wire_1.on.iter_mut() {
        for q in wire_2.on.iter_mut() { 
            j += 1;
            //println!("p: {}.{} | q: {}.{} _ steps: {}, {}", p.x, p.y, q.x, q.y, i, j);
            if p.x == q.x && p.y == q.y {
                let steps = i as i32 + j as i32;

                println!("hit! at {}.{} steps: p{} q{}", p.x, p.y, i as i32, j as i32);
                
                if fewest < 0 {
                    fewest = steps;
                } else if steps < fewest {
                    fewest = steps;
                }
            }
        }
        i += 1;
        j = 0;
    }

    Ok(fewest)
}

fn update_location(w: &mut Wire, index: usize) {
    let step = w.path[index];
    let direction = step.chars().nth(0).unwrap();
    let magnitude = step[1..].parse::<i32>().unwrap();
       
    let mut steps_added = 0;
    match direction {
        // up
        'U' => {
            println!("up");
            for y in (w.curr.y + 1)..(w.curr.y + (magnitude + 1)) { steps_added += 1;

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
            println!("down");
            for y in ((w.curr.y - magnitude)..(w.curr.y)).rev() {
                steps_added += 1;
                let point = Point{
                    x: w.curr.x,
                    y: y,
                };
               
                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
                w.curr = point;
            }
        },

        // left
        'L' => {
            println!("left");
            for x in ((w.curr.x - magnitude)..(w.curr.x)).rev() {
                steps_added += 1;
                let point = Point{
                    x: x,
                    y: w.curr.y,
                };

                //println!("marking {}.{}", point.x, point.y);
                w.on.push(point);
                w.curr = point;
            }
        },

        // right
        'R' => {
            println!("right");
            for x in (w.curr.x + 1)..(w.curr.x + (magnitude + 1)) {
                steps_added += 1;
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
    
    println!("Added {} steps", steps_added);
}

