use nannou::prelude::*;
use nannou::rand::random_range;

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {
    n: usize,
    w: f32,
    h: f32,
    previous: Vec<Point2>,
    next: Vec<Point2>,
    interval: u32,
    t: u32,
}

impl Model {
    pub fn new(n: usize, w: f32, h: f32, interval: u32) -> Model {
        Model {
            n,
            w,
            h,
            previous: Model::get_points(n, w, h),
            next: Model::get_points(n, w, h),
            interval,
            t: 0,
        }
    }

    fn get_point(w: f32, h: f32) -> Point2 {
        pt2(random_range(-w / 2.0, w / 2.0), random_range(-h / 2.0, h / 2.0))
    }

    fn get_points(n: usize, w: f32, h: f32) -> Vec<Point2> {
        let mut vec = Vec::new();
        for i in 0..n {
            vec.push(Model::get_point(w, h));
        }
        vec
    }

    pub fn get_current(&self) -> Vec<Point2> {
        let mut vec: Vec<Point2> = Vec::new();
        for i in 0..self.n {
            let d = (self.next[i] - self.previous[i]).map(|x| {
                x * self.t as f32 / self.interval as f32
            });
            vec.push(self.previous[i] + d);
        }
        vec
    }

    pub fn next(&mut self) {
        if self.t < self.interval {
            self.t += 1;
        } else {
            self.t = 0;
            self.previous = self.next.clone();
            self.next = Model::get_points(self.n, self.w, self.h);
        }
    }
}

fn model(_app: &App) -> Model {
    Model::new(20, 200.0, 200.0, 60)
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.next();
}

fn view(_app: &App, _model: &Model, frame: Frame) {
    let draw = _app.draw();
    draw.background().color(PURPLE);
    let points = _model.get_current().iter().map(|point| {
        (point, BLACK)
    });
    draw.to_frame(_app, &frame);
}
