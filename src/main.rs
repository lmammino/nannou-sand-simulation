mod cave;

use cave::*;
mod simulation;
use nannou::prelude::*;
use simulation::*;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::rate_fps(240.0))
        .run();
}

const PIXEL_SIZE: f32 = 3.0;
const STEPS_PER_FRAME: usize = 4;

struct Model<'a> {
    simulation: Simulation<'a>,
    last_step: Option<Step>,
    bounds: Rect,
    min_x: f32,
}

fn model(app: &App) -> Model<'static> {
    app.new_window().size(600, 600).view(view).build().unwrap();

    let input = include_str!("input.txt");
    let cave: Cave = input.parse().unwrap();
    let min_x = cave.tiles.keys().map(|pos| pos.x).min().unwrap() as f32;
    let min_y = cave.sand_emitter.y as f32;
    let max_x = cave.tiles.keys().map(|pos| pos.x).max().unwrap() as f32;
    let max_y = cave.tiles.keys().map(|pos| pos.y).max().unwrap() as f32;
    let bounds = Rect::from_w_h((max_x - min_x) * PIXEL_SIZE, (max_y - min_y) * PIXEL_SIZE);

    let simulation = part1(cave);
    Model {
        simulation,
        last_step: None,
        bounds,
        min_x,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb8(200, 211, 211));

    if let Some(step) = &model.last_step {
        for (pos, block) in step.tiles.iter() {
            let pixel = Rect::from_x_y_w_h(
                (pos.x as f32 - model.min_x) * PIXEL_SIZE - (model.bounds.w() / 2.0),
                ((pos.y as f32) * PIXEL_SIZE - (model.bounds.h() / 2.0)) * -1.0,
                PIXEL_SIZE,
                PIXEL_SIZE,
            );

            let color = match block {
                Block::Wall => BLACK,
                Block::Sand => rgb8(55, 44, 44),
            };
            draw.rect().xy(pixel.xy()).wh(pixel.wh()).color(color);
        }
        if let Some(grain_pos) = &step.grain {
            let pixel = Rect::from_x_y_w_h(
                (grain_pos.x as f32 - model.min_x) * PIXEL_SIZE - (model.bounds.w() / 2.0),
                ((grain_pos.y as f32) * PIXEL_SIZE - (model.bounds.h() / 2.0)) * -1.0,
                PIXEL_SIZE,
                PIXEL_SIZE,
            );
            draw.rect()
                .xy(pixel.xy())
                .wh(pixel.wh())
                .color(rgb8(69, 55, 55));
        }
        draw.to_frame(app, &frame).unwrap();

        // Decomment the code below if you want to save the frames to disk
        // let path = app
        //     .project_path()
        //     .unwrap()
        //     .join("frames")
        //     .join(format!("{:06}.png", frame.nth()))
        //     .with_extension("png");
        // app.main_window().capture_frame(path);
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for _ in 0..STEPS_PER_FRAME {
        if let Some(step) = model.simulation.resume() {
            model.last_step = Some(step);
        }
    }
}
