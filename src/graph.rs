use std::collections::HashMap;
use std::io::{Error, ErrorKind};

pub type Index = u32;

pub struct Node<'a, T> {
	content: &'a T,
	lhs: Option<Index>,
	rhs: Option<Index>,
}

impl<'a, T> Node<'a, T> {
	pub fn new(content: &'a T) -> Node<'a, T> {
		Node {
			content,
			lhs: None,
			rhs: None,
		}
	}

	pub fn push(&mut self, index: Index) -> Result<(), Error> {
		if self.lhs.is_some() {
			if self.rhs.is_some() {
				return Err(Error::new(ErrorKind::Other, "Graph: node already filled"))
			}
			self.rhs = Some(index);
			return Ok(())
		}
		self.lhs = Some(index);
		Ok(())
	}

	pub fn get(&'a self) -> &'a Node<'a, T> {
		&self
	}

	pub fn get_content(&'a self) -> &'a T {
		&self.content
	}
}


pub struct Graph<'a, T>(HashMap<Index, Node<'a, T>>);

impl<'a, T> Graph<'a, T> {
	pub fn new(size: usize) -> Graph<'a, T> {
		Graph(HashMap::with_capacity(size))
	}

	pub fn insert(&mut self, key: Index, content: &'a T) -> Result<(), Error> {
		let node = Node::new(content);
		self.0.insert(key, node);
		Ok(())
	}

	pub fn get(&self, key: Index) -> Option<&Node<'a, T>> {
		for (i, noderef) in self.0.iter() {
			if *i == key {
				return Some(&noderef)
			}
		}
		None
	}
}
