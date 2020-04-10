use crate::tile::Map;

#[derive(Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

fn line(x1: i32, y1: i32, x2: i32, y2: i32, map: &Map) -> Vec<Point> {
    let mut coordinates = vec![];
    let mut x1 = x1;
    let mut y1 = y1;

    let dx = i32::abs(x2 - x1);
    let dy = i32::abs(y2 - y1);

    let sx = if x1 < x2 { 1 } else { -1 };
    let sy = if y1 < y2 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };
    let mut err2;

    loop {
        coordinates.push(Point { x: x1, y: y1 });

        if x1 == x2 && y1 == y2 || map[x1 as usize][y1 as usize].block_sight {
            break;
        }

        err2 = err;

        if err2 > -dx {
            err -= dy;
            x1 += sx;
        }

        if err2 < dy {
            err += dx;
            y1 += sy;
        }
    }

    coordinates
}

pub fn raycast_on_map(map: &mut Map, px: i32, py: i32, points: &Vec<Point>) {
    let mut coords: Vec<Vec<Point>> = vec![];

    for point in points {
        coords.push(line(px, py, point.x, point.y, &map));
    }

    // reset map visibility
    for elem in map.iter_mut() {
        for j in elem.iter_mut() {
            j.currently_visible = false;
        }
    }

    for i in coords.iter() {
        for point in i.iter() {
            map[point.x as usize][point.y as usize].visible = true;
            map[point.x as usize][point.y as usize].currently_visible = true;
        }
    }
}
