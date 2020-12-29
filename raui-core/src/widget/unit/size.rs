use crate::{
    props::{Props, PropsDef},
    widget::{
        node::{WidgetNode, WidgetNodeDef},
        unit::{WidgetUnit, WidgetUnitData},
        utils::Rect,
        WidgetId,
    },
    Scalar,
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SizeBoxSizeValue {
    Content,
    Fill,
    Exact(Scalar),
}

impl Default for SizeBoxSizeValue {
    fn default() -> Self {
        Self::Content
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SizeBox {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub slot: Box<WidgetUnit>,
    #[serde(default)]
    pub width: SizeBoxSizeValue,
    #[serde(default)]
    pub height: SizeBoxSizeValue,
    #[serde(default)]
    pub margin: Rect,
}

impl WidgetUnitData for SizeBox {
    fn id(&self) -> &WidgetId {
        &self.id
    }

    fn get_children<'a>(&'a self) -> Vec<&'a WidgetUnit> {
        vec![&self.slot]
    }
}

impl TryFrom<SizeBoxNode> for SizeBox {
    type Error = ();

    fn try_from(node: SizeBoxNode) -> Result<Self, Self::Error> {
        let SizeBoxNode {
            id,
            slot,
            width,
            height,
            margin,
            ..
        } = node;
        Ok(Self {
            id,
            slot: Box::new(WidgetUnit::try_from(*slot)?),
            width,
            height,
            margin,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub struct SizeBoxNode {
    pub id: WidgetId,
    pub props: Props,
    pub slot: Box<WidgetNode>,
    pub width: SizeBoxSizeValue,
    pub height: SizeBoxSizeValue,
    pub margin: Rect,
}

impl SizeBoxNode {
    pub fn remap_props<F>(&mut self, mut f: F)
    where
        F: FnMut(Props) -> Props,
    {
        let props = std::mem::take(&mut self.props);
        self.props = (f)(props);
    }
}

impl Into<WidgetNode> for SizeBoxNode {
    fn into(self) -> WidgetNode {
        WidgetNode::Unit(self.into())
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SizeBoxNodeDef {
    #[serde(default)]
    pub id: WidgetId,
    #[serde(default)]
    pub props: PropsDef,
    #[serde(default)]
    pub slot: Box<WidgetNodeDef>,
    #[serde(default)]
    pub width: SizeBoxSizeValue,
    #[serde(default)]
    pub height: SizeBoxSizeValue,
    #[serde(default)]
    pub margin: Rect,
}
