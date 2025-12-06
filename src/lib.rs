pub mod catplush_main {
    /////////////////////////////////////////////////////////////////
    //////////////// UI Heirarchy Data Structures ///////////////////
    /////////////////////////////////////////////////////////////////

    use std::{cmp::Ordering};
    use std::ffi::c_void;
    use glam::{Vec2, Vec4};

    // Holds all of the layout information and currently opened elements for building the ui
    // heirarchy
    pub struct CatplushContext<'a> {
        layout_elements: Vec<Node<'a>>,

        open_layout_elements: Vec<usize>
    }

    impl CatplushContext<'_> {
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
    struct Node<'a> {
        parent: Option<usize>,
        element: UiElement<'a>,
        child_elements: Vec<usize>
    }

    impl<'a> Node<'a> {
       fn new(element: UiElement<'a>, parent: usize) -> Self {
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

    impl ObjectColor {
        pub fn as_u32(&self) -> u32 {
            u32::from_be_bytes([self.0, self.1, self.2, self.3])
        }
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

        pub fn as_vec4(&self) -> Vec4 {
            Vec4::new(self.top_right, self.top_left, self.bottom_left, self.bottom_right)
        }
    }

    #[derive(Default, Copy, Clone)]
    pub struct BorderWidth {
        pub left: i32,
        pub right: i32,
        pub top: i32,
        pub bottom: i32
    }

    impl BorderWidth {
        pub fn new(left: i32, right: i32, top: i32, bottom: i32) -> Self {
            BorderWidth {left, right, top, bottom}
        }

        pub fn all(width: i32) -> Self {
            BorderWidth {left: width, right: width, top: width, bottom: width}
        }

        pub fn as_vec4(&self) -> Vec4 {
            Vec4::new(self.left as f32, self.right as f32, self.top as f32, self.bottom as f32)
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
    pub struct CatplushImageData<'a> {
        pub(crate) data: &'a[u8],
        pub(crate) width: i32,
        pub(crate) height: i32,
    }

    #[derive(Default)]
    pub enum ElementType<'a> {
        #[default]
        Unset,
        Rectangle,
        Text ( String, u8 ),
        Image ( CatplushImageData<'a> )
    }

    #[derive(Default)]
    pub struct UiElement<'a> {
        pub object_type: ElementType<'a>,
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

    impl UiElement<'_> {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn rectangle(mut self, color: ObjectColor, corner_radius: CornerRadius) -> Self {
            self.object_type = ElementType::Rectangle;
            self.color = color;
            self.corner_radius = corner_radius;
            self
        }

        pub fn border(mut self, color: ObjectColor, stroke_color: ObjectColor, corner_radius: CornerRadius, border_width: BorderWidth) -> Self {
            self.border_width = border_width;
            self.corner_radius = corner_radius;
            self.color = color;
            self.stroke_color = stroke_color;
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

    impl<'a> CatplushContext<'a> {
        //////////// Layout Building Functions //////////////
        pub fn open_element(&'a mut self, element: UiElement<'a>) {
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
                        match child_size.total_cmp(&smallest_size) {
                            Ordering::Less => { second_smallest_size = smallest_size; smallest_size = child_size; },
                            Ordering::Equal => { continue; },
                            Ordering::Greater => { second_smallest_size = f32::min(second_smallest_size, child_size); width_to_add = second_smallest_size - smallest_size; }
                        }
                    }

                    width_to_add = f32::min(width_to_add, size_to_distribute / (growable_elements.len() as f32));

                    for child_index in &growable_elements {
                        let child_size =
                            if left_to_right { &mut self.layout_elements[*child_index].element.final_size_x }
                            else { &mut self.layout_elements[*child_index].element.final_size_y };
                        let initial_size = *child_size;

                        *child_size += width_to_add;
                        size_to_distribute -= *child_size - initial_size
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

        // Solves all sizing and positioning and returns a set of render commands for passing to the
        // renderer
        pub fn end_layout<'a>(mut self) -> Vec<RenderCommand<'a>> {
            self.open_layout_elements.clear();

            self.size_all();
            self.position_all();

            let mut render_commands: Vec<RenderCommand> = vec![];
            for node in &self.layout_elements[1..self.layout_elements.len()] {
                let element = &node.element;
                let bounding_box = BoundingBox { x: element.final_pos_x, y: element.final_pos_y, width: element.final_size_x, height: element.final_size_y };
                let render_data = match &element.object_type {
                    ElementType::Unset => RenderData::NoType,
                    ElementType::Rectangle => {
                        RenderData::RectangleData(RectangleRenderData { color: element.color, stroke_color: element.stroke_color, corner_radius: element.corner_radius, border_width: element.border_width })
                    },
                    ElementType::Text(contents, font_size) => {
                        RenderData::TextData(TextRenderData { color: element.color, font_size: *font_size, letter_spacing: 0, line_height: 0 })
                    },
                    ElementType::Image(file) => {
                        RenderData::ImageData(ImageRenderData { data: file.data, width: file.width, height: file.height })
                    }
                };

                let id = element.id.unwrap_or_default();

                render_commands.push( RenderCommand { bounding_box, render_data, id } );
            }

            render_commands
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
        pub(crate) stroke_color: ObjectColor,
        pub(crate) corner_radius: CornerRadius,
        pub(crate) border_width: BorderWidth
    }


    pub(crate) struct TextRenderData {
        pub(crate) color: ObjectColor,

        pub(crate) font_size: u8,
        pub(crate) letter_spacing: u8,
        pub(crate) line_height: u8
    }

    pub(crate) struct ImageRenderData<'a> {
        pub(crate) data: &'a[u8],
        pub(crate) width: i32,
        pub(crate) height: i32,

        // Need to implement these myself.
        // pub(crate) tint: ObjectColor,
        // pub(crate) corner_radius: CornerRadius
    }

    pub(crate) enum RenderData<'a> {
        NoType,
        RectangleData(RectangleRenderData),
        TextData(TextRenderData),
        ImageData(ImageRenderData<'a>)
    }

    pub struct RenderCommand<'a> {
        pub(crate) bounding_box: BoundingBox,

        pub(crate) render_data: RenderData<'a>,

        pub id: &'static str,
    }
}

pub mod catplush_raylib {
    use crate::catplush_main::*;
    use frienderer::{TextLayout, RawImage, Renderer};
    use parley::{Alignment, FontContext, FontStack, LayoutContext, LineHeight, StyleProperty};
    use image::ImageFormat;
    use glow::{NativeTexture};
    use glam::{Vec2};

    #[allow(clippy::too_many_arguments)]
    pub fn create_text(layout_context: &mut LayoutContext<()>, mut font_context: FontContext, text: &str, font_stack: &str, line_height: f32, font_size: f32, alignment: Alignment, color: ObjectColor, scale: f32) -> TextLayout {
        let mut builder = layout_context.ranged_builder(&mut font_context, text, scale, true);
        {
            builder.push_default(FontStack::from(font_stack));
            builder.push_default(StyleProperty::LineHeight(LineHeight::FontSizeRelative(line_height)));
            builder.push_default(StyleProperty::FontSize(font_size));
        }
        TextLayout::new(builder.build(text), alignment, color.as_u32(), scale)
    }

    pub fn load_texture(renderer: Renderer, image_bytes: &[u8], format: ImageFormat) -> NativeTexture {
        let opengl_image = image::load_from_memory_with_format(image_bytes, format).unwrap();
        renderer.upload_texture(RawImage {
            width: opengl_image.width(),
            height: opengl_image.height(),
            pixels: opengl_image.as_bytes()
        })
    }

    pub fn plush_to_frend_rect(rectangle: UiElement) -> frienderer::RRect {
        frienderer::RRect {
            pos: Vec2::new(rectangle.final_pos_x, rectangle.final_pos_y),
            size: Vec2::new(rectangle.final_size_x, rectangle.final_size_y),
            border_radius: rectangle.corner_radius.as_vec4(),
            border_width: rectangle.border_width.as_vec4(),
            fill_color: rectangle.color.as_u32(),
            stroke_color: rectangle.stroke_color.as_u32()
        }
    }
}
