use crate::{
    widget,
    widget::{
        component::containers::flex_box::{flex_box, FlexBoxProps},
        unit::flex::FlexBoxDirection,
    },
    widget_component, Scalar,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VerticalBoxProps {
    #[serde(default)]
    pub separation: Scalar,
    #[serde(default)]
    pub reversed: bool,
}
implement_props_data!(VerticalBoxProps, "VerticalBoxProps");

widget_component! {
    pub vertical_box(key, props, listed_slots) {
        let VerticalBoxProps { separation, reversed } = props.read_cloned_or_default();
        let props = props.clone().with(FlexBoxProps {
            direction: if reversed {
                FlexBoxDirection::VerticalBottomToTop
            } else {
                FlexBoxDirection::VerticalTopToBottom
            },
            separation,
            wrap: false,
        });

        widget! {
            (#{key} flex_box: {props} |[ listed_slots ]|)
        }
    }
}
