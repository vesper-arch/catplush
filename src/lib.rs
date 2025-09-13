pub mod clay_main {
    use std::cmp::max;

    // ClayContext is the goddamn backbone of this whole library. It lets functions look at the
    // current open elements so the UI Heirarchy can be constructed. This will be extended to store the info of the layout itself but that's a lotta work and i dont give a shit right now. There should only be a single one of these in existence at any given time. If there are
    // multiple uhh shit's gonna break.
    pub struct ClayContext {
        layout_elements: Vec<Node>,

        open_layout_elements: Vec<usize>
    }

    impl ClayContext {
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

    impl Default for ClayContext {
        fn default() -> Self {
            Self {
                layout_elements: vec![],
                open_layout_elements: vec![]
            }
        }
    }

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

       fn get_parent_element<'a>(&self, context: &'a mut ClayContext) -> &'a mut Node {
           context.layout_elements.get_mut(self.parent.unwrap()).unwrap()
       }
    }

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

    impl SizingMode {
        fn get_as_float(&self) -> f32 {
            match self {
                // Not completely sure why the deref is necessary but no more compiler error
                SizingMode::Fixed(size) => *size as f32,
                SizingMode::Fit => panic!("Given that fit should have been taken care of already, this is weird error."),
                SizingMode::Grow => 0.0,
            }
        }

        fn get_as_int(&self) -> i32 {
            match self {
                SizingMode::Fixed(size) => *size,
                SizingMode::Fit => panic!("Given that fit should have been taken care of already, this is weird error."),
                SizingMode::Grow => 0,
            }
        }
    }

    pub struct SizeConstraint {
        min: i32,
        max: i32
    }

    impl Default for SizeConstraint {
        fn default() -> Self {
            return Self {min: 0, max: 999999999}
        }
    }

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

    pub struct ChildGap(pub i32);

    pub enum ChildXAlignment { AlignXLeft, AlignXCenter, AlignXRight }
    pub enum ChildYAlignment { AlignYTop, AlignYCenter, AlignYBottom }

    pub struct ChildAlignment {
        pub x: ChildXAlignment,
        pub y: ChildYAlignment,
    }

    impl ChildAlignment {
        pub fn default() -> Self {
            ChildAlignment {x: ChildXAlignment::AlignXLeft, y: ChildYAlignment::AlignYTop}
        }

        pub fn new(x_align: ChildXAlignment, y_align: ChildYAlignment) -> Self {
            ChildAlignment {x: x_align, y: y_align}
        }
        
    }

    pub struct LayoutConfig {
        pub sizing: Sizing,
        pub size_constraints: (SizeConstraint, SizeConstraint),
        pub padding: Padding,
        pub child_gap: ChildGap,
        pub layout_direction: ChildLayoutDirection,
        pub child_alignment: ChildAlignment
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            LayoutConfig {
                sizing: Sizing::both(SizingMode::Fit),
                size_constraints: (SizeConstraint::default(), SizeConstraint::default()),
                padding: Padding::all(0),
                child_gap: ChildGap(0),
                layout_direction: ChildLayoutDirection::LeftToRight,
                child_alignment: ChildAlignment::default()
            }
        }
    }

    pub enum ElementType {
        Unset,
        Rectangle,
        Text ( String, u8 ),
        Image ( String )
    }

    // This is what the user will be working with instead of creating the ClayObject directly.
    // Basic idea is to make this as convenient as possible and then have a function to translate
    // this to the actual ClayObject struct.
    pub struct ClayElement {
        pub object_type: ElementType,
        pub id: Option<String>,
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
            return Self {
                object_type: ElementType::Unset,
                id: None,
                layout: LayoutConfig::default(),

                color: ObjectColor( 0, 0, 0, 0 ),
                corner_radius: CornerRadius::all(0.0),
                
                final_size_x: 0.0,
                final_size_y: 0.0,
                final_pos_x: 0.0,
                final_pos_y: 0.0
            }
        }
        

        pub fn rectangle(mut self, color: ObjectColor, corner_radius: CornerRadius) -> Self {
            self.object_type = ElementType::Rectangle;
            self.color = color;
            self.corner_radius = corner_radius;
            return self
        }

        pub fn text(mut self, text: String, color: ObjectColor, font_size: u8) -> Self {
            self.object_type = ElementType::Text( text, font_size );
            self.color = color;
            return self
        }

        pub fn image(mut self, file: String, corner_radius: CornerRadius) -> Self {
            self.object_type = ElementType::Image( file );
            self.corner_radius = corner_radius;
            return self
        }

        pub fn child_gap(mut self, amount: i32) -> Self {
            self.layout.child_gap = ChildGap(amount);
            return self
        }

        pub fn padding(mut self, padding: Padding) -> Self {
            self.layout.padding = padding;
            return self
        }

        pub fn sizing(mut self, x: SizingMode, y: SizingMode) -> Self {
            self.layout.sizing = Sizing{ width: x, height: y };
            return self
        }

        pub fn constrain(mut self, width: SizeConstraint, height: SizeConstraint) -> Self {
            self.layout.size_constraints = (width, height);
            return self
        }

        pub fn layout_direction(mut self, direction: ChildLayoutDirection) -> Self {
            self.layout.layout_direction = direction;
            return self
        }

        pub fn alignment(mut self, x_align: ChildXAlignment, y_align: ChildYAlignment) -> Self {
            self.layout.child_alignment = ChildAlignment{ x: x_align, y: y_align };
            return self
        }

        pub fn id(mut self, id: String) -> Self {
            self.id = Some(id);
            return self
        }
    }

    pub fn open_element(context: &mut ClayContext, element: ClayElement) {
        let new_element_index = context.layout_elements.len();
        let mut parent_element: usize = 0;
        
        if context.open_layout_elements.len() > 0 {
            context.layout_elements.last_mut().unwrap().child_elements.push(new_element_index);
            parent_element = *context.open_layout_elements.last().unwrap();
        }

        context.open_layout_elements.push(new_element_index);
        context.layout_elements.push(Node::new(element, parent_element));
    }

    pub fn close_element(context: &mut ClayContext) {
        let layout_slice = &mut context.layout_elements[..];
        let last_opened_element = *context.open_layout_elements.last().unwrap();
        // let parent_element = layout_slice[last_opened_element].parent.unwrap();
        // index 0: parent node | index 1: last opened node
        let current_elements = layout_slice.get_disjoint_mut([last_opened_element]).unwrap();

        // Fixed Sizing
        match current_elements[0].element.layout.sizing.width {
            SizingMode::Fixed(size) => {current_elements[0].element.final_size_x = size as f32},
            SizingMode::Fit => {},
            SizingMode::Grow => {},
        }
        match current_elements[0].element.layout.sizing.height {
            SizingMode::Fixed(size) => {current_elements[0].element.final_size_y = size as f32},
            SizingMode::Fit => {},
            SizingMode::Grow => {},
        }
        // Fit Sizing
        // if current_elements[0].element.layout.sizing.width == SizingMode::Fit {
        //     if current_elements[0].element.layout.layout_direction == ChildLayoutDirection::LeftToRight {
        //         current_elements[0].element.final_size_x += current_elements[1].element.layout.sizing.width.get_as_float();
        //
        //         current_elements[0].element.final_size_y = max(current_elements[0].element.final_size_y as i32, current_elements[1].element.layout.sizing.height.get_as_int()) as f32;
        //     } else {
        //         current_elements[0].element.final_size_y += current_elements[1].element.layout.sizing.height.get_as_float();
        //
        //         current_elements[0].element.final_size_x = max(current_elements[0].element.final_size_x as i32, current_elements[1].element.layout.sizing.width.get_as_int()) as f32;
        //     }
        //
        //     current_elements[0].element.final_size_x += (current_elements[0].element.layout.padding.left + current_elements[0].element.layout.padding.right) as f32;
        //     current_elements[0].element.final_size_y += (current_elements[0].element.layout.padding.top + current_elements[0].element.layout.padding.bottom) as f32;
        // }

        context.open_layout_elements.pop();
    }
}

pub mod clay_raylib {
    use raylib::prelude::*;
    use crate::clay_main;

    pub fn raylib_init() -> (RaylibHandle, RaylibThread){
        let (rl, thread) = raylib::init()
            .size(640, 480)
            .title("hell")
            .resizable()
            .build();

        return (rl, thread)
    }

    pub fn clay_to_raylib_rect(object: &clay_main::ClayElement) -> Rectangle {
        Rectangle {
            x: object.final_pos_x,
            y: object.final_pos_y,
            width: object.final_size_x,
            height: object.final_size_x
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 4, 6);
    }
}
