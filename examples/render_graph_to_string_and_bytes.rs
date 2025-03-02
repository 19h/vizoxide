use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine};
use graphviz::render::{render_to_string, render_to_bytes, Format};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new GraphViz context.
    let context = Context::new()?;
    
    // Build a simple directed graph.
    let mut graph = Graph::new("string_render", true)?;
    let node1 = graph.add_node("Node1")?;
    let node2 = graph.add_node("Node2")?;
    graph.add_edge(&node1, &node2, None)?;
    
    // Apply layout using the Neato engine.
    apply_layout(&context, &mut graph, Engine::Neato)?;
    
    // Render graph to a text string in SVG format.
    let svg_output = render_to_string(&context, &graph, Format::Svg)?;
    println!("SVG Output:\n{}", svg_output);
    
    // Render graph to bytes in PNG format.
    let png_bytes = render_to_bytes(&context, &graph, Format::Png)?;
    println!("PNG output size: {} bytes", png_bytes.len());
    
    Ok(())
}
