use rust_clayout::clay_main::*;
use rust_clayout::clay_raylib::*;
use raylib::prelude::*;

fn template_button(button_width: i32) -> ClayElement {
    ClayElement::new()
        .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
        .sizing(SizingMode::Fixed(button_width), SizingMode::Grow)
}

fn spacer() -> ClayElement {
    ClayElement::new()
        .sizing(SizingMode::Grow, SizingMode::Grow)
}

fn sidebar_element() -> ClayElement {
    ClayElement::new()
        .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
        .sizing(SizingMode::Grow, SizingMode::Fixed(50))
}

fn main() {
    let (mut rl, thread) = init()
        .size(640, 640)
        .title("I am going to explode")
        .resizable()
        .build();

    while !rl.window_should_close() {
        let mut ui = ClayContext::begin_layout((rl.get_screen_width(), rl.get_screen_height()), ChildLayoutDirection::TopToBottom);

        ui.open_element(ClayElement::new()
            .sizing(SizingMode::Grow, SizingMode::Grow)
            .layout_direction(ChildLayoutDirection::TopToBottom)
            .padding(Padding::all(10))
            .child_gap(10));

            ui.open_element(ClayElement::new()
                .rectangle(ObjectColor(0, 105, 143, 255), CornerRadius::all(15.0))
                .sizing(SizingMode::Grow, SizingMode::Fixed(50))
                .padding(Padding::all(10))
                .child_gap(10));

                ui.open_element(template_button(100));
                ui.close_element();

                ui.open_element(template_button(75));
                ui.close_element();

                ui.open_element(spacer());
                ui.close_element();

                ui.open_element(template_button(75));
                ui.close_element();

            ui.close_element();

            ui.open_element(ClayElement::new()
                .rectangle(ObjectColor(26, 67, 87, 255), CornerRadius::all(15.0))
                .sizing(SizingMode::Grow, SizingMode::Grow)
                .padding(Padding::all(10))
                .child_gap(10));

                ui.open_element(ClayElement::new()
                    .rectangle(ObjectColor(17, 36, 46, 255), CornerRadius::all(15.0))
                    .sizing(SizingMode::Fixed(200), SizingMode::Grow)
                    .layout_direction(ChildLayoutDirection::TopToBottom)
                    .padding(Padding::all(10))
                    .child_gap(10));

                    for _ in 1..5 {
                        ui.open_element(sidebar_element());
                        ui.close_element();
                    }

                ui.close_element();

                ui.open_element(ClayElement::new()
                    .rectangle(ObjectColor(17, 36, 46, 255), CornerRadius::all(15.0))
                    .sizing(SizingMode::Grow, SizingMode::Grow));
                ui.close_element();

            ui.close_element();

        ui.close_element();

        let render_commands = ui.end_layout();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        raylib_render_all(render_commands, &mut d);
    }
}
