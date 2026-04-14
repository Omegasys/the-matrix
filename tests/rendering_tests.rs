use crate::client::renderer::scene_graph::{Scene, Node};

#[test]
fn test_scene_creation() {
    let mut scene = Scene::new();

    let child = Node::new(1, "child");
    scene.root.add_child(child);

    assert_eq!(scene.root.children.len(), 1);
}
