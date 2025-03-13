#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point, 
    b: Point, 
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 2 },
            b: Point { x: 7, y: 5 },
        },
        Rectangle {
            a: Point { x: 1, y: 1 },
            b: Point { x: 5, y: 7 },
        },
        Rectangle {
            a: Point { x: 6, y: 1 },
            b: Point { x: 9, y: 6 },
        },
    ]
}

fn area_occupied(rectangles: &Vec<Rectangle>) -> i32 {
    if rectangles.is_empty() {
        return 0;
    }

    let (mut min_x, mut min_y) = (i32::MAX, i32::MAX);
    let (mut max_x, mut max_y) = (i32::MIN, i32::MIN);

    for r in rectangles {
        let rx1 = r.a.x.min(r.b.x);
        let rx2 = r.a.x.max(r.b.x);
        let ry1 = r.a.y.min(r.b.y);
        let ry2 = r.a.y.max(r.b.y);

        if rx1 < min_x { min_x = rx1; }
        if rx2 > max_x { max_x = rx2; }
        if ry1 < min_y { min_y = ry1; }
        if ry2 > max_y { max_y = ry2; }
    }

    let width = (max_x - min_x) as usize;
    let height = (max_y - min_y) as usize;

    let mut coverage = vec![vec![false; width]; height];

    for r in rectangles {
        let rx1 = r.a.x.min(r.b.x);
        let rx2 = r.a.x.max(r.b.x);
        let ry1 = r.a.y.min(r.b.y);
        let ry2 = r.a.y.max(r.b.y);

        for x in rx1..rx2 {
            for y in ry1..ry2 {
                // Зсув у сітці
                let cx = (x - min_x) as usize;
                let cy = (y - min_y) as usize;
                coverage[cy][cx] = true;
            }
        }
    }

    let mut area = 0;
    for row in coverage {
        for cell in row {
            if cell {
                area += 1;
            }
        }
    }
    area
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_occupied_test() {
        let data = test_data();
        let occupied = area_occupied(&data);
        assert_eq!(occupied, 60);
    }
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Rectangles: {:#?}", data);
    println!("Occupied area = {}", occupied);
}
