use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub enum EasingKind {
    EaseInSine,
    EaseOutSine,
    EaseInOutSine,
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInQuart,
    EaseOutQuart,
    EaseInOutQuart,
    EaseInOutBack,
}

trait Calcuratable {
    fn calc(v: f64) -> f64;
}

struct EaseInSine {}
impl Calcuratable for EaseInSine {
    fn calc(v: f64) -> f64 {
        1.0 - ((v * PI) / 2.0).cos()
    }
}

struct EaseOutSine {}
impl Calcuratable for EaseOutSine {
    fn calc(v: f64) -> f64 {
        ((v * PI) / 2.0).sin()
    }
}

struct EaseInOutSine {}
impl Calcuratable for EaseInOutSine {
    fn calc(v: f64) -> f64 {
        ((PI * v).cos() - 1.0) / 2.0
    }
}

struct EaseInCubic {}
impl Calcuratable for EaseInCubic {
    fn calc(v: f64) -> f64 {
        v * v * v
    }
}

struct EaseOutCubic {}
impl Calcuratable for EaseOutCubic {
    fn calc(v: f64) -> f64 {
        1.0 - (1.0 - v).powf(3.0)
    }
}

struct EaseInOutCubic {}
impl Calcuratable for EaseInOutCubic {
    fn calc(v: f64) -> f64 {
        if v < 0.5 {
            return 4.0 * v * v * v;
        } else {
            return 1.0 - (-2.0 * v + 2.0).powf(3.0) / 2.0;
        }
    }
}

struct EaseInQuad {}
impl Calcuratable for EaseInQuad {
    fn calc(v: f64) -> f64 {
        v * v
    }
}

struct EaseOutQuad {}
impl Calcuratable for EaseOutQuad {
    fn calc(v: f64) -> f64 {
        1.0 - (1.0 - v) * (1.0 - v)
    }
}

struct EaseInOutQuad {}
impl Calcuratable for EaseInOutQuad {
    fn calc(v: f64) -> f64 {
        if v < 0.5 {
            return 2.0 * v * v;
        } else {
            return 1.0 - (-2.0 * v + 2.0).powf(2.0) / 2.0;
        }
    }
}

struct EaseInQuart {}
impl Calcuratable for EaseInQuart {
    fn calc(v: f64) -> f64 {
        v * v * v * v
    }
}

struct EaseOutQuart {}
impl Calcuratable for EaseOutQuart {
    fn calc(v: f64) -> f64 {
        1.0 - (1.0 - v).powf(4.0)
    }
}

struct EaseInOutQuart {}
impl Calcuratable for EaseInOutQuart {
    fn calc(v: f64) -> f64 {
        if v < 0.5 {
            return 8.0 * v * v * v * v;
        } else {
            return 1.0 - (-2.0 * v + 2.0).powf(4.0) / 2.0;
        }
    }
}

struct EaseInOutBack {}
impl Calcuratable for EaseInOutBack {
    fn calc(v: f64) -> f64 {
        const C1: f64 = 1.70158;
        const C2: f64 = C1 * 1.525;

        if v < 0.5 {
            return (2.0 * v).powf(2.0) * ((C2 + 1.0) * 2.0 * v - C2) / 2.0;
        } else {
            return ((2.0 * v - 2.0).powf(2.0) * ((C2 + 1.0) * (v * 2.0 - 2.0) + C2) + 2.0) / 2.0;
        }
    }
}

pub fn match_easing(name: EasingKind) -> fn(v: f64) -> f64 {
    match name {
        EasingKind::EaseInSine => return EaseInSine::calc,
        EasingKind::EaseOutSine => return EaseOutSine::calc,
        EasingKind::EaseInOutSine => return EaseInOutSine::calc,
        EasingKind::EaseInCubic => return EaseInCubic::calc,
        EasingKind::EaseOutCubic => return EaseOutCubic::calc,
        EasingKind::EaseInOutCubic => return EaseInOutCubic::calc,
        EasingKind::EaseInQuad => return EaseInQuad::calc,
        EasingKind::EaseOutQuad => return EaseOutQuad::calc,
        EasingKind::EaseInOutQuad => return EaseInOutQuad::calc,
        EasingKind::EaseInQuart => return EaseInQuart::calc,
        EasingKind::EaseOutQuart => return EaseOutQuart::calc,
        EasingKind::EaseInOutQuart => return EaseInOutQuart::calc,
        EasingKind::EaseInOutBack => return EaseInOutBack::calc,
    }
}

// +++++++++++++++++++
//  utils
// +++++++++++++++++++
pub fn norm(value: f64, upper: f64, bottom: f64) -> f64 {
    (value - bottom) / (upper - bottom)
}

fn lerp(upper: f64, bottom: f64, ratio: f64) -> f64 {
    bottom + (upper - bottom) * ratio
}

pub fn mapping(
    to_upper: f64,
    to_bottom: f64,
    from_upper: f64,
    from_bottom: f64,
    value: f64,
) -> f64 {
    lerp(to_upper, to_bottom, norm(value, from_upper, from_bottom))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ease_in_sine_works() {}

    #[test]
    fn mapping_works() {
        let result = mapping(500.0, 300.0, 200.0, 100.0, 140.0);
        assert_eq!(result, 380.0);
    }

    #[test]
    fn norm_works() {
        let result = norm(0.01, 1.0, 0.0);
        assert_eq!(result, 0.01);
    }
}
