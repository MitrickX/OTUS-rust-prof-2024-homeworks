use std::fmt::Display;

pub trait Visitor<T> {
    fn visit_a_node(&mut self, node: &NodeA<T>);
    fn visit_b_node(&mut self, node: &NodeB<T>);
}

pub trait Accepter<T, V>
where
    V: Visitor<T>,
{
    fn accept(&self, v: &mut V);
}

pub struct NodeA<T> {
    pub value: T,
    b_nodes: Vec<NodeB<T>>,
}

pub struct NodeB<T> {
    pub value: T,
    a_nodes: Vec<NodeA<T>>,
}

impl<T> NodeA<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            b_nodes: vec![],
        }
    }
    pub fn with_nodes(value: T, b_nodes: Vec<NodeB<T>>) -> Self {
        Self { value, b_nodes }
    }
}

impl<T> NodeB<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            a_nodes: vec![],
        }
    }
    pub fn with_nodes(value: T, a_nodes: Vec<NodeA<T>>) -> Self {
        Self { value, a_nodes }
    }
}

impl<T, V> Accepter<T, V> for NodeA<T>
where
    V: Visitor<T>,
{
    fn accept(&self, v: &mut V) {
        v.visit_a_node(self);
        self.b_nodes.iter().for_each(|n| n.accept(v));
    }
}

impl<T, V> Accepter<T, V> for NodeB<T>
where
    V: Visitor<T>,
{
    fn accept(&self, v: &mut V) {
        v.visit_b_node(self);
        self.a_nodes.iter().for_each(|n| n.accept(v));
    }
}

#[derive(Default)]
pub struct CollectVisitor<T>
where
    T: Copy + Display,
{
    a_nodes_values: Vec<T>,
    b_nodes_values: Vec<T>,
}

impl<T> CollectVisitor<T>
where
    T: Copy + Display,
{
    pub fn print_a_nodes(&self) {
        if !self.a_nodes_values.is_empty() {
            println!("a-nodes values:");
            self.a_nodes_values.iter().for_each(|v| print!("{} ", v));
            println!()
        } else {
            println!("a-nodes values: <empty>");
        }
    }

    pub fn print_b_nodes(&self) {
        if !self.a_nodes_values.is_empty() {
            println!("b-nodes values:");
            self.b_nodes_values.iter().for_each(|v| print!("{} ", v));
            println!()
        } else {
            println!("b-nodes values: <empty>");
        }
    }
}

impl<T> Visitor<T> for CollectVisitor<T>
where
    T: Copy + Display,
{
    fn visit_a_node(&mut self, node: &NodeA<T>) {
        self.a_nodes_values.push(node.value);
    }

    fn visit_b_node(&mut self, node: &NodeB<T>) {
        self.b_nodes_values.push(node.value);
    }
}
