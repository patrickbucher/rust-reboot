enum Shape {
    Rectangle { width: f32, height: f32 },
    Square(f32),
}

impl Shape {
    fn circumference(&self) -> f32 {
        match self {
            Shape::Rectangle {
                width: w,
                height: h,
            } => 2.0 * (w + h),
            Shape::Square(s) => 4.0 * s,
        }
    }
}

fn maybe_sqrt(x: f32) -> Option<f32> {
    if x >= 0.0 {
        Some(x.sqrt())
    } else {
        None
    }
}

fn main() {
    let r = Shape::Rectangle {
        width: 3.0,
        height: 4.0,
    };
    let s = Shape::Square(5.0);
    println!("{} {}", r.circumference(), s.circumference());

    let x: f32 = 16.0;
    if let Some(y) = maybe_sqrt(16.0) {
        println!("sqrt({})={}", x, y);
    }
}
