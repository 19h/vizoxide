use vizoxide::{Graph, Context};
use vizoxide::layout::{
    apply_layout, Engine, radial_layout, force_directed_layout
};
use vizoxide::render::{render_to_file, Format};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new()?;
    
    // --- Radial Layout Example ---
    let mut graph_radial = Graph::new("radial_layout", true)?;
    // Create nodes in a circular formation.
    let node_names = vec!["A", "B", "C", "D", "E"];
    let mut nodes = Vec::new();
    for name in node_names {
        let node = graph_radial.add_node(name)?;
        nodes.push(node);
    }
    // Connect each node to form a cycle.
    for i in 0..nodes.len() {
        let next_index = (i + 1) % nodes.len();
        graph_radial.add_edge(&nodes[i], &nodes[next_index], None)?;
    }
    // Apply a radial layout using predefined settings.
    let radial_settings = radial_layout();
    radial_settings.apply(&graph_radial)?;
    apply_layout(&context, &mut graph_radial, Engine::Circo)?;
    render_to_file(&context, &graph_radial, Format::Svg, "radial_layout.svg")?;
    
    // --- Force-Directed Layout Example ---
    let mut graph_force = Graph::new("force_directed", true)?;
    let f_node1 = graph_force.add_node("X")?;
    let f_node2 = graph_force.add_node("Y")?;
    let f_node3 = graph_force.add_node("Z")?;
    graph_force.add_edge(&f_node1, &f_node2, None)?;
    graph_force.add_edge(&f_node2, &f_node3, None)?;
    graph_force.add_edge(&f_node3, &f_node1, None)?;
    
    // Apply force-directed layout settings.
    let force_settings = force_directed_layout();
    force_settings.apply(&graph_force)?;
    apply_layout(&context, &mut graph_force, Engine::Fdp)?;
    render_to_file(&context, &graph_force, Format::Svg, "force_directed_layout.svg")?;
    
    Ok(())
}
