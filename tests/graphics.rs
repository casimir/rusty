extern crate rusty;

use rusty::graphics::*;

#[test]
fn color_init() {
    assert_eq!("#01020310".parse::<Color>(),
               Ok(Color {
                      r: 0x01,
                      g: 0x02,
                      b: 0x03,
                      a: 0x10,
                  }));
    assert_eq!("#FF1493".parse::<Color>(),
               Ok(Color {
                      r: 0xff,
                      g: 0x14,
                      b: 0x93,
                      a: 0xff,
                  }));
    assert_eq!("nope".parse::<Color>(), Err(ColorError::InvalidColorError));
    assert!("#42".parse::<Color>().is_err());
}

#[test]
fn color_add() {
    assert_eq!("#112244".parse::<Color>().unwrap() + "#112244".parse::<Color>().unwrap(),
               "#224488".parse::<Color>().unwrap());
    assert_eq!("#FFFF00".parse::<Color>().unwrap() + "#00FF80".parse::<Color>().unwrap(),
               "#FFFF80".parse::<Color>().unwrap())
}

#[test]
fn color_mult() {
    assert_eq!("#222222".parse::<Color>().unwrap() * 0.5,
               "#111111".parse::<Color>().unwrap());
    assert_eq!("#222222".parse::<Color>().unwrap() * 1.0,
               "#222222".parse::<Color>().unwrap());
    assert_eq!("#222222".parse::<Color>().unwrap() * 2.0,
               "#444444".parse::<Color>().unwrap());

    assert_eq!("#222222".parse::<Color>().unwrap() * "#00FF80".parse::<Color>().unwrap(),
               "#002211".parse::<Color>().unwrap())
}

#[test]
fn canvas_manipulation() {
    let color = Color {
        r: 0xff,
        g: 0xff,
        b: 0xff,
        a: 0xff,
    };
    let mut canvas = Canvas::new(1, 1);
    assert_eq!(canvas.get(0, 0), Pixel::Blank);
    canvas.set(0, 0, color);
    assert_eq!(canvas.get(0, 0), Pixel::Data(color));
    canvas.unset(0, 0);
    assert_eq!(canvas.get(0, 0), Pixel::Blank);
}
