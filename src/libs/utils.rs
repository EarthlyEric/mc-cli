use owo_colors::OwoColorize;

pub fn print_gradient_text(text: &str, start: (u8, u8, u8), end: (u8, u8, u8), is_new_line: bool) {
    let len = text.chars().count() as f32;
    let (r1, g1, b1) = (start.0 as f32, start.1 as f32, start.2 as f32);
    let (r2, g2, b2) = (end.0 as f32, end.1 as f32, end.2 as f32);

    for (i, c) in text.chars().enumerate() {
        let ratio = i as f32 / len;
        let r = (r1 + ratio * (r2 - r1)) as u8;
        let g = (g1 + ratio * (g2 - g1)) as u8;
        let b = (b1 + ratio * (b2 - b1)) as u8;
        print!("{}", c.to_string().truecolor(r, g, b));
    }

    if is_new_line {
        println!();
    }
}
