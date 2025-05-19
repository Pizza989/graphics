use crate::geometry::{Point, Rect, Size};
use indexmap::IndexMap;
use std::{
    cell::RefCell,
    clone::Clone,
    collections::{HashMap, HashSet, VecDeque},
    ops::{Deref, DerefMut},
    rc::Rc,
};
use thiserror::Error;
use uuid::Uuid;

use super::{
    layout::Layout,
    ui_node::{LayoutRefMut, NodeId, UiElement, UiNode, UiNodeMut},
    widget::Widget,
};

pub struct UiTree<T: Clone> {
    root: LayoutRefMut<T>,
    nodes: HashMap<NodeId, UiNodeMut<T>>,
}

#[derive(Debug, Error)]
pub enum GetNodeError {
    #[error("Node with id `{0}` not found")]
    NotFound(NodeId),
}

#[derive(Debug, Error)]
pub enum GetParentNodeError {
    #[error("Get node failed: {0}")]
    GetNode(#[from] GetNodeError),
    #[error("Node with id `{0}` is the root node and therefore has no parent")]
    IsRootNode(NodeId),
}

#[derive(Debug, Error)]
pub enum InsertError {
    #[error("Get node failed: {0}")]
    GetNode(#[from] GetNodeError),
    #[error(
        "Tried to insert under node with id {0}, however it is a widget and therefore has no children"
    )]
    NodeIsNotLayout(NodeId),
}

// TODO: make it so modifications of the tree can only be done through
// certain methods so the redundancies stay in sync.
impl<T: Clone> UiTree<T> {
    pub fn new(root: LayoutRefMut<T>) -> Self {
        let mut nodes = HashMap::new();
        // TODO: inset all the recursive children of root aswell
        nodes.insert(Uuid::new_v4(), root.clone().into());
        todo!();
        Self { root, nodes }
    }

    fn get_node_mut(&self, id: NodeId) -> Result<UiNodeMut<T>, GetNodeError> {
        match self.nodes.get(&id).cloned() {
            Some(node) => Ok(node),
            None => Err(GetNodeError::NotFound(id)),
        }
    }

    pub fn get_node(&self, id: NodeId) -> Result<UiNode<'_, T>, GetNodeError> {
        match self.nodes.get(&id) {
            Some(node_mut) => Ok(node_mut.into()),
            None => Err(GetNodeError::NotFound(id)),
        }
    }

    fn get_parent_node(&self, id: NodeId) -> Result<LayoutRefMut<T>, GetParentNodeError> {
        todo!()
    }

    pub fn insert_by_id(&self, id: NodeId, node: UiElement<T>) -> Result<(), InsertError> {
        match self.get_node_mut(id) {
            Ok(ui_node_mut) => match ui_node_mut {
                UiNodeMut::Widget { widget_ref } => Err(InsertError::NodeIsNotLayout(id)),
                UiNodeMut::Layout { layout_ref } => {
                    layout_ref.borrow_mut().add_child(node.into());
                    Ok(())
                }
            },
            Err(err) => Err(InsertError::GetNode(err)),
        }
    }

    fn replace_by_id(&self, id: NodeId, node: UiNodeMut<T>) -> Result<(), GetParentNodeError> {
        match self.get_parent_node(id) {
            Ok(layout) => {
                // unwrapping is safe because i just got layout as the parent
                // of the node with the id `id`
                layout.borrow_mut().replace_child(id, node).unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }
}
