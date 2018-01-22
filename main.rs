use std::io;
use std::f32;

#[derive(Debug, Clone, Copy)]
struct LineSegment {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl LineSegment {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> LineSegment {
        LineSegment{
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        }
    }
}
#[derive(Clone, Copy)]
struct Beam {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    new_x: f32,
    new_y: f32,
}

impl Beam {
    fn new(x1: f32, y1: f32, x2: f32, y2: f32) -> Beam {
        Beam{
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            new_x: 0.0,
            new_y: 0.0,
        }
    }
}

fn max (a: f32, b: f32) -> f32{
    if a > b {
        return a;
    } else {
        return b;
    }
}
fn min (a: f32, b: f32) -> f32{
    if a > b {
        return b;
    } else {
        return a;
    }
}

fn is_crossing(mut beam: Beam, seg: LineSegment) -> (bool, f32){
    let a1 = (beam.y1 - beam.y2) / (beam.x1 - beam.x2);
    let b1 = beam.y1 - a1 * beam.x1;
    beam.new_x = max(seg.x1, seg.x2);
    beam.new_y = beam.new_x * a1 + b1;
    let a2 = (seg.y1 - seg.y2) / (seg.x1 - seg.x2);
    let b2 = seg.y1 - a2 * seg.x1;

    if (beam.x1 - beam.x2 == 0.0) && (seg.x1 - seg.x2 == 0.0) {
        if ((beam.y1 >= seg.y1) && (beam.y1 <= seg.y2)) || ((beam.y1 <= seg.y1) && (beam.y1 >= seg.y2)) {
            return (true, 0.0);
        }

        if beam.x1 == seg.x1 {
            if !( (beam.y1 > max(seg.y1, seg.y2)) || (beam.y1 < min(seg.y1, seg.y2) )  
            && ((beam.y1*seg.y1 + beam.y2*seg.y2) < 0.0 ) )  {
                let res = if (beam.y1 - seg.y1).abs() < (beam.y1 - seg.y2).abs() {(beam.y1 - seg.y1).abs()} else {(beam.y1 - seg.y2).abs()} ;
                return (true, res);
            } 
        }

        return (false, 0.0);
    }   

    if beam.x1 - beam.x2 == 0.0 {
        let xa = beam.x1;
        let ya = a2 * xa + b2;
        if (beam.y2 - beam.y1 > 0.0) && (ya > beam.y1){
            return (true, ya - beam.y1)
        }
        if (beam.y2 - beam.y1) < 0.0 && (ya < beam.y1){
            return (true, (ya - beam.y1).abs())
        }
        return (false, 0.0);
    } 

    if seg.x1 - seg.x2 == 0.0 {
        let xa = seg.x1;
        let ya = a1 * xa + b1;
        if min(seg.y1, seg.y2) <= ya && max(seg.y1, seg.y2) >= ya {
            let res = ((xa - beam.x1).powf(2.0) + (ya - beam.y1).powf(2.0)).sqrt();
            return (true, res);
        }
        return (false, 0.0);
    }

    if a1 == a2 {
        return (false, 0.0);
    }

    let xa = (b2 - b1) / (a1 - a2);

    if (xa < max(beam.x1, seg.x1)) || (xa > min( beam.new_x, seg.x2)) {
        return (false, 0.0); 
    }
    else {
        let ya = xa * a1 + b1;
        let res = ((xa - beam.x1).powf(2.0) + (ya - beam.y1).powf(2.0)).sqrt();
        return (true, res);
    }
}

fn main() {
    println!("enter coordinates of beam x1,y1 x2,y2");
    let mut input = String::new();
    let mut coordinates: Vec<f32> = vec![];
    io::stdin().read_line(&mut input)
        .expect("Не удалось прочитать строку");
    let mut v: Vec<_> = input.split(|c| c == ',' || c == ' ' || c == '\n').collect();
    for i in 0..4 {
        let num: f32 = v[i].trim().parse()
            .expect("Пожалуйста, введите число!");
        coordinates.push(num);
    }
    let mut _beam = Beam::new(coordinates[0],coordinates[1],
    coordinates[2],coordinates[3]);
    let mut min = f32::MAX;
    let mut close_seg = LineSegment::new(0.0,0.0,1.0,1.0);
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Не удалось прочитать строку");
        let mut v: Vec<_> = input.split(|c| c == ',' || c == ' ' || c == '\n').collect();
        if v[0] == "z" {
            break;
        }
        for i in 0..4 {
            let num: f32 = v[i].trim().parse()
                .expect("Пожалуйста, введите число!");
            coordinates[i] = num;
        }
        let seg = LineSegment::new(coordinates[0],coordinates[1],
        coordinates[2],coordinates[3]);
        let res = is_crossing(_beam, seg);
        if res.0 {
            if res.1 < min {
                close_seg = seg;
                min = res.1;
            }
        }
    }
    if min == f32::MAX {
        println!("\n");
    } else {
        println!("{:?}", close_seg);
    }
}