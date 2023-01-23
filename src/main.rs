use nannou::prelude::*;
use std::cmp::min;

const WIN_W: u32 = 600;
const WIN_H: u32 = 900;
const SQUARE_W: f32 = 60.0;
const SQUARE_H: f32 = 80.0;
const NUM_X: i32 = 7;
const NUM_Y: i32 = 9;
const MAX_NUM_SQUARES: usize = 8;
const COLORS: &[&[f32; 3]; 5] = &[
    &[0.24, 0.24, 0.24], // 0.72
    &[0.1, 0.43, 0.65],  // 1.18
    &[0.38, 0.68, 0.67], // 1.73
    &[0.92, 0.54, 0.38], // 1,84
    &[0.65, 0.54, 0.68], // 1,87
];

fn main() {
    nannou::app(model).update(update).run();
}

struct Square {
    points: Vec<Vec2>,
    weight: f32,
}
impl Square {
    pub fn new(randomness: f32, i: usize) -> Self {
        let w = SQUARE_W / 2.2;
        let h = SQUARE_H / 2.2;

        let min = 0.99 - randomness * (i as f32) * 0.2;
        let max = 1.0 + randomness * (i as f32) * 0.2;

        let mut points = vec![];
        let point1 = pt2(random_range(min, max) * -w, random_range(min, max) * -h);
        points.push(point1);
        points.push(pt2(random_range(min, max) * w, random_range(min, max) * -h));
        points.push(pt2(random_range(min, max) * w, random_range(min, max) * h));
        points.push(pt2(random_range(min, max) * -w, random_range(min, max) * h));
        points.push(point1);

        let weight = random_range::<f32>(1.0, 3.0);

        Square { points, weight }
    }
}
struct SquareStructure {
    position: Vec2,
    collection: Vec<Square>,
    randomness: f32,
}

impl SquareStructure {
    pub fn new(i_x: f32, i_y: f32) -> Self {
        let w = SQUARE_W;
        let h = SQUARE_H;
        let max_x = WIN_W as f32;
        let max_y = WIN_H as f32;

        let x = i_x * w;
        let y = i_y * h;
        let x = x - (max_x * 0.3);
        let y = y - (max_y * 0.35);

        let randomness = (i_y - (i_x + 1.0)) / 8.0;
        println!("x = {}, y = {}, randomness = {}", i_x, i_y, randomness);

        let collection = (0..MAX_NUM_SQUARES)
            .into_iter()
            .map(|i| Square::new(randomness, i))
            .collect();

        SquareStructure {
            position: pt2(x, y),
            collection,
            randomness,
        }
    }

    pub fn set_points(&mut self) {
        self.collection = (0..MAX_NUM_SQUARES)
            .into_iter()
            .map(|i| Square::new(self.randomness, i))
            .collect();
    }
}

fn gen_structures() -> Vec<SquareStructure> {
    let mut sqrs = vec![];
    for i in 0..NUM_X {
        for j in 0..NUM_Y {
            let q = SquareStructure::new(i as f32, j as f32);
            sqrs.push(q);
        }
    }
    sqrs
}

struct Model {
    _window: WindowId,
    squares: Vec<SquareStructure>,
    current_num_squares: usize,
    export: usize,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .title("cc in nannou")
        .size(WIN_W, WIN_H)
        .mouse_released(mouse_released)
        .mouse_wheel(mouse_wheel)
        .view(view)
        .build()
        .unwrap();

    let squares = gen_structures();
    Model {
        _window,
        squares,
        current_num_squares: 1,
        export: 0,
    }
}

fn save_frame(app: &App, model: &mut Model) {
    let file_path = app
        .project_path()
        .expect("failed to locate `project_path`")
        .join(app.exe_name().unwrap())
        .join(format!("{:03}", model.export))
        .with_extension("png");
    app.main_window().capture_frame(file_path);
    model.export +=1;
}

fn mouse_released(app: &App, model: &mut Model, button: MouseButton) {
    match button {
        MouseButton::Left => {
            for structure in model.squares.iter_mut() {
                structure.set_points();
            }
        }
        MouseButton::Right => {
            save_frame(app, model);
        }
        _ => {}
    }
}

fn mouse_wheel(app: &App, model: &mut Model, dt: MouseScrollDelta, _phase: TouchPhase) {
    match dt {
        MouseScrollDelta::LineDelta(_, y) if y > 0.0 => {
            if model.current_num_squares < MAX_NUM_SQUARES {
                model.current_num_squares += 1;
                save_frame(app, model);
            }
        }
        MouseScrollDelta::LineDelta(_, y) if y < 0.0 => {
            if model.current_num_squares > 1 {
                model.current_num_squares -= 1;
                save_frame(app, model);
            }
        }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().rgb(0.88, 0.87, 0.85);

    for structure in model.squares.iter() {
        for (i, square) in structure.collection.iter().enumerate().take(min(
            model.current_num_squares,
            (structure.randomness * 8.0).abs() as usize + 1,
        )) {
            let cc = COLORS[i % COLORS.len()];
            let c = rgb(cc[0], cc[1], cc[2]);
            draw.polyline()
                .xy(structure.position)
                .color(c)
                .weight(square.weight)
                .points(square.points.clone());
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
