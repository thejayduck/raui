pub extern crate typetag;

pub mod application;
pub mod messenger;
#[macro_use]
pub mod props;
pub mod renderer;
pub mod state;
#[macro_use]
pub mod widget;
pub mod animator;
pub mod interactive;
pub mod layout;
pub mod signals;

#[cfg(feature = "scalar64")]
pub type Scalar = f64;
#[cfg(not(feature = "scalar64"))]
pub type Scalar = f32;
#[cfg(feature = "integer64")]
pub type Integer = i32;
#[cfg(not(feature = "integer64"))]
pub type Integer = i64;

pub mod prelude {
    pub use crate::{
        animator::*,
        application::*,
        interactive::default_interactions_engine::*,
        interactive::*,
        layout::default_layout_engine::*,
        layout::*,
        messenger::*,
        props::*,
        renderer::*,
        signals::*,
        state::*,
        typetag,
        widget::*,
        widget::{
            component::*,
            component::{
                containers::{
                    content_box::*, flex_box::*, grid_box::*, horizontal_box::*, size_box::*,
                    switch_box::*, variant_box::*, vertical_box::*, wrap_box::*,
                },
                image_box::*,
                interactive::{button::*, input_field::*},
                space_box::*,
                text_box::*,
            },
            context::*,
            node::*,
            unit::*,
            unit::{content::*, flex::*, grid::*, image::*, size::*, text::*},
            utils::*,
        },
        Integer, Scalar,
    };
}
