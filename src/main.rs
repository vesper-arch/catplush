use rust_clayout::clay_main::*;
use rust_clayout::clay_raylib::*;
use raylib::prelude::*;
use raylib::ffi;

fn main() {
    let (mut rl, thread) = init()
        .size(640, 640)
        .title("I am going to explode")
        .resizable()
        .build();

    while !rl.window_should_close() {
        let mut ui = ClayContext::begin_layout((rl.get_screen_width(), rl.get_screen_height()), ChildLayoutDirection::LeftToRight);


        // Only used to show where the below image should be by inserting a gap.
        ui.open_element(ClayElement::new()
            .rectangle(ObjectColor(0, 0, 0, 0), CornerRadius::all(0.0))
            .sizing(SizingMode::Fixed(32), SizingMode::Fixed(32)));
        ui.close_element();

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

        let test_image = rl.load_texture(&thread, "/home/vesper-arch/Documents/Downfall/Tiny Card Icons/CardIcon_Boss_Skill_Uncommon.png").unwrap();
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        d.draw_texture(test_image, 0, 0, Color{r: 0, g: 0, b: 0, a: 255});
        raylib_render_all(render_commands, &mut d);
    }
}
