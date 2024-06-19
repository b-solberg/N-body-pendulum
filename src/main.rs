use std::f64::consts::PI;
mod rk4; use rk4::rk4_step;

use nannou::prelude::*;

use nannou_egui::{self, egui, Egui};

//TODO! Imports for saving to gifs or other file formats

// use std::path::Path;
// use std::{fs::File,env};
// use image::{io::Reader,Frame as Img,codecs::gif::GifEncoder};
// use std::fs::read_dir;

struct Model {
    _window: window::Id,
    objects: usize,
    object_update: usize,
    angle: Vec<f64>,
    velocity: Vec<f64>,
    lengths: Vec<f64>,
    masses: Vec<f64>,
    egui: Egui,
}


fn main() {
    nannou::app(model).update(update).run();
   
}
fn model(app: &App) -> Model {
    
    let n: usize  = 2;
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);
    
      Model {
        _window: window_id,
        objects: n,
        object_update:n,
        angle: vec![PI/2.0;n],
        velocity: vec![0.0;n],
        lengths: vec![1.0;n],
        masses: vec![1.0;n],
        egui: egui,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let egui = &mut model.egui;

    let ctx = egui.begin_frame();

    egui::Window::new("Menu").show(&ctx, |ui| {

        ui.label("Number of Pendulums:");
        ui.add(egui::Slider::new(&mut model.object_update, 1..=50));
        let clicked = ui.button("Reset").clicked();

        if clicked {
            model.lengths=vec![1.0;model.object_update];
            model.masses= vec![1.0;model.object_update];
            model.angle = vec![PI/2.0;model.object_update];
            model.velocity = vec![0.0;model.object_update];
            model.objects = model.object_update;
        }

        //TODO! : Save to gif or other file format
        // let clicked = ui.button("Save to gif").clicked();

        // if clicked {
        //     let num_frames = read_dir("./src/video/").unwrap().count();
        //     let frames: Vec<Img> = (0..num_frames).step_by(2).map(|i| Img::new(Reader::open(app.project_path()
        //     .expect("failed to locate `project_path`")
        //     // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        //     // .join(app.exe_name().unwrap())
        //     .join(Path::new("./src/video/"))
    
        //     // Name each file after the number of the frame.
        //     .join(format!("{:10}", i))
        //     // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        //     .with_extension("tiff")).unwrap().decode().unwrap().to_rgba8())).collect::<Vec<Img>>();
        //     let file_out = File::create("pendulum_sim.gif").unwrap();
        //     let mut encoder = GifEncoder::new_with_speed(file_out,30);//.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();
        //     let _ = encoder.encode_frames(frames.into_iter());
        //     app.quit()
        // }
    });
   
    let mut step = rk4_step(&model.objects, &mut model.angle, &mut model.velocity, &model.lengths, &model.masses, (app.duration.since_prev_update.secs() as f64)/2.0);
    step = rk4_step(&model.objects, &step.0, &step.1, &model.lengths, &model.masses, (app.duration.since_prev_update.secs() as f64)/2.0);
    
    
    model.angle = step.0;
    model.velocity = step.1;
}

fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
    model.egui.handle_raw_event(event);
}
fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // This is here to change to full screen if needed
    // app.window(model._window).unwrap().set_fullscreen(true);
    let range = model.lengths.iter().sum::<f64>();
    
    let boundary = app.window_rect().pad(75.0);

    let mut x_coords = vec![0.0;model.objects+1];
    let mut y_coords = vec![0.0;model.objects+1];
    let mut xs = vec![0.0;model.objects+1];
    let mut ys = vec![0.0;model.objects+1];
    for i in 1..=model.objects{
        x_coords[i] = x_coords[i-1] + model.angle[i-1].sin(); 
        y_coords[i] = y_coords[i-1] - model.angle[i-1].cos();
        xs[i] = map_range(x_coords[i], -range,range, boundary.bottom(), boundary.top());
        ys[i] = map_range(y_coords[i], -range,range, boundary.bottom(), boundary.top());
    }

    draw.background().color(PLUM);
    
    for i in 1..=model.objects{
    draw.ellipse().color(STEELBLUE).x_y(xs[i], ys[i]).w(15.0).h(15.0);
    draw.line()
    .start(pt2(xs[i-1],ys[i-1]))
    .end(pt2(xs[i],ys[i]))
    .weight(8.0)
    .color(STEELBLUE);
    }
    draw.to_frame(app, &frame).unwrap();
    
    model.egui.draw_to_frame(&frame).unwrap();
    //let file_path = captured_frame_path(app, &frame);
    //app.main_window().capture_frame(file_path);
}

//TODO! : Save to gif or other file format

// fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
//     // Create a path that we want to save this frame to.
//     app.project_path()
//         .expect("failed to locate `project_path`")
//         // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
//         // .join(app.exe_name().unwrap())
//         .join(Path::new("./src/video/"))

//         // Name each file after the number of the frame.
//         .join(format!("{:10}", frame.nth()))
//         // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
//         .with_extension("tiff")
// }
