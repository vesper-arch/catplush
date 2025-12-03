
#[allow(unused)]
pub mod clay_main {
    /////////////////////////////////////////////////////////////////
    //////////////// UI Heirarchy Data Structures ///////////////////
    /////////////////////////////////////////////////////////////////

    use std::{cmp::Ordering};
    use std::ffi::c_void;

    // Holds all of the layout information and currently opened elements for building the ui
    // heirarchy
    pub struct ClayContext {
        layout_elements: Vec<Node>,

        open_layout_elements: Vec<usize>
    }

    impl ClayContext {
        pub fn begin_layout(window_size: (i32, i32), layout_direction: ChildLayoutDirection) -> Self {
            let mut new_context = ClayContext {
                layout_elements: vec![],
                open_layout_elements: vec![]
            };

            new_context.open_element(ClayElement::new()
                .sizing(SizingMode::Fixed(window_size.0), SizingMode::Fixed(window_size.1))
                .layout_direction(layout_direction));

            new_context.layout_elements[0].element.final_size_x = window_size.0 as f32;
            new_context.layout_elements[0].element.final_size_y = window_size.1 as f32;

            new_context
        }

        fn get_last_opened_element(&mut self) -> Option<&mut Node> {
            let last_opened_element_index: usize = *self.open_layout_elements.last().expect("There are no currently opened elements");

            Some(self.layout_elements.get_mut(last_opened_element_index).unwrap())
        }

        pub fn get_all_elements(&mut self) -> Vec<&ClayElement> {
            let mut temp: Vec<&ClayElement> = vec![];
            for node in &self.layout_elements[..] {
                temp.push(&node.element);
            }
            temp
        }
    }


    #[derive(Default)]
    struct Node {
        parent: Option<usize>,
        element: ClayElement,
        child_elements: Vec<usize>
    }

    impl Node {
       fn new(element: ClayElement, parent: usize) -> Self {
           Node {
               parent: Some(parent),
               element,
               child_elements: vec![]
           }
       }
    }

    ///////////////////////////////////////////////////////
    //////////////// Element Structures ///////////////////
    ///////////////////////////////////////////////////////

    #[derive(Default, Copy, Clone)]
    pub struct ObjectColor( pub u8, pub u8, pub u8, pub u8 );

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

    pub struct SizeConstraint {
        min: i32,
        max: i32
    }

    impl Default for SizeConstraint {
        fn default() -> Self {
            Self {min: 0, max: 999999999}
        }
    }

    #[derive(Default, Copy, Clone)]
    pub struct CornerRadius {
        pub top_right: f32,
        pub top_left: f32,
        pub bottom_left: f32,
        pub bottom_right: f32
    }

    impl CornerRadius {
        pub fn new(top_right: f32, top_left: f32, bottom_left: f32, bottom_right: f32) -> Self {
            CornerRadius {top_right, top_left, bottom_left, bottom_right}
        }

        pub fn all(radius: f32) -> Self {
            CornerRadius {top_right: radius, top_left: radius, bottom_left: radius, bottom_right: radius}
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

    #[derive(Default)]
    pub enum ChildXAlignment {
        #[default]
        AlignXLeft,
        AlignXCenter,
        AlignXRight
    }

    #[derive(Default)]
    pub enum ChildYAlignment {
        #[default]
        AlignYTop,
        AlignYCenter,
        AlignYBottom
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
        pub sizing: Sizing,
        pub size_constraints: (SizeConstraint, SizeConstraint),
        pub padding: Padding,
        pub child_gap: i32,
        pub layout_direction: ChildLayoutDirection,
        pub child_alignment: ChildAlignment
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            LayoutConfig {
                sizing: Sizing::both(SizingMode::Fit),
                size_constraints: (SizeConstraint::default(), SizeConstraint::default()),
                padding: Padding::all(0),
                child_gap: 0,
                layout_direction: ChildLayoutDirection::LeftToRight,
                child_alignment: ChildAlignment::default()
            }
        }
    }

    #[derive(Copy, Clone)]
    pub struct ClayImageData {
        pub(crate) data: *mut c_void,
        pub(crate) width: i32,
        pub(crate) height: i32,
        pub(crate) mipmaps: i32,
        pub(crate) format: i32
    }

    #[derive(Default)]
    pub enum ElementType {
        #[default]
        Unset,
        Rectangle,
        Border ( i32 ),
        Text ( String, u8 ),
        Image ( ClayImageData )
    }

    #[derive(Default)]
    pub struct ClayElement {
        pub object_type: ElementType,
        pub id: Option<&'static str>,
        pub layout: LayoutConfig,

        pub color: ObjectColor,
        pub corner_radius: CornerRadius,

        pub(crate) final_size_x: f32,
        pub(crate) final_size_y: f32,
        pub(crate) final_pos_x: f32,
        pub(crate) final_pos_y: f32,
    }

    impl ClayElement {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn rectangle(mut self, color: ObjectColor, corner_radius: CornerRadius) -> Self {
            self.object_type = ElementType::Rectangle;
            self.color = color;
            self.corner_radius = corner_radius;
            self
        }

        pub fn border(mut self, color: ObjectColor, corner_radius: CornerRadius, border_width: i32) -> Self {
            self.object_type = ElementType::Border(border_width);
            self.color = color;
            self.corner_radius = corner_radius;
            self
        }

        pub fn text(mut self, text: String, color: ObjectColor, font_size: u8) -> Self {
            self.object_type = ElementType::Text( text, font_size );
            self.color = color;
            self
        }

        pub fn child_gap(mut self, amount: i32) -> Self {
            self.layout.child_gap = amount;
            self
        }

        pub fn padding(mut self, padding: Padding) -> Self {
            self.layout.padding = padding;
            self
        }

        pub fn sizing(mut self, x: SizingMode, y: SizingMode) -> Self {
            self.layout.sizing = Sizing{ width: x, height: y };
            self
        }

        pub fn constrain(mut self, width: SizeConstraint, height: SizeConstraint) -> Self {
            self.layout.size_constraints = (width, height);
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

        pub fn id(mut self, id: &'static str) -> Self {
            self.id = Some(id);
            self
        }
    }

    //////////////////////////////////////////////////////////
    ////////////////  Render Structures  /////////////////////
    //////////////////////////////////////////////////////////

    #[derive(Copy, Clone)]
    pub(crate) struct BoundingBox {
        pub(crate) x: f32,
        pub(crate) y: f32,
        pub(crate) width: f32,
        pub(crate) height: f32
    }

    pub(crate) struct RectangleRenderData {
        pub(crate) color: ObjectColor,
        pub(crate) corner_radius: CornerRadius
    }

    pub(crate) struct BorderRenderData {
        pub(crate) color: ObjectColor,
        pub(crate) corner_radius: CornerRadius,
        pub(crate) width: i32
    }

    pub(crate) struct TextRenderData {
        pub(crate) color: ObjectColor,

        pub(crate) font_size: u8,
        pub(crate) letter_spacing: u8,
        pub(crate) line_height: u8
    }

    pub(crate) struct ImageRenderData {
        pub(crate) data: *mut c_void,
        pub(crate) width: i32,
        pub(crate) height: i32,
        pub(crate) mipmaps: i32,
        pub(crate) format: i32,

        pub(crate) tint: ObjectColor,
        pub(crate) corner_radius: CornerRadius
    }

    pub(crate) enum RenderData {
        NoType,
        RectangleData(RectangleRenderData),
        BorderData(BorderRenderData),
        TextData(TextRenderData),
        ImageData(ImageRenderData)
    }

    pub struct RenderCommand {
        pub(crate) bounding_box: BoundingBox,

        pub(crate) render_data: RenderData,

        pub id: &'static str,
    }

    impl ClayContext {
        //////////// Finalizing Functions //////////////

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
                    ElementType::Unset => RenderData::NoType,
                    ElementType::Rectangle => { RenderData::RectangleData(RectangleRenderData { color: element.color, corner_radius: element.corner_radius }) },
                    ElementType::Border(width) => { RenderData::BorderData(BorderRenderData { color: element.color, corner_radius: element.corner_radius, width: *width }) }
                    ElementType::Text(contents, font_size) => {
                        RenderData::TextData(TextRenderData { color: element.color, font_size: *font_size, letter_spacing: 0, line_height: 0 })
                    },
                    ElementType::Image(file) => {
                        RenderData::ImageData(ImageRenderData { data: file.data, width: file.width, height: file.height, mipmaps: file.mipmaps, format: file.format, tint: element.color, corner_radius: element.corner_radius })
                    }
                };

                let id = element.id.unwrap_or_default();

                render_commands.push( RenderCommand { bounding_box, render_data, id } );
            }

            render_commands
        }

        //////////// Layout Building Functions //////////////

        pub fn open_element(&mut self, element: ClayElement) {
            let new_element_index = self.layout_elements.len();
            let mut parent_element: usize = 0;

            if !self.open_layout_elements.is_empty() {
                self.layout_elements[*self.open_layout_elements.last_mut().unwrap()].child_elements.push(new_element_index);
                parent_element = *self.open_layout_elements.last().unwrap();
            }

            self.open_layout_elements.push(new_element_index);
            self.layout_elements.push(Node::new(element, parent_element));
        }

        pub fn close_element(&mut self) {
            // Naturally gets called in Depth First Order so we can do fixed sizing and fit sizing
            // widths right here
            if self.open_layout_elements.len() <= 1 {
                return
            }

            let layout_slice = &mut self.layout_elements[..];
            let last_opened_element = *self.open_layout_elements.last().unwrap();
            let parent_element = layout_slice[last_opened_element].parent.unwrap();

            let [parent_node, closing_node] = layout_slice.get_disjoint_mut([parent_element, last_opened_element]).unwrap();

            // Padding
            closing_node.element.final_size_x += (closing_node.element.layout.padding.left + closing_node.element.layout.padding.right) as f32;
            closing_node.element.final_size_y += (closing_node.element.layout.padding.top + closing_node.element.layout.padding.bottom) as f32;

            // Fixed Sizing
            match closing_node.element.layout.sizing.width {
                SizingMode::Fixed(size) => {closing_node.element.final_size_x = size as f32},
                SizingMode::Fit => {},
                SizingMode::Grow => {},
            }
            match closing_node.element.layout.sizing.height {
                SizingMode::Fixed(size) => {closing_node.element.final_size_y = size as f32},
                SizingMode::Fit => {},
                SizingMode::Grow => {},
            }

            let child_gap = (parent_node.child_elements.len() as i32 - 1) * parent_node.element.layout.child_gap;

            // Fit Sizing
            if parent_node.element.layout.sizing.width == SizingMode::Fit || parent_node.element.layout.sizing.width == SizingMode::Grow {
                if parent_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
                    parent_node.element.final_size_x += child_gap as f32;
                    parent_node.element.final_size_x += closing_node.element.final_size_x;
                } else {
                    parent_node.element.final_size_x = f32::max(closing_node.element.final_size_x, parent_node.element.final_size_x)
                }

            }
            if parent_node.element.layout.sizing.height == SizingMode::Fit || parent_node.element.layout.sizing.height == SizingMode::Grow {
                if parent_node.element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
                    parent_node.element.final_size_y = f32::max(closing_node.element.final_size_y, parent_node.element.final_size_y);
                } else {
                    parent_node.element.final_size_y += child_gap as f32;
                    parent_node.element.final_size_y += closing_node.element.final_size_y;
                }
            }

            self.open_layout_elements.pop();
        }

        pub(crate) fn size_all(&mut self) {
            self.size_along_axis(true, 0);
            self.size_along_axis(false, 0);
        }

        pub(crate) fn size_along_axis(&mut self, left_to_right: bool, current_index: usize) {
            let mut current_node = &mut self.layout_elements[current_index];
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

                growable_elements.retain(
                    |&x|
                    (left_to_right && self.layout_elements[x].element.layout.sizing.width == SizingMode::Grow)
                    ||
                    (!left_to_right && self.layout_elements[x].element.layout.sizing.height == SizingMode::Grow)
                );

                if size_to_distribute > 0.0 {
                    let mut smallest_size = f32::MAX;
                    let mut second_smallest_size = f32::MAX;
                    let mut width_to_add = size_to_distribute;

                    for child_index in &growable_elements {
                        let child_size = if left_to_right { self.layout_elements[*child_index].element.final_size_x } else { self.layout_elements[*child_index].element.final_size_y };
                        for child_index in &growable_elements {
                            let child_size = if left_to_right { self.layout_elements[*child_index].element.final_size_x } else { self.layout_elements[*child_index].element.final_size_y };
                            match child_size.total_cmp(&smallest_size) {
                                Ordering::Less => { second_smallest_size = smallest_size; smallest_size = child_size; },
                                Ordering::Equal => { continue; },
                                Ordering::Greater => { second_smallest_size = f32::min(second_smallest_size, child_size); width_to_add = second_smallest_size - smallest_size; }
                            }
                        }

                        width_to_add = f32::min(width_to_add, size_to_distribute / (growable_elements.len() as f32));

                        for child_index in &growable_elements {
                            let mut child_size =
                                if left_to_right { &mut self.layout_elements[*child_index].element.final_size_x }
                                else { &mut self.layout_elements[*child_index].element.final_size_y };
                            let initial_size = *child_size;

                            *child_size += width_to_add;
                            size_to_distribute -= (*child_size - initial_size)
                        }
                    }
                }
            } else {
                for child_index in &growable_elements {
                    let mut child_size =
                        if left_to_right { &mut self.layout_elements[*child_index].element.final_size_x }
                        else { &mut self.layout_elements[*child_index].element.final_size_y };

                    let max_size = parent_size - padding as f32;

                    *child_size = max_size
                }
            }

            for child in self.layout_elements[current_index].child_elements.clone() {
                self.size_along_axis(left_to_right, child);
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

            for child in self.layout_elements[current_index].child_elements.clone() {
                self.position_along_axis(left_to_right, child);
            }
        }
    }
}

pub mod clay_raylib {
    use raylib::prelude::*;
    use crate::clay_main::{self, ClayImageData};
    use crate::clay_main::{RenderCommand, RenderData};

    pub fn raylib_render_all(render_commands: Vec<RenderCommand>, draw_handle: &mut RaylibDrawHandle<'_>) {
        for command in render_commands {
            let bounding_box = command.bounding_box;
            match command.render_data {
                RenderData::NoType => {},
                RenderData::RectangleData(data) => {
                    if data.corner_radius.top_left != 0.0 {
                        let radius = (data.corner_radius.top_left * 2.0) / (if bounding_box.width > bounding_box.height { bounding_box.height } else { bounding_box.width });
                        draw_handle.draw_rectangle_rounded(clay_to_raylib_rect(&bounding_box), radius, 16, clay_to_raylib_color(&data.color));
                    } else {
                        draw_handle.draw_rectangle(bounding_box.x as i32, bounding_box.y as i32, bounding_box.width as i32, bounding_box.height as i32, clay_to_raylib_color(&data.color));
                    }
                }
                RenderData::BorderData(data) => {
                    if data.corner_radius.top_left != 0.0 {
                        let radius = (data.corner_radius.top_left * 2.0) / (if bounding_box.width > bounding_box.height { bounding_box.height } else { bounding_box.width });
                        draw_handle.draw_rectangle_rounded_lines_ex(clay_to_raylib_rect(&bounding_box), radius, 8, data.width as f32, clay_to_raylib_color(&data.color));
                    } else {
                        draw_handle.draw_rectangle_lines_ex(clay_to_raylib_rect(&bounding_box), data.width as f32, clay_to_raylib_color(&data.color));
                    }
                }
                // not even sure how to handle text yet sob. Images do NOT work with this implementation. Raylib will have to be changed out.
                RenderData::TextData(_) | RenderData::ImageData(_) => todo!()

            }
        }
    }

    pub(crate) fn clay_to_raylib_rect(object: &clay_main::BoundingBox) -> Rectangle {
        Rectangle {
            x: object.x,
            y: object.y,
            width: object.width,
            height: object.height
        }
    }

    pub fn clay_to_raylib_color(color: &clay_main::ObjectColor) -> Color {
        Color {
            r: color.0,
            g: color.1,
            b: color.2,
            a: color.3
        }
    }

    pub fn raylib_to_clay_image(image: &Texture2D) -> clay_main::ClayImageData {
        let raw_image = image.load_image().unwrap().to_raw();

        ClayImageData { data: raw_image.data, width: raw_image.width, height: raw_image.height, mipmaps: raw_image.mipmaps, format: raw_image.format }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 4, 6);
    }
}
