pub fn ease_back_out(t: f32) -> f32 {
    let c1 = 1.70158;
    let c3 = c1 + 1.0;
    1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
}

pub fn ease_elastic_out(t: f32) -> f32 {
    let c4 = (2.0 * std::f32::consts::PI) / 3.0;
    if t == 0.0 {
        0.0
    } else if t == 1.0 {
        1.0
    } else {
        2.0f32.powf(-10.0 * t) * ((t * 10.0 - 0.75) * c4).sin() + 1.0
    }
}

pub fn ease_out_bounce(mut t: f32) -> f32 {
    let n1 = 7.5625;
    let d1 = 2.75;
    if t < 1.0 / d1 {
        n1 * t * t
    } else if t < 2.0 / d1 {
        t -= 1.5 / d1;
        n1 * t * t + 0.75
    } else if t < 2.5 / d1 {
        t -= 2.25 / d1;
        n1 * t * t + 0.9375
    } else {
        t -= 2.625 / d1;
        n1 * t * t + 0.984375
    }
}

#[cfg(test)]
mod tests {
    use super::{ease_back_out, ease_elastic_out, ease_out_bounce};

    #[test]
    fn easing_endpoints() {
        assert!((ease_back_out(1.0) - 1.0).abs() < 0.0001);
        assert!((ease_elastic_out(0.0) - 0.0).abs() < 0.0001);
        assert!((ease_elastic_out(1.0) - 1.0).abs() < 0.0001);
        assert!((ease_out_bounce(0.0) - 0.0).abs() < 0.0001);
        assert!((ease_out_bounce(1.0) - 1.0).abs() < 0.0001);
    }
}
