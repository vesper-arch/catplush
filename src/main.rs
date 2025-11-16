use rust_clayout::clay_main::*;
use rust_clayout::clay_raylib;
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = init()
        .size(640, 640)
        .title("I think therefore I am.")
        .resizable()
        .build();

    while !rl.window_should_close() {
        let mut ui = ClayContext::begin_layout((rl.get_screen_width(), rl.get_screen_height()), ChildLayoutDirection::LeftToRight);

        ui.open_element(ClayElement::new()
            .rectangle(ObjectColor(26, 67, 87, 255), CornerRadius::all(20.0))
            .sizing(SizingMode::Grow, SizingMode::Grow)
            .padding(Padding::all(10))
            .child_gap(10));

            ui.open_element(ClayElement::new()
                .layout_direction(ChildLayoutDirection::LeftToRight)
                .rectangle(ObjectColor(0, 105, 143, 255), CornerRadius::all(15.0))
                .sizing(SizingMode::Grow, SizingMode::Fit)
                .padding(Padding::all(10))
                .child_gap(10));

                ui.open_element(ClayElement::new()
                    .border(ObjectColor(89, 172, 212, 255), CornerRadius::all(10.0), 2)
                    .sizing(SizingMode::Fixed(180), SizingMode::Fixed(120)));
                ui.close_element();

                ui.open_element(ClayElement::new()
                    .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
                    .sizing(SizingMode::Grow, SizingMode::Fixed(150)));
                ui.close_element();

                ui.open_element(ClayElement::new()
                    .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
                    .sizing(SizingMode::Grow, SizingMode::Grow));
                ui.close_element();
            ui.close_element();

        ui.close_element();

        let render_commands = ui.end_layout();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        clay_raylib::raylib_render_all(render_commands, &mut d);
    }
}
