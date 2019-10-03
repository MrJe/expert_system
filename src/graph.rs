use std::io::{Error, ErrorKind};

pub type NodeIndex = usize;

#[derive(Clone, Debug)]
pub struct Node<T> {
	content: T,
	lhs: Option<NodeIndex>,
	rhs: Option<NodeIndex>,
	parent: Option<NodeIndex>,
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
			return Err(Error::new(ErrorKind::Other, "Graph: lhs already filled"))
		}
		self.lhs = Some(index);
		Ok(index)
	}

	pub fn set_rhs(&mut self, index: NodeIndex) -> Result<NodeIndex, Error> {
		if self.rhs.is_some() {
			return Err(Error::new(ErrorKind::Other, "Graph: rhs already filled"))
		}
		self.rhs = Some(index);
		Ok(index)
	}

	pub fn set_parent(&mut self, index: NodeIndex) -> Result<NodeIndex, Error> {
		if self.parent.is_some() {
			return Err(Error::new(ErrorKind::Other, "Graph: parent already filled"))
		}
		self.parent = Some(index);
		Ok(index)
	}
}

#[derive(Clone, Debug)]
pub struct Graph<T>(Vec<Node<T>>);

impl<T> Graph<T> {
	pub fn new() -> Graph<T> {
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
					return Err(err)
				}
			}
		}
		self.0.pop();
		return Err(Error::new(ErrorKind::InvalidData, "Graph: node index not exist"))
	}

	pub fn insert_rhs(&mut self, current: NodeIndex, content: T) -> Result<NodeIndex, Error> {
		let new_node_index = self.push(content, Some(current));
		if let Some(node) = self.get_mut(current) {
			match node.set_rhs(new_node_index) {
				Ok(child_index) => return Ok(child_index),
				Err(err) => {
					self.0.pop();
					return Err(err)
				}
			}
		}
		self.0.pop();
		return Err(Error::new(ErrorKind::InvalidData, "Graph: node index not exist"))
	}

	pub fn insert_operand(&mut self, current: NodeIndex, content: T) -> Result<NodeIndex, Error> {
		self.insert_lhs(current, content)
	}

	pub fn get(&self, key: NodeIndex) -> Option<&Node<T>> {
		self.0.get(key)
	}

	pub fn get_mut(&mut self, key: NodeIndex) -> Option<&mut Node<T>> {
		self.0.get_mut(key)
	}
}
