use crate::{
    props::Props,
    widget::{
        component::{WidgetComponent, WidgetComponentPrefab},
        unit::{WidgetUnitNode, WidgetUnitNodePrefab},
    },
    Prefab,
};
use serde::{Deserialize, Serialize};
use std::mem::MaybeUninit;

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
pub enum WidgetNode {
    None,
    Component(WidgetComponent),
    Unit(WidgetUnitNode),
    Tuple(Vec<WidgetNode>),
}

impl WidgetNode {
    pub fn is_none(&self) -> bool {
        match self {
            Self::None => true,
            Self::Unit(unit) => unit.is_none(),
            Self::Tuple(v) => v.is_empty(),
            _ => false,
        }
    }

    pub fn is_some(&self) -> bool {
        match self {
            Self::None => false,
            Self::Unit(unit) => unit.is_some(),
            Self::Tuple(v) => !v.is_empty(),
            _ => true,
        }
    }

    pub fn as_component(&self) -> Option<&WidgetComponent> {
        match self {
            Self::Component(c) => Some(c),
            _ => None,
        }
    }

    pub fn as_unit(&self) -> Option<&WidgetUnitNode> {
        match self {
            Self::Unit(u) => Some(u),
            _ => None,
        }
    }

    pub fn as_tuple(&self) -> Option<&[WidgetNode]> {
        match self {
            Self::Tuple(v) => Some(v),
            _ => None,
        }
    }

    pub fn props(&self) -> Option<&Props> {
        match self {
            Self::Component(c) => Some(&c.props),
            Self::Unit(u) => u.props(),
            _ => None,
        }
    }

    pub fn props_mut(&mut self) -> Option<&mut Props> {
        match self {
            Self::Component(c) => Some(&mut c.props),
            Self::Unit(u) => u.props_mut(),
            _ => None,
        }
    }

    pub fn remap_props<F>(&mut self, f: F)
    where
        F: FnMut(Props) -> Props,
    {
        match self {
            Self::Component(c) => c.remap_props(f),
            Self::Unit(u) => u.remap_props(f),
            _ => {}
        }
    }

    pub fn shared_props(&self) -> Option<&Props> {
        match self {
            Self::Component(c) => c.shared_props.as_ref(),
            _ => None,
        }
    }

    pub fn shared_props_mut(&mut self) -> Option<&mut Props> {
        match self {
            Self::Component(c) => {
                if c.shared_props.is_none() {
                    c.shared_props = Some(Default::default());
                }
                c.shared_props.as_mut()
            }
            _ => None,
        }
    }

    pub fn remap_shared_props<F>(&mut self, f: F)
    where
        F: FnMut(Props) -> Props,
    {
        if let Self::Component(c) = self {
            c.remap_shared_props(f);
        }
    }

    pub fn pack_tuple<const N: usize>(data: [WidgetNode; N]) -> Self {
        Self::Tuple(data.into())
    }

    pub fn unpack_tuple<const N: usize>(self) -> [WidgetNode; N] {
        let mut data: [MaybeUninit<WidgetNode>; N] = unsafe { MaybeUninit::uninit().assume_init() };
        for item in data.iter_mut().take(N) {
            *item = MaybeUninit::new(WidgetNode::None);
        }
        if let WidgetNode::Tuple(mut v) = self {
            for i in (0..(v.len().min(N))).rev() {
                data[i] = MaybeUninit::new(v.swap_remove(i));
            }
        }
        // TODO: workaround for MaybeUninit to array transmute not working with generics.
        let ptr = &data as *const _ as *const [WidgetNode; N];
        let res = unsafe { ptr.read() };
        std::mem::forget(data);
        res
    }
}

impl Default for WidgetNode {
    fn default() -> Self {
        Self::None
    }
}

impl From<()> for WidgetNode {
    fn from(_: ()) -> Self {
        Self::None
    }
}

impl From<()> for Box<WidgetNode> {
    fn from(_: ()) -> Self {
        Box::new(WidgetNode::None)
    }
}

impl From<WidgetComponent> for WidgetNode {
    fn from(component: WidgetComponent) -> Self {
        Self::Component(component)
    }
}

impl From<WidgetUnitNode> for WidgetNode {
    fn from(unit: WidgetUnitNode) -> Self {
        Self::Unit(unit)
    }
}

impl From<WidgetUnitNode> for Box<WidgetNode> {
    fn from(unit: WidgetUnitNode) -> Self {
        Box::new(WidgetNode::Unit(unit))
    }
}

impl<const N: usize> From<[WidgetNode; N]> for WidgetNode {
    fn from(data: [WidgetNode; N]) -> Self {
        Self::pack_tuple(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WidgetNodePrefab {
    None,
    Component(WidgetComponentPrefab),
    Unit(WidgetUnitNodePrefab),
    Tuple(Vec<WidgetNodePrefab>),
}

impl Default for WidgetNodePrefab {
    fn default() -> Self {
        Self::None
    }
}

impl Prefab for WidgetNodePrefab {}
