use hw12::visitor::{Accepter, CollectVisitor, NodeA, NodeB};

fn main() {
    let node_a_1 = NodeA::new(1);
    let node_a_2 = NodeA::new(2);
    let node_a_3 = NodeA::new(3);
    let node_a_4 = NodeA::new(4);
    let node_a_5 = NodeA::new(5);
    let node_b_6 = NodeB::with_nodes(6, vec![node_a_1, node_a_3]);
    let node_b_7 = NodeB::with_nodes(7, vec![node_a_2, node_a_4, node_a_5]);
    let node_a_8 = NodeA::with_nodes(8, vec![node_b_6, node_b_7]);

    let mut collect_visitor = CollectVisitor::<i32>::default();

    node_a_8.accept(&mut collect_visitor);

    collect_visitor.print_a_nodes();
    collect_visitor.print_b_nodes();
}
