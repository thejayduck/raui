pub mod content;
pub mod flex;
pub mod grid;
pub mod image;
pub mod size;
pub mod text;

use crate::{
    props::Props,
    widget::{
        node::WidgetNode,
        unit::{
            content::{ContentBox, ContentBoxNode, ContentBoxNodeDef},
            flex::{FlexBox, FlexBoxNode, FlexBoxNodeDef},
            grid::{GridBox, GridBoxNode, GridBoxNodeDef},
            image::{ImageBox, ImageBoxNode, ImageBoxNodeDef},
            size::{SizeBox, SizeBoxNode, SizeBoxNodeDef},
            text::{TextBox, TextBoxNode, TextBoxNodeDef},
        },
        WidgetId,
    },
};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct WidgetUnitInspectionNode {
    pub id: WidgetId,
    pub children: Vec<WidgetUnitInspectionNode>,
}

pub trait WidgetUnitData {
    fn id(&self) -> &WidgetId;

    fn get_children<'a>(&'a self) -> Vec<&'a WidgetUnit> {
        vec![]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetUnit {
    None,
    ContentBox(ContentBox),
    FlexBox(FlexBox),
    GridBox(GridBox),
    SizeBox(SizeBox),
    ImageBox(ImageBox),
    TextBox(TextBox),
}

impl Default for WidgetUnit {
    fn default() -> Self {
        Self::None
    }
}

impl WidgetUnit {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn as_data(&self) -> Option<&dyn WidgetUnitData> {
        match self {
            Self::None => None,
            Self::ContentBox(v) => Some(v as &dyn WidgetUnitData),
            Self::FlexBox(v) => Some(v as &dyn WidgetUnitData),
            Self::GridBox(v) => Some(v as &dyn WidgetUnitData),
            Self::SizeBox(v) => Some(v as &dyn WidgetUnitData),
            Self::ImageBox(v) => Some(v as &dyn WidgetUnitData),
            Self::TextBox(v) => Some(v as &dyn WidgetUnitData),
        }
    }

    pub fn inspect(&self) -> Option<WidgetUnitInspectionNode> {
        if let Some(data) = self.as_data() {
            Some(WidgetUnitInspectionNode {
                id: data.id().to_owned(),
                children: data
                    .get_children()
                    .into_iter()
                    .filter_map(|child| child.inspect())
                    .collect::<Vec<_>>(),
            })
        } else {
            None
        }
    }
}

impl TryFrom<WidgetUnitNode> for WidgetUnit {
    type Error = ();

    fn try_from(node: WidgetUnitNode) -> Result<Self, Self::Error> {
        match node {
            WidgetUnitNode::None => Ok(Self::None),
            WidgetUnitNode::ContentBox(n) => Ok(WidgetUnit::ContentBox(ContentBox::try_from(n)?)),
            WidgetUnitNode::FlexBox(n) => Ok(WidgetUnit::FlexBox(FlexBox::try_from(n)?)),
            WidgetUnitNode::GridBox(n) => Ok(WidgetUnit::GridBox(GridBox::try_from(n)?)),
            WidgetUnitNode::SizeBox(n) => Ok(WidgetUnit::SizeBox(SizeBox::try_from(n)?)),
            WidgetUnitNode::ImageBox(n) => Ok(WidgetUnit::ImageBox(ImageBox::try_from(n)?)),
            WidgetUnitNode::TextBox(n) => Ok(WidgetUnit::TextBox(TextBox::try_from(n)?)),
        }
    }
}

impl TryFrom<WidgetNode> for WidgetUnit {
    type Error = ();

    fn try_from(node: WidgetNode) -> Result<Self, Self::Error> {
        match node {
            WidgetNode::None => Ok(Self::None),
            WidgetNode::Component(_) => Err(()),
            WidgetNode::Unit(u) => Self::try_from(u),
        }
    }
}

#[derive(Debug, Clone)]
pub enum WidgetUnitNode {
    None,
    ContentBox(ContentBoxNode),
    FlexBox(FlexBoxNode),
    GridBox(GridBoxNode),
    SizeBox(SizeBoxNode),
    ImageBox(ImageBoxNode),
    TextBox(TextBoxNode),
}

impl Default for WidgetUnitNode {
    fn default() -> Self {
        Self::None
    }
}

impl WidgetUnitNode {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_some(&self) -> bool {
        !matches!(self, Self::None)
    }

    pub fn props(&self) -> Option<&Props> {
        match self {
            Self::ContentBox(v) => Some(&v.props),
            Self::FlexBox(v) => Some(&v.props),
            Self::GridBox(v) => Some(&v.props),
            Self::SizeBox(v) => Some(&v.props),
            Self::ImageBox(v) => Some(&v.props),
            Self::TextBox(v) => Some(&v.props),
            _ => None,
        }
    }

    pub fn props_mut(&mut self) -> Option<&mut Props> {
        match self {
            Self::ContentBox(v) => Some(&mut v.props),
            Self::FlexBox(v) => Some(&mut v.props),
            Self::GridBox(v) => Some(&mut v.props),
            Self::SizeBox(v) => Some(&mut v.props),
            Self::ImageBox(v) => Some(&mut v.props),
            Self::TextBox(v) => Some(&mut v.props),
            _ => None,
        }
    }

    pub fn remap_props<F>(&mut self, f: F)
    where
        F: FnMut(Props) -> Props,
    {
        match self {
            Self::None => {}
            Self::ContentBox(v) => v.remap_props(f),
            Self::FlexBox(v) => v.remap_props(f),
            Self::GridBox(v) => v.remap_props(f),
            Self::SizeBox(v) => v.remap_props(f),
            Self::ImageBox(v) => v.remap_props(f),
            Self::TextBox(v) => v.remap_props(f),
        }
    }
}

impl TryFrom<WidgetNode> for WidgetUnitNode {
    type Error = ();

    fn try_from(node: WidgetNode) -> Result<Self, Self::Error> {
        if let WidgetNode::Unit(v) = node {
            Ok(v)
        } else {
            Err(())
        }
    }
}

impl From<()> for WidgetUnitNode {
    fn from(_: ()) -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetUnitNodeDef {
    None,
    ContentBox(ContentBoxNodeDef),
    FlexBox(FlexBoxNodeDef),
    GridBox(GridBoxNodeDef),
    SizeBox(SizeBoxNodeDef),
    ImageBox(ImageBoxNodeDef),
    TextBox(TextBoxNodeDef),
}

impl Default for WidgetUnitNodeDef {
    fn default() -> Self {
        Self::None
    }
}

macro_rules! implement_from_unit {
    { $( $type_name:ident => $variant_name:ident ),+ $(,)? } => {
        $(
            impl From<$type_name> for WidgetUnitNode {
                fn from(unit: $type_name) -> Self {
                    Self::$variant_name(unit)
                }
            }
        )+
    };
}

implement_from_unit! {
    ContentBoxNode => ContentBox,
    FlexBoxNode => FlexBox,
    GridBoxNode => GridBox,
    SizeBoxNode => SizeBox,
    ImageBoxNode => ImageBox,
    TextBoxNode => TextBox,
}
