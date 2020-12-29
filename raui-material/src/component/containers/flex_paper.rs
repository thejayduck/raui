use crate::component::containers::paper::paper;
use raui_core::prelude::*;

widget_component! {
    pub flex_paper(key, props, listed_slots) {
        widget! {
            (#{key} paper: {props.clone()} [
                (#{"flex"} flex_box: {props.clone()} |[ listed_slots ]|)
            ])
        }
    }
}
