pub mod catplush_main {
    use std::{cmp::Ordering, num::{NonZeroU32}};
    use glam::{Vec4, Vec2};

    /////////////////////////////////////////////////////////////////
    //////////////// UI Heirarchy Data Structures ///////////////////
    /////////////////////////////////////////////////////////////////

    // Holds all of the layout information and currently opened elements for building the ui
    // heirarchy
    pub struct CatplushContext {
        layout_elements: Vec<Node>,

        open_layout_elements: Vec<usize>
    }

    impl CatplushContext {
        pub fn begin_layout(window_size: (i32, i32), layout_direction: ChildLayoutDirection) -> Self {
            let mut new_context = CatplushContext {
                layout_elements: vec![],
                open_layout_elements: vec![]
            };

            new_context.open_element(UiElement::new()
                .sizing(SizingMode::Fixed(window_size.0), SizingMode::Fixed(window_size.1))
                .layout_direction(layout_direction));

            new_context.layout_elements[0].element.final_size_x = window_size.0 as f32;
            new_context.layout_elements[0].element.final_size_y = window_size.1 as f32;

            new_context
        }

        pub fn get_all_elements(&mut self) -> Vec<&UiElement> {
            let mut temp: Vec<&UiElement> = vec![];
            for node in &self.layout_elements[..] {
                temp.push(&node.element);
            }
            temp
        }
    }


    #[derive(Default)]
    struct Node {
        parent: Option<usize>,
        element: UiElement,
        child_elements: Vec<usize>
    }

    ///////////////////////////////////////////////////////
    //////////////// Element Structures ///////////////////
    ///////////////////////////////////////////////////////

    #[derive(Default, Copy, Clone)]
    pub struct ObjectColor( pub u8, pub u8, pub u8, pub u8 );

    impl ObjectColor {
        pub fn as_u32(&self) -> u32 {
            u32::from_be_bytes([self.0, self.1, self.2, self.3])
        }

        /// 0xrrggbbaa
        pub const fn from_u32_hex(hex_code: u32) -> Self {
            let [r, g, b, a] = hex_code.to_be_bytes();
            Self (r, g, b, a)
        }

        pub const TRANSPARENT: Self = ObjectColor(0, 0, 0, 0);

        pub const BLACK: Self = ObjectColor(0  , 0  , 0  , 255);
        pub const WHITE: Self = ObjectColor(255, 255, 255, 255);
        pub const RED:   Self = ObjectColor(255, 0  , 0  , 255);
        pub const GREEN: Self = ObjectColor(0  , 255, 0  , 255);
        pub const BLUE:  Self = ObjectColor(0  , 0  , 255, 255);
    }

    #[derive(PartialEq)]
    pub enum ChildLayoutDirection {
        LeftToRight,
        TopToBottom,
    }

    #[derive(Clone, Copy, PartialEq)]
    pub enum SizingMode {
        Fixed(i32),
        Fit,
        Grow
    }

    pub struct Sizing {
        pub width: SizingMode,
        pub height: SizingMode
    }

    impl Sizing {
        pub fn new(width: SizingMode, height: SizingMode) -> Self {
            Sizing {width, height}
        }

        pub fn both(size: SizingMode) -> Self {
            Sizing {width: size, height: size}
        }
    }

    pub struct SizeLimit {
        min: i32,
        max: i32
    }

    impl Default for SizeLimit {
        fn default() -> Self {
            Self {min: 0, max: i32::MAX}
        }
    }

    #[derive(Default)]
    pub struct SizingLimits {
        width: SizeLimit,
        height: SizeLimit
    }

    #[derive(Default, Copy, Clone)]
    pub struct CornerRadius {
        pub top_right: f32,
        pub bottom_right: f32,
        pub bottom_left: f32,
        pub top_left: f32
    }

    impl CornerRadius {
        pub fn new(top_right: f32, bottom_right: f32, bottom_left: f32, top_left: f32) -> Self {
            CornerRadius {top_right, bottom_right, bottom_left, top_left}
        }

        pub fn all(radius: f32) -> Self {
            CornerRadius {top_right: radius, bottom_right: radius, bottom_left: radius, top_left: radius}
        }

        pub fn as_vec4(&self) -> Vec4 {
            Vec4::new(self.top_right, self.bottom_right, self.bottom_left, self.top_left)
        }
    }

    #[derive(Default, Copy, Clone)]
    pub struct BorderWidth {
        pub top: i32,
        pub right: i32,
        pub bottom: i32,
        pub left: i32
    }

    impl BorderWidth {
        pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
            BorderWidth {top, right, bottom, left}
        }

        pub fn all(width: i32) -> Self {
            BorderWidth {top: width, right: width, bottom: width, left: width}
        }

        pub fn as_vec4(&self) -> Vec4 {
            Vec4::new(self.top as f32, self.right as f32, self.bottom as f32, self.left as f32)
        }
    }

    pub struct Padding {
        left: i32,
        right: i32,
        top: i32,
        bottom: i32,
    }

    impl Padding {
        pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
            Padding {left, right, top, bottom}
        }

        pub fn hv(left_right: i32, top_bottom: i32) -> Self {
            Padding {left: left_right, right: left_right, top: top_bottom, bottom: top_bottom}
        }

        pub fn all(padding: i32) -> Self {
            Padding {left: padding, right: padding, top: padding, bottom: padding}
        }
    }

    #[derive(Default, PartialEq)]
    pub enum ChildXAlignment {
        #[default]
        Left,
        Center,
        Right
    }

    #[derive(Default, PartialEq)]
    pub enum ChildYAlignment {
        #[default]
        Top,
        Center,
        Bottom
    }

    #[derive(Default)]
    pub struct ChildAlignment {
        pub x: ChildXAlignment,
        pub y: ChildYAlignment,
    }

    impl ChildAlignment {
        pub fn new(x_align: ChildXAlignment, y_align: ChildYAlignment) -> Self {
            ChildAlignment {x: x_align, y: y_align}
        }
    }

    pub struct LayoutConfig {
        pub(crate) sizing: Sizing,
        pub(crate) size_constraints: SizingLimits,
        pub(crate) padding: Padding,
        pub(crate) child_gap: i32,
        pub(crate) layout_direction: ChildLayoutDirection,
        pub(crate) child_alignment: ChildAlignment,

        pub(crate) keep_aspect_ratio: bool,
        /// If this is true, the growing algorithm will not try to make all child elements the same size
        /// and elements with different initial sizes will still be different sizes.
        pub(crate) grow_elements_unevenly: bool
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            LayoutConfig {
                sizing: Sizing::both(SizingMode::Fit),
                size_constraints: SizingLimits::default(),
                padding: Padding::all(0),
                child_gap: 0,
                layout_direction: ChildLayoutDirection::LeftToRight,
                child_alignment: ChildAlignment::default(),

                keep_aspect_ratio: true,
                grow_elements_unevenly: false
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct CatplushTextureData {
        pub texture_id: NonZeroU32,
        pub width: i32,
        pub height: i32,
    }

    pub struct CatplushTextData {
        pub(crate) bitmap: BitmapConfiguration,
        pub(crate) text: String,
        pub(crate) font_size: u32,
        pub(crate) split_indices: Vec<u32>,
        pub(crate) break_on_overflow: bool
    }

    #[derive(Default)]
    pub enum ObjectType {
        #[default]
        Unset,
        Rectangle,
        Text ( CatplushTextData ),
        Image ( CatplushTextureData )
    }

    #[derive(Default)]
    pub struct UiElement {
        pub object_type: ObjectType,
        pub id: Option<&'static str>,
        pub layout: LayoutConfig,

        pub color: ObjectColor,
        pub stroke_color: ObjectColor,
        pub corner_radius: CornerRadius,
        pub border_width: BorderWidth,

        pub(crate) final_size_x: f32,
        pub(crate) final_size_y: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
    }

    impl UiElement {
        /// Returns a new empty element. It will fit to its contents by default.
        pub fn new() -> Self {
            Self::default()
        }

        pub fn rectangle(mut self, color: ObjectColor, corner_radius: CornerRadius) -> Self {
            self.object_type = ObjectType::Rectangle;
            self.color = color;
            self.corner_radius = corner_radius;
            self
        }

        /// Gives a rectangle element a colored border.
        pub fn border(mut self, stroke_color: ObjectColor, border_width: BorderWidth) -> Self {
            self.border_width = border_width;
            self.stroke_color = stroke_color;
            self
        }

        pub fn image(mut self, texture: CatplushTextureData, width: Option<i32>, height: Option<i32>, ignore_aspect_ratio: bool) -> Self {
            let width_to_height_ratio = texture.width as f32 / texture.height as f32;
            let height_to_width_ratio = texture.height as f32 / texture.width as f32;

            let actual_height: i32;
            let actual_width: i32;

            if ignore_aspect_ratio {
                actual_width = width.unwrap_or(texture.width);
                actual_height = height.unwrap_or(texture.height);
            } else {
                match (width, height) {
                    (Some(amount), Some(_)) | (Some(amount), None) => {
                        actual_width = amount;
                        actual_height = (actual_width as f32 * width_to_height_ratio) as i32;
                    },
                    (None, Some(amount)) => {
                        actual_height = amount;
                        actual_width = (actual_height as f32 * height_to_width_ratio) as i32;
                    },
                    (None, None) => {
                        actual_width = texture.width;
                        actual_height = texture.height;
                    }
                }
            }

            self.object_type = ObjectType::Image(CatplushTextureData {
                texture_id: texture.texture_id,
                width: actual_width,
                height: actual_height
            });

            self.layout.sizing = Sizing { width: SizingMode::Fixed(actual_width), height: SizingMode::Fixed(actual_height) };
            self
        }

        pub fn text(mut self, bitmap: &BitmapConfiguration, text: &str, font_size: u32, break_on_overflow: bool) -> Self {
            let font_size_factor = font_size as f32 / bitmap.cell_size.y;

            self.layout.sizing = Sizing {
                width: SizingMode::Fixed(((bitmap.cell_size.x * font_size_factor) * text.len() as f32 - 1.0) as i32),
                height: SizingMode::Fixed((bitmap.cell_size.y * font_size_factor) as i32)
            };

            self = self.limit_width(0, ((bitmap.cell_size.x * font_size_factor) * text.len() as f32 - 1.0) as i32);

            for char in text.chars() {
                if bitmap.character_list.find(char).is_none() {
                    panic!("Character {} in the string \"{}\" is not in the bitmap you provided, or the list of characters is missing some that are in the bitmap.", char, text);
                }
            }
            // The .map call gets only the indexes of the matches.
            let new_lines: Vec<u32> = text.match_indices("\n").map(|x| x.0 as u32).collect();

            self.object_type = ObjectType::Text(CatplushTextData {
                bitmap: bitmap.clone(),
                text: text.to_string(),
                font_size,
                split_indices: new_lines,
                break_on_overflow
            });

            self
        }

        /// Sets the gap between each child element.
        pub fn child_gap(mut self, amount: i32) -> Self {
            self.layout.child_gap = amount;
            self
        }

        /// Sets the inside padding.
        pub fn padding(mut self, padding: Padding) -> Self {
            self.layout.padding = padding;
            self
        }

        pub fn sizing(mut self, width: SizingMode, height: SizingMode) -> Self {
            self.layout.sizing = Sizing{ width, height };
            self
        }

        /// This element will stay the same aspect ratio as its initial size when it is resized.
        /// (Not implemented)
        pub fn keep_aspect_ratio(mut self) -> Self {
            self.layout.keep_aspect_ratio = true;
            self
        }

        pub fn grow_elements_unevenly(mut self) -> Self {
            self.layout.grow_elements_unevenly = true;
            self
        }

        pub fn limit_width(mut self, min: i32, max: i32) -> Self {
            self.layout.size_constraints.width = SizeLimit { min, max };
            self
        }

        pub fn limit_height(mut self, min: i32, max: i32) -> Self {
            self.layout.size_constraints.height = SizeLimit { min, max };
            self
        }

        pub fn layout_direction(mut self, direction: ChildLayoutDirection) -> Self {
            self.layout.layout_direction = direction;
            self
        }

        pub fn alignment(mut self, x_align: ChildXAlignment, y_align: ChildYAlignment) -> Self {
            self.layout.child_alignment = ChildAlignment{ x: x_align, y: y_align };
            self
        }

        // Commenting this out until I figure out how to do IDs
        // pub fn id(mut self, id: &'static str) -> Self {
        //     self.id = Some(id);
        //     self
        // }
    }

    pub fn split_multiple_indices<'a>(text: &'a str, indices_to_split: &[usize]) -> Vec<&'a str> {
        if indices_to_split.is_empty() {
            return vec![text]
        }

        let mut segments: Vec<&str> = vec![];
        let mut temporary_split: (&str, &str);

        for (index, split) in indices_to_split.iter().enumerate() {
            temporary_split = text.split_at(*split);

            if index == 0 {
                segments.push(temporary_split.0);
            }
            segments.push(temporary_split.1);
        }

        segments
    }

    impl CatplushContext {
        //////////// Layout Building Functions //////////////
        pub fn open_element(&mut self, element: UiElement) {
            let new_element_index = self.layout_elements.len();
            let mut parent_index: Option<usize> = None;

            if !self.open_layout_elements.is_empty() {
                self.layout_elements[*self.open_layout_elements.last_mut().unwrap()].child_elements.push(new_element_index);
                parent_index = Some(*self.open_layout_elements.last().unwrap());
            }

            self.open_layout_elements.push(new_element_index);
            self.layout_elements.push(Node {
                parent: parent_index,
                element,
                child_elements: vec![]
            });
        }

        pub fn close_element(&mut self) {
            if self.open_layout_elements.len() <= 1 {
                return
            }
            self.open_layout_elements.pop();
        }

        pub(crate) fn size_all(&mut self) {
            self.initial_sizing_along_axis(true, 0);
            self.size_along_axis(true, 0);
            self.wrap_text(0);
            self.initial_sizing_along_axis(false, 0);
            self.size_along_axis(false, 0);
        }

        pub(crate) fn initial_sizing_along_axis(&mut self, left_to_right: bool, current_index: usize) {
            for child_index in self.layout_elements[current_index].child_elements.clone() {
                self.initial_sizing_along_axis(left_to_right, child_index);
            }

            if self.layout_elements[current_index].parent.is_none() {
                return;
            }

            let parent_index = self.layout_elements[current_index].parent.unwrap();
            let [current_node, parent_node] = self.layout_elements.get_disjoint_mut([current_index, parent_index]).unwrap();

            let child_gap = (current_node.child_elements.len() as i32 - 1) * current_node.element.layout.child_gap;

            // Padding
            if left_to_right {
                current_node.element.final_size_x += current_node.element.layout.size_constraints.width.min as f32;
                current_node.element.final_size_x += (current_node.element.layout.padding.left + current_node.element.layout.padding.right) as f32;

                if current_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
                    current_node.element.final_size_x += child_gap as f32;
                }

                match current_node.element.layout.sizing.width {
                    SizingMode::Fixed(size) => {current_node.element.final_size_x = size as f32},
                    SizingMode::Fit => {},
                    SizingMode::Grow => {},
                }

                if parent_node.element.layout.sizing.width == SizingMode::Fit || parent_node.element.layout.sizing.width == SizingMode::Grow {
                    if parent_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
                        parent_node.element.final_size_x += current_node.element.final_size_x;
                    } else {
                        parent_node.element.final_size_x = f32::max(current_node.element.final_size_x, parent_node.element.final_size_x)
                    }
                }
            } else {
                current_node.element.final_size_y += current_node.element.layout.size_constraints.height.min as f32;
                current_node.element.final_size_y += (current_node.element.layout.padding.top + current_node.element.layout.padding.bottom) as f32;

                if current_node.element.layout.layout_direction == ChildLayoutDirection::TopToBottom {
                    current_node.element.final_size_y += child_gap as f32;
                }

                match current_node.element.layout.sizing.height {
                    SizingMode::Fixed(size) => {current_node.element.final_size_y = size as f32},
                    SizingMode::Fit => {},
                    SizingMode::Grow => {},
                }

                if parent_node.element.layout.sizing.height == SizingMode::Fit || parent_node.element.layout.sizing.height == SizingMode::Grow {
                    if parent_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
                        parent_node.element.final_size_y = f32::max(current_node.element.final_size_y, parent_node.element.final_size_y)
                    } else {
                        parent_node.element.final_size_y += current_node.element.final_size_y;
                    }
                }
            }
        }

        pub(crate) fn size_along_axis(&mut self, left_to_right: bool, current_index: usize) {
            let current_node = &mut self.layout_elements[current_index];
            let mut growable_elements: Vec<usize> = vec![];

            let sizing_along_axis =
                (left_to_right && current_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight)
                || (!left_to_right && current_node.element.layout.layout_direction == ChildLayoutDirection::TopToBottom);

            let padding =
                if left_to_right {current_node.element.layout.padding.left + current_node.element.layout.padding.right}
                else { current_node.element.layout.padding.top + current_node.element.layout.padding.bottom };
            let child_gap = (current_node.child_elements.len() as i32 - 1) * current_node.element.layout.child_gap;
            let parent_size = if left_to_right {current_node.element.final_size_x} else {current_node.element.final_size_y};

            let mut inner_content_size = 0.0;
            for child in current_node.child_elements.clone() {
                if left_to_right {
                    inner_content_size += self.layout_elements[child].element.final_size_x;
                } else {
                    inner_content_size += self.layout_elements[child].element.final_size_y;
                }

                let child_sizing_mode =
                    if left_to_right { self.layout_elements[child].element.layout.sizing.width == SizingMode::Grow }
                    else { self.layout_elements[child].element.layout.sizing.height == SizingMode::Grow };
                if child_sizing_mode {
                    growable_elements.push(child);
                }
            }

            if sizing_along_axis {
                let mut size_to_distribute = parent_size - padding as f32 - child_gap as f32 - inner_content_size;


                if size_to_distribute > 0.0 && !growable_elements.is_empty() {
                    // If the element is able to grow and is not at its max size, keep growing it.
                    // I apologize for this closure.
                    growable_elements.retain(
                        |&x|
                        left_to_right && self.layout_elements[x].element.final_size_x < self.layout_elements[x].element.layout.size_constraints.width.max as f32
                        && self.layout_elements[x].element.layout.sizing.width == SizingMode::Grow
                        ||
                        !left_to_right && self.layout_elements[x].element.final_size_y < self.layout_elements[x].element.layout.size_constraints.height.max as f32
                        && self.layout_elements[x].element.layout.sizing.height == SizingMode::Grow
                    );

                    while size_to_distribute > 0.01 && !growable_elements.is_empty() {
                        let mut smallest_size = f32::MAX;
                        let mut second_smallest_size = f32::MAX;
                        let mut width_to_add = size_to_distribute;

                        for child_index in &growable_elements {
                            let child_size = if left_to_right { self.layout_elements[*child_index].element.final_size_x } else { self.layout_elements[*child_index].element.final_size_y };
                            match child_size.total_cmp(&smallest_size) {
                                Ordering::Less => {
                                    second_smallest_size = smallest_size;
                                    smallest_size = child_size;
                                },
                                Ordering::Equal => { continue; },
                                Ordering::Greater => {
                                    second_smallest_size = f32::min(second_smallest_size, child_size);
                                    width_to_add = second_smallest_size - smallest_size;
                                }
                            }
                        }

                        width_to_add = f32::min(width_to_add, size_to_distribute / (growable_elements.len() as f32));

                        for child_index in &growable_elements {
                            let grow_elements_unevenly = self.layout_elements[*child_index].element.layout.grow_elements_unevenly;
                            let max_size =
                                if left_to_right { self.layout_elements[*child_index].element.layout.size_constraints.width.max }
                                else { self.layout_elements[*child_index].element.layout.size_constraints.height.max } as f32;
                            let child_size =
                                if left_to_right { &mut self.layout_elements[*child_index].element.final_size_x }
                                else { &mut self.layout_elements[*child_index].element.final_size_y };
                            let initial_size = *child_size;

                            // For some reason this check makes ONLY the smallest element grow (sort of) bleghhhh
                            if *child_size == smallest_size && !grow_elements_unevenly {
                                *child_size += width_to_add;
                                size_to_distribute -= *child_size - initial_size;
                                if *child_size >= max_size {
                                    *child_size = max_size;
                                }
                            }
                        }
                    }
                }
            } else {
                for child_index in &growable_elements {
                    let child_size =
                        if left_to_right { &mut self.layout_elements[*child_index].element.final_size_x }
                        else { &mut self.layout_elements[*child_index].element.final_size_y };

                    let max_size = parent_size - padding as f32;

                    *child_size = max_size
                }
            }

            for child_index in self.layout_elements[current_index].child_elements.clone() {
                self.size_along_axis(left_to_right, child_index);
            }
        }

        pub(crate) fn wrap_text(&mut self, current_index: usize) {
            // Not sure if this actually does need a depth first search
            for child_index in self.layout_elements[current_index].child_elements.clone() {
                self.wrap_text(child_index);
            }

            if self.layout_elements[current_index].parent.is_none() {
                return;
            }

            let parent_index = self.layout_elements[current_index].parent.unwrap();

            let character_width: f32;
            let line_height: f32;
            let text: String;
            let break_on_overflow: bool;

            match &mut self.layout_elements[current_index].element.object_type {
                ObjectType::Text(text_data) => {
                    character_width = text_data.bitmap.cell_size.x;
                    line_height = text_data.bitmap.cell_size.y;
                    text = text_data.text.clone();
                    break_on_overflow = text_data.break_on_overflow
                },
                _ => { return }
            }

            let padding = self.layout_elements[parent_index].element.layout.padding.left + self.layout_elements[parent_index].element.layout.padding.right;
            let child_gap = (self.layout_elements[parent_index].child_elements.len() as i32 - 1) * self.layout_elements[parent_index].element.layout.child_gap;
            let parent_size = self.layout_elements[parent_index].element.final_size_x;

            let mut inner_content_size = 0.0;
            for child in self.layout_elements[current_index].child_elements.clone() {
                inner_content_size += self.layout_elements[child].element.final_size_x;
            }

            let available_width = parent_size - padding as f32 - child_gap as f32 - inner_content_size;
            let number_of_wraps = ((character_width * text.len() as f32) / available_width) as i32;
            let mut indexes_to_wrap: Vec<u32> = vec![];
            if number_of_wraps == 0 { return }

            for _ in 1..number_of_wraps {
                let character_on_border: u32 = (available_width / character_width) as u32;

                let last_split = *indexes_to_wrap.last().unwrap_or(&0) as usize;
                let space_to_wrap: usize = match &text[last_split..character_on_border as usize].rfind(" ") {
                    Some(index) => { *index },
                    None => {
                        if break_on_overflow {
                            character_on_border as usize
                        } else {
                            continue;
                        }
                    }
                };

                indexes_to_wrap.push(space_to_wrap as u32);
                self.layout_elements[current_index].element.final_size_y += line_height;
            }

            // what is this syntax huh
            if let ObjectType::Text(text_data) = &mut self.layout_elements[current_index].element.object_type {
                text_data.split_indices.append(&mut indexes_to_wrap);
            }
        }

        pub(crate) fn position_all(&mut self) {
            self.position_along_axis(true, 0);
            self.position_along_axis(false, 0);
        }

        pub(crate) fn position_along_axis(&mut self, left_to_right: bool, current_index: usize) {
            let mut child_num = 1;
            let mut total_child_offset = 0.0;

            for child in self.layout_elements[current_index].child_elements.clone() {
                if left_to_right {
                    match self.layout_elements[current_index].element.layout.layout_direction {
                        ChildLayoutDirection::LeftToRight => {
                            self.layout_elements[child].element.final_pos_x = total_child_offset
                                + self.layout_elements[current_index].element.final_pos_x
                                + self.layout_elements[current_index].element.layout.padding.left as f32
                                + (self.layout_elements[current_index].element.layout.child_gap * (child_num - 1)) as f32;
                        }
                        ChildLayoutDirection::TopToBottom => {
                            self.layout_elements[child].element.final_pos_x = self.layout_elements[current_index].element.final_pos_x
                                + self.layout_elements[current_index].element.layout.padding.left as f32;
                        }
                    };

                    total_child_offset += self.layout_elements[child].element.final_size_x;
                } else {
                    match self.layout_elements[current_index].element.layout.layout_direction {
                        ChildLayoutDirection::LeftToRight => {
                            self.layout_elements[child].element.final_pos_y = self.layout_elements[current_index].element.final_pos_y
                                + self.layout_elements[current_index].element.layout.padding.top as f32;
                        }
                        ChildLayoutDirection::TopToBottom => {
                            self.layout_elements[child].element.final_pos_y = total_child_offset
                                + self.layout_elements[current_index].element.final_pos_y
                                + self.layout_elements[current_index].element.layout.padding.top as f32
                                + (self.layout_elements[current_index].element.layout.child_gap * (child_num - 1)) as f32;
                        }
                    };

                    total_child_offset += self.layout_elements[child].element.final_size_y;
                }
                child_num += 1;
            }

            let aligning_along_axis =
                if left_to_right { self.layout_elements[current_index].element.layout.layout_direction == ChildLayoutDirection::LeftToRight
                   && self.layout_elements[current_index].element.layout.child_alignment.x != ChildXAlignment::Left }
                else { self.layout_elements[current_index].element.layout.layout_direction == ChildLayoutDirection::TopToBottom
                    && self.layout_elements[current_index].element.layout.child_alignment.y != ChildYAlignment::Top };

            let padding =
                if left_to_right {self.layout_elements[current_index].element.layout.padding.left + self.layout_elements[current_index].element.layout.padding.right}
                else { self.layout_elements[current_index].element.layout.padding.top + self.layout_elements[current_index].element.layout.padding.bottom };
            let child_gap = (self.layout_elements[current_index].child_elements.len() as i32 - 1) * self.layout_elements[current_index].element.layout.child_gap;
            let parent_size = if left_to_right {self.layout_elements[current_index].element.final_size_x} else {self.layout_elements[current_index].element.final_size_y};

            let mut inner_content_size = 0.0;
            for child in self.layout_elements[current_index].child_elements.clone() {
                if left_to_right {
                    inner_content_size += self.layout_elements[child].element.final_size_x;
                } else {
                    inner_content_size += self.layout_elements[child].element.final_size_y;
                }
            }

            let mut distance_to_add = parent_size - padding as f32 - child_gap as f32 - inner_content_size;

            for child_index in self.layout_elements[current_index].child_elements.clone() {
                if !aligning_along_axis {
                    distance_to_add =
                        if left_to_right {
                            parent_size - padding as f32 - self.layout_elements[child_index].element.final_size_x
                        } else {
                            parent_size - padding as f32 - self.layout_elements[child_index].element.final_size_y
                        };
                }
                if left_to_right {
                    match self.layout_elements[current_index].element.layout.child_alignment.x {
                        ChildXAlignment::Left => {},
                        ChildXAlignment::Center => {
                            self.layout_elements[child_index].element.final_pos_x += distance_to_add/2.0
                        },
                        ChildXAlignment::Right => {
                            self.layout_elements[child_index].element.final_pos_x += distance_to_add
                        },
                    }
                } else {
                    match self.layout_elements[current_index].element.layout.child_alignment.y {
                        ChildYAlignment::Top => {},
                        ChildYAlignment::Center => {
                            self.layout_elements[child_index].element.final_pos_y += distance_to_add/2.0;
                        },
                        ChildYAlignment::Bottom => {
                            self.layout_elements[child_index].element.final_pos_y += distance_to_add;
                        },
                    }
                }
            }

            for child in self.layout_elements[current_index].child_elements.clone() {
                self.position_along_axis(left_to_right, child);
            }
        }

        // Solves all sizing and positioning and returns a set of render commands for passing to the
        // renderer
        pub fn end_layout(mut self) -> Vec<RenderCommand> {
            self.open_layout_elements.clear();

            self.size_all();
            self.position_all();

            let mut render_commands: Vec<RenderCommand> = vec![];
            for node in &self.layout_elements[1..self.layout_elements.len()] {
                let element = &node.element;
                let bounding_box = BoundingBox { x: element.final_pos_x, y: element.final_pos_y, width: element.final_size_x, height: element.final_size_y };
                let render_data = match &element.object_type {
                    ObjectType::Unset => RenderData::NoType,
                    ObjectType::Rectangle => {
                        RenderData::RectangleData(RectangleRenderData { color: element.color, stroke_color: element.stroke_color, corner_radius: element.corner_radius, border_width: element.border_width })
                    },
                    ObjectType::Text(data) => {
                        RenderData::TextData(TextRenderData {
                            bitmap: data.bitmap.clone(),
                            text: data.text.clone(),
                            font_size: data.font_size,
                            split_indices: data.split_indices.clone()
                        })
                    },
                    ObjectType::Image(data) => {
                        RenderData::ImageData(TextureRenderData { texture_id: data.texture_id, width: data.width, height: data.height })
                    }
                };

                let id = element.id.unwrap_or_default();

                render_commands.push( RenderCommand { bounding_box, render_data, id } );
            }

            render_commands
        }
    }

    /////////////////////////////////////////////////////
    ////////////////  Render Structures  ////////////////
    /////////////////////////////////////////////////////

    #[derive(Copy, Clone)]
    pub(crate) struct BoundingBox {
        pub(crate) x: f32,
        pub(crate) y: f32,
        pub(crate) width: f32,
        pub(crate) height: f32
    }

    pub(crate) struct RectangleRenderData {
        pub(crate) color: ObjectColor,
        pub(crate) stroke_color: ObjectColor,
        pub(crate) corner_radius: CornerRadius,
        pub(crate) border_width: BorderWidth
    }


    pub(crate) struct TextRenderData {
        pub(crate) bitmap: BitmapConfiguration,
        pub(crate) text: String,
        pub(crate) font_size: u32,
        pub(crate) split_indices: Vec<u32>
    }

    pub(crate) struct TextureRenderData {
        pub(crate) texture_id: NonZeroU32,
        pub(crate) width: i32,
        pub(crate) height: i32,

        // Need to implement these myself.
        // pub(crate) tint: ObjectColor,
        // pub(crate) corner_radius: CornerRadius
    }

    pub(crate) enum RenderData {
        NoType,
        RectangleData(RectangleRenderData),
        TextData(TextRenderData),
        ImageData(TextureRenderData)
    }

    pub struct RenderCommand {
        pub(crate) bounding_box: BoundingBox,

        pub(crate) render_data: RenderData,

        pub id: &'static str,
    }

    #[derive(Clone)]
    pub struct BitmapConfiguration {
        pub texture: NonZeroU32,
        pub texture_size: Vec2,
        pub cell_size: Vec2,
        pub character_list: String,
        pub characters_per_row: u8
    }
}

pub mod catplush_friend {
    use crate::catplush_main::*;
    use std::num::NonZeroU32;
    use frienderer::{DrawCommand, Quad, RRect, RawImage, Renderer};
    use image::{DynamicImage, ImageFormat};
    use glow::{NativeTexture};
    use glam::{Vec2};


    pub fn friender_render_all(renderer: &mut Renderer, render_commands: Vec<RenderCommand>) {
        for render_command in render_commands {
            match render_command.render_data {
                RenderData::NoType => {},
                RenderData::RectangleData(data) => {
                    renderer.push_draw_command(DrawCommand::RRect(RRect {
                        pos: Vec2::new(render_command.bounding_box.x, render_command.bounding_box.y),
                        size: Vec2::new(render_command.bounding_box.width, render_command.bounding_box.height),
                        border_radius: data.corner_radius.as_vec4(),
                        border_width: data.border_width.as_vec4(),
                        fill_color: data.color.as_u32(),
                        stroke_color: data.stroke_color.as_u32()
                    }));
                },
                RenderData::ImageData(data) => {
                    renderer.push_draw_command(DrawCommand::TextureQuad(
                        Quad {
                            pos: Vec2::new(render_command.bounding_box.x, render_command.bounding_box.y),
                            size: Vec2::new(data.width as f32, data.height as f32),
                            origin: Vec2::ZERO,
                            uv_pos: Vec2::ZERO,
                            uv_size: Vec2::ONE,
                            rotation: 0.0
                        },
                        NativeTexture(data.texture_id)
                    ));
                },
                RenderData::TextData(mut data) => {
                    render_text(renderer, data.text, &mut data.split_indices, Vec2::new(render_command.bounding_box.x, render_command.bounding_box.y), data.bitmap, data.font_size);
                }
            }
        }

        renderer.draw();
    }

    pub(crate) fn render_text(renderer: &mut Renderer, text: String, split_indexes: &mut [u32], position: Vec2, bitmap: BitmapConfiguration, font_size: u32) {
        split_indexes.sort();
        let actual_split_indexes: Vec<usize> = split_indexes.iter().map(|x| *x as usize).collect();
        let lines = split_multiple_indices(&text, &actual_split_indexes);
        for line in lines {
            for (i, char) in line.chars().enumerate() {
                let scale_factor = font_size as f32 / bitmap.cell_size.y;
                let index_in_bitmap = bitmap.character_list.find(char).unwrap() as i32;

                let uv_cell_size = bitmap.cell_size / bitmap.texture_size;
                let x = (index_in_bitmap % bitmap.characters_per_row as i32) as f32 * uv_cell_size.x;
                let y = (index_in_bitmap / bitmap.characters_per_row as i32) as f32 * uv_cell_size.y;

                renderer.push_draw_command(DrawCommand::TextureQuad(
                    Quad {
                        pos: Vec2::new(position.x + ((i as f32 * bitmap.cell_size.x) * scale_factor), position.y),
                        size: bitmap.cell_size * scale_factor,
                        origin: Vec2::ZERO,
                        uv_pos: Vec2::new(x, y),
                        uv_size: uv_cell_size,
                        rotation: 0.0
                    },
                    NativeTexture(bitmap.texture)
                ));
            }
        }
    }

    pub fn load_texture_from_image(renderer: &mut Renderer, image: &DynamicImage) -> NativeTexture {
        renderer.upload_texture(RawImage {
            width: image.width(),
            height: image.height(),
            pixels: image.as_bytes()
        })
    }

    pub fn get_texture_id(texture: &NativeTexture) -> NonZeroU32 { texture.0 }

    pub fn load_frienderer_texture(renderer: &mut Renderer, image_data: &[u8], format: ImageFormat) -> CatplushTextureData {
        let image = image::load_from_memory_with_format(image_data, format).unwrap();
        let texture = renderer.upload_texture(RawImage {
            width: image.width(),
            height: image.height(),
            pixels: image.as_bytes()
        }).0;

        CatplushTextureData {
            texture_id: texture,
            width: image.width() as i32,
            height: image.height() as i32
        }
    }
}
