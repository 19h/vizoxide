use vizoxide::{Graph, Context};
use vizoxide::layout::{apply_layout, Engine};
use vizoxide::render::{render_to_file, Format};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a GraphViz context with plugins.
    let context = Context::new_with_plugins(true, true)?;
    
    // Create a simple directed graph.
    let mut graph = Graph::new("plugin_example", true)?;
    let node1 = graph.add_node("Node1")?;
    let node2 = graph.add_node("Node2")?;
    graph.add_edge(&node1, &node2, None)?;
    
    // Apply the layout using the Neato engine.
    apply_layout(&context, &mut graph, Engine::Neato)?;
    
    // Render the graph to a PNG file.
    render_to_file(&context, &graph, Format::Png, "plugin_example.png")?;
    
    Ok(())
}
