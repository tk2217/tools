pub fn rgb_to_hsv(rgb: (u8, u8, u8)) -> (f32, f32, f32) {
    let r = rgb.0 as f32 / 255.0;
    let g = rgb.1 as f32 / 255.0;
    let b = rgb.2 as f32 / 255.0;

    let min = f32::min(r, f32::min(g, b));
    let max = f32::max(r, f32::max(g, b));
    let delta = max - min;

    let s = if max != 0.0 { delta / max } else { 0.0 };

    let h = if r == max {
        (g - b) / delta
    } else if g == max {
        (b - r) / delta + 2.0
    } else {
        (r - g) / delta + 4.0
    };

    let h = h * 60.0;
    let h = if h < 0.0 { h + 360.0 } else { h };

    (h / 360.0, s, max)
}
