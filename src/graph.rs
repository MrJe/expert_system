use core::slice::{Iter, IterMut};
use std::io::{Error, ErrorKind};

pub type NodeIndex = usize;

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub content: T,
    pub lhs: Option<NodeIndex>,
    pub rhs: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}

impl<T> Node<T> {
    pub fn new(content: T, parent: Option<NodeIndex>) -> Node<T> {
        Node {
            content,
            lhs: None,
            rhs: None,
            parent,
        }
    }

    pub fn set_lhs(&mut self, index: NodeIndex) -> Result<NodeIndex, Error> {
        if self.lhs.is_some() {
            return Err(Error::new(ErrorKind::Other, "Graph: lhs already filled"));
        }
        self.lhs = Some(index);
        Ok(index)
    }

    pub fn set_rhs(&mut self, index: NodeIndex) -> Result<NodeIndex, Error> {
        if self.rhs.is_some() {
            return Err(Error::new(ErrorKind::Other, "Graph: rhs already filled"));
        }
        self.rhs = Some(index);
        Ok(index)
    }

    pub fn set_parent(&mut self, index: NodeIndex) -> Result<NodeIndex, Error> {
        if self.parent.is_some() {
            return Err(Error::new(ErrorKind::Other, "Graph: parent already filled"));
        }
        self.parent = Some(index);
        Ok(index)
    }

    pub fn have_child(&self) -> bool {
        self.lhs.is_some() | self.rhs.is_some()
    }
}

#[derive(Clone, Debug, Default)]
pub struct Graph<T>(Vec<Node<T>>);

impl<T> Graph<T> {
    pub fn new() -> Self {
        Graph(Vec::new())
    }

    fn push(&mut self, content: T, parent: Option<NodeIndex>) -> NodeIndex {
        let node = Node::new(content, parent);
        self.0.push(node);
        self.0.len() - 1
    }

    pub fn add_query(&mut self, content: T) -> NodeIndex {
        self.push(content, None)
    }

    pub fn insert_lhs(&mut self, current: NodeIndex, content: T) -> Result<NodeIndex, Error> {
        let new_node_index = self.push(content, Some(current));
        if let Some(node) = self.get_mut(current) {
            match node.set_lhs(new_node_index) {
                Ok(child_index) => return Ok(child_index),
                Err(err) => {
                    self.0.pop();
                    return Err(err);
                }
            }
        }
        self.0.pop();
        Err(Error::new(
            ErrorKind::NotFound,
            "Graph: node index not exist",
        ))
    }

    pub fn insert_rhs(&mut self, current: NodeIndex, content: T) -> Result<NodeIndex, Error> {
        let new_node_index = self.push(content, Some(current));
        if let Some(node) = self.get_mut(current) {
            match node.set_rhs(new_node_index) {
                Ok(child_index) => return Ok(child_index),
                Err(err) => {
                    self.0.pop();
                    return Err(err);
                }
            }
        }
        self.0.pop();
        Err(Error::new(
            ErrorKind::NotFound,
            "Graph: node index not exist",
        ))
    }

    pub fn append(&mut self, to_append: &mut Graph<T>, parent_node_index: NodeIndex) {
        let offset = self.len();
        println!("Append: offset = {}, {}", offset, parent_node_index);
        for node in to_append.iter_mut() {
            if let Some(index) = node.lhs.as_mut() {
                print!("lhs: new={}, old={}\t", *index + offset, *index);
                *index += offset;
            }
            if let Some(index) = node.rhs.as_mut() {
                print!("rhs: new={}, old={}\t", *index + offset, *index);
                *index += offset;
            }
            if let Some(index) = node.parent.as_mut() {
                print!("parent: new={}, old={}\t", *index + offset, *index);
                *index += offset;
            } else if node.parent.is_none() && parent_node_index > 0 {
                if let Some(parent) = self.get(parent_node_index) {
                    if let Some(grandpa_index) = parent.parent {
                        node.parent = Some(grandpa_index);
                    }
                }
            }
            // println!();
            // println!("{}, {}, {}", node.lhs.unwrap(), node.rhs.unwrap(), node.parent.unwrap());
        }
        self.0.append(&mut to_append.0);
    }

    pub fn get(&self, key: NodeIndex) -> Option<&Node<T>> {
        self.0.get(key)
    }

    pub fn get_mut(&mut self, key: NodeIndex) -> Option<&mut Node<T>> {
        self.0.get_mut(key)
    }

    pub fn iter(&self) -> Iter<Node<T>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Node<T>> {
        self.0.iter_mut()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn truncate(&mut self, new_size: usize) {
        self.0.truncate(new_size);
    }
}
