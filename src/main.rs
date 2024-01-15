fn main() {
    //    let width = 100;
    //    let height = 75;
    let width = 50;
    let height = 50;
    let mut cave = randomize(width, height);
    show(&cave, width, height);
    for i in 0..3 {
        println!("-----[Cave Generation]-----");
        cave = update(&cave, width, height);
        show(&cave, width, height);
    }
}

fn randomize(width: i32, height: i32) -> Vec<bool> {
    let mut cave = Vec::with_capacity(width as usize * height as usize);

    for y in 0..height {
        for x in 0..width {
            cave.push(rand::random::<bool>());
        }
    }

    cave
}

//fn randomize(width: i32, height: i32) -> Vec<bool> {
//    let mut cave = Vec::with_capacity(width as usize * height as usize);
//    let center_x = width / 2;
//    let center_y = height / 2;
//    let radius = 50.0;
//
//    println!("width:{}", width);
//    println!("height:{}", height);
//    println!("center_x:{}", center_x);
//    println!("center_y:{}", center_y);
//
//    for y in 0..height {
//        for x in 0..width {
//            let value = rand::random::<f32>() * radius;
//            let ox = x as f32 - center_x as f32;
//            let oy = y as f32 - center_y as f32;
//
//            let distance = radius - (ox * ox + oy * oy).sqrt();
//            cave.push(value > distance);
//
////            if (ox*ox + oy*oy) < radius * radius {
////                cave.push(value > 0.5);
////            } else {
////                cave.push(true);
////            }
//        }
//    }
//
//    cave
//}

fn is_solid(cave: &Vec<bool>, width: i32, height: i32, x: i32, y: i32) -> bool {
    if x >= 0 && x <= (width - 1) && y >= 0 && y <= (height - 1) {
        return *cave.get(y as usize * width as usize + x as usize).unwrap();
    }
    true
}

fn update(cave: &Vec<bool>, width: i32, height: i32) -> Vec<bool> {
    let mut new_cave = Vec::with_capacity(width as usize * height as usize);

    for y in 0..height {
        for x in 0..width {
            let mut count = 0;

            for j in (y - 1)..=(y + 1) {
                for i in (x - 1)..=(x + 1) {
                    if is_solid(cave, width, height, i, j) {
                        count += 1;
                    }
                }
            }

            if count >= 5 {
                new_cave.push(true);
            } else {
                new_cave.push(false);
            }
        }
    }

    new_cave
}

fn show(cave: &Vec<bool>, width: i32, height: i32) {
    for y in 0..height {
        for x in 0..width {
            if *cave.get(y as usize * width as usize + x as usize).unwrap() {
                print!("# ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}
