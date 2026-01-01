use catplush::catplush_main::*;
use catplush::catplush_friend::*;
use frienderer::{Renderer};
use glam::{ivec2, Vec2};
use glfw::{Action, Context, Key, OpenGlProfileHint, WindowHint};
use image::ImageFormat;

struct CardColor;

impl CardColor {
    pub const RED: ObjectColor = ObjectColor::from_u32_hex(0x91352eff);
    pub const GREEN: ObjectColor = ObjectColor::from_u32_hex(0x066a2dff);
    pub const BLUE: ObjectColor = ObjectColor::from_u32_hex(0x31549dff);
    pub const PURPLE: ObjectColor = ObjectColor::from_u32_hex(0x69418eff);
}

fn topbar_button(button_width: i32) -> UiElement {
    UiElement::new()
        .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
        .sizing(SizingMode::Fixed(button_width), SizingMode::Grow)
        .alignment(ChildXAlignment::Center, ChildYAlignment::Center)
}

fn spacer() -> UiElement {
    UiElement::new()
        .sizing(SizingMode::Grow, SizingMode::Grow)
}

fn sidebar_element() -> UiElement {
    UiElement::new()
        .rectangle(ObjectColor(51, 136, 175, 255), CornerRadius::all(10.0))
        .sizing(SizingMode::Grow, SizingMode::Fixed(50))
        .padding(Padding::all(20))
}

fn card_cell(ui: &mut CatplushContext, bitmap: &BitmapConfiguration, card_name: &str, card_icon: CatplushTextureData, card_color: ObjectColor) {
    ui.open_element(UiElement::new()
        .rectangle(card_color, CornerRadius::all(10.0))
        .sizing(SizingMode::Grow, SizingMode::Fixed(40))
        .padding(Padding::new(10, 15, 5, 5))
        .alignment(ChildXAlignment::Left, ChildYAlignment::Center)
        .child_gap(5));

        ui.open_element(UiElement::new()
            .image(card_icon, None, None, false));
        ui.close_element();

        ui.open_element(spacer());
        ui.close_element();

        ui.open_element(UiElement::new()
            .text(bitmap, card_name, 17));
        ui.close_element();

    ui.close_element();
}

const CONTRAST_HIGHLIGHT: ObjectColor = ObjectColor::from_u32_hex(0xeebe2bff);
const MONOCHROME_HIGHLIGHT: ObjectColor = ObjectColor::from_u32_hex(0x7dd1eeff);
const DARK_BORDER: ObjectColor = ObjectColor::from_u32_hex(0x213d4dff);

const WATCHER_RARE_SKILL_IMAGE: &[u8] = include_bytes!("../resources/CardIcon_Watcher_Skill_Rare.png");
const IRONCLAD_UNCOMMON_POWER_IMAGE: &[u8] = include_bytes!("../resources/CardIcon_Ironclad_Power_Uncommon.png");
const DEFECT_RARE_ATTACK_IMAGE: &[u8] = include_bytes!("../resources/CardIcon_Defect_Attack_Rare.png");
const SILENT_COMMON_ATTACK_IMAGE: &[u8] = include_bytes!("../resources/CardIcon_Silent_Attack_Common.png");
const AWAKENED_IMAGE: &[u8] = include_bytes!("../resources/Achv-Awakened.png");

const UIUA_BITMAP: &[u8] = include_bytes!("../resources/Uiua386.png");

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
	glfw.window_hint(WindowHint::ContextVersion(3, 3));
	glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
	glfw.window_hint(WindowHint::Resizable(true));
	glfw.window_hint(WindowHint::TransparentFramebuffer(true));

	let (mut window, events) = glfw
		.create_window(1200, 700, "LETS FUCKING GOOOOOOOO", glfw::WindowMode::Windowed)
		.expect("Failed to create GLFW window.");

	window.set_key_polling(true);
	window.make_current();

	let (width, height) = window.get_size();
	let scale_factor = window.get_content_scale().0;
	let viewport = ivec2(width, height).as_vec2() / scale_factor;

	// Load OpenGL functions
	let gl = unsafe {
		glow::Context::from_loader_function(|symbol| {
			(window.get_proc_address(symbol))
				.map(|f| f as *const _)
				.unwrap_or_default()
		})
	};

	let mut renderer = Renderer::new(viewport, gl);
	renderer.set_clear_color(0.0, 0.0, 0.0, 0.5);

	let watcher_rare_skill_image = load_frienderer_texture(&mut renderer, WATCHER_RARE_SKILL_IMAGE, ImageFormat::Png);
	let ironclad_uncommon_power_image = load_frienderer_texture(&mut renderer, IRONCLAD_UNCOMMON_POWER_IMAGE, ImageFormat::Png);
	let defect_rare_attack_image = load_frienderer_texture(&mut renderer, DEFECT_RARE_ATTACK_IMAGE, ImageFormat::Png);
	let silent_common_attack_image = load_frienderer_texture(&mut renderer, SILENT_COMMON_ATTACK_IMAGE, ImageFormat::Png);

	let awakened_image = load_frienderer_texture(&mut renderer, AWAKENED_IMAGE, ImageFormat::Png);

	let uiua_bitmap_texture = load_frienderer_texture(&mut renderer, UIUA_BITMAP, ImageFormat::Png);
	let uiua_bitmap = BitmapConfiguration {
	    texture: uiua_bitmap_texture.texture_id,
		texture_size: Vec2::new(uiua_bitmap_texture.width as f32, uiua_bitmap_texture.height as f32),
		cell_size: Vec2::new(15., 24.),
		character_list: " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".to_owned(),
		characters_per_row: 19
	};

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            #[allow(clippy::single_match)]
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => {}
            }
        }
        renderer.resize(window.get_size().0, window.get_size().1, scale_factor);

        let mut ui = CatplushContext::begin_layout(window.get_size(), ChildLayoutDirection::TopToBottom);

        ui.open_element(UiElement::new()
            .sizing(SizingMode::Grow, SizingMode::Grow)
            .layout_direction(ChildLayoutDirection::TopToBottom)
            .padding(Padding::all(10))
            .child_gap(10));

            ui.open_element(UiElement::new()
                .rectangle(DARK_BORDER, CornerRadius::all(15.0))
                .sizing(SizingMode::Grow, SizingMode::Fixed(40))
                .padding(Padding::all(5))
                .child_gap(10));

                ui.open_element(topbar_button(100)
                    .border(MONOCHROME_HIGHLIGHT, BorderWidth::all(2)));

                    ui.open_element(UiElement::new()
                        .text(&uiua_bitmap, "File", 16));
                    ui.close_element();

                ui.close_element();

                ui.open_element(topbar_button(75));
                    ui.open_element(UiElement::new()
                        .text(&uiua_bitmap, "Edit", 16));
                    ui.close_element();
                ui.close_element();

                ui.open_element(spacer());
                ui.close_element();

                ui.open_element(topbar_button(75));
                    ui.open_element(UiElement::new()
                        .text(&uiua_bitmap, "Close", 16));
                    ui.close_element();
                ui.close_element();

            ui.close_element();

            ui.open_element(UiElement::new()
                .rectangle(ObjectColor(26, 67, 87, 255), CornerRadius::all(20.0))
                .sizing(SizingMode::Grow, SizingMode::Grow)
                .padding(Padding::all(10))
                .child_gap(10));

                ui.open_element(UiElement::new()
                    .rectangle(ObjectColor(17, 36, 46, 255), CornerRadius::all(15.0))
                    .sizing(SizingMode::Fixed(200), SizingMode::Grow)
                    .layout_direction(ChildLayoutDirection::TopToBottom)
                    .alignment(ChildXAlignment::Center, ChildYAlignment::Top)
                    .padding(Padding::all(10))
                    .child_gap(10));

                    ui.open_element(sidebar_element()
                        .alignment(ChildXAlignment::Center, ChildYAlignment::Center));

                        ui.open_element(UiElement::new()
                            .image(awakened_image, Some(30), None, false));
                        ui.close_element();

                        ui.open_element(spacer());
                        ui.close_element();

                        ui.open_element(UiElement::new()
                            .text(&uiua_bitmap, "Cards", 22));
                        ui.close_element();

                    ui.close_element();
                    for _ in 1..5 {
                        ui.open_element(sidebar_element());
                        ui.close_element();
                    }

                ui.close_element();

                ui.open_element(UiElement::new()
                    .rectangle(ObjectColor(17, 36, 46, 255), CornerRadius::all(17.0))
                    .border(CONTRAST_HIGHLIGHT, BorderWidth::all(3))
                    .sizing(SizingMode::Grow, SizingMode::Grow)
                    .alignment(ChildXAlignment::Center, ChildYAlignment::Top)
                    .layout_direction(ChildLayoutDirection::TopToBottom)
                    .padding(Padding::all(10))
                    .child_gap(10));

                    ui.open_element(UiElement::new()
                        .sizing(SizingMode::Grow, SizingMode::Fit)
                        .alignment(ChildXAlignment::Right, ChildYAlignment::Center)
                        .child_gap(10));

                        card_cell(&mut ui, &uiua_bitmap, "Alpha", watcher_rare_skill_image, CardColor::PURPLE);
                        card_cell(&mut ui, &uiua_bitmap, "Blasphemy", watcher_rare_skill_image, CardColor::PURPLE);
                        card_cell(&mut ui, &uiua_bitmap, "Conjure Blade", watcher_rare_skill_image, CardColor::PURPLE);

                        card_cell(&mut ui, &uiua_bitmap, "Combust", ironclad_uncommon_power_image, CardColor::RED);
                        card_cell(&mut ui, &uiua_bitmap, "Evolve", ironclad_uncommon_power_image, CardColor::RED);
                        card_cell(&mut ui, &uiua_bitmap, "Feel No Pain", ironclad_uncommon_power_image, CardColor::RED);

                    ui.close_element();

                    ui.open_element(UiElement::new()
                        .sizing(SizingMode::Grow, SizingMode::Fit)
                        .alignment(ChildXAlignment::Right, ChildYAlignment::Center)
                        .grow_elements_unevenly()
                        .child_gap(10));

                        card_cell(&mut ui, &uiua_bitmap, "Bane", silent_common_attack_image, CardColor::GREEN);
                        card_cell(&mut ui, &uiua_bitmap, "Dagger Spray", silent_common_attack_image, CardColor::GREEN);
                        card_cell(&mut ui, &uiua_bitmap, "Poisoned Stab", silent_common_attack_image, CardColor::GREEN);

                        card_cell(&mut ui, &uiua_bitmap, "Thunder Strike", defect_rare_attack_image, CardColor::BLUE);
                        card_cell(&mut ui, &uiua_bitmap, "Hyperbeam", defect_rare_attack_image, CardColor::BLUE);
                        card_cell(&mut ui, &uiua_bitmap, "All for One", defect_rare_attack_image, CardColor::BLUE);

                    ui.close_element();

                ui.close_element();

            ui.close_element();

        ui.close_element();

        let render_commands = ui.end_layout();

        friender_render_all(&mut renderer, render_commands);

        window.swap_buffers();
    }
}
