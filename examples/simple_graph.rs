use vizoxide::{Graph, Context};
use vizoxide::layout::{apply_layout, Engine};
use vizoxide::render::{render_to_file, Format};
use vizoxide::attr::AttributeContainer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new GraphViz context.
    let context = Context::new()?;
    
    // Create a directed graph named "basic_graph".
    let mut graph = Graph::new("basic_graph", true)?;
    
    // Add nodes to the graph.
    let node_a = graph.add_node("A")?;
    let node_b = graph.add_node("B")?;
    let node_c = graph.add_node("C")?;
    
    // Add edges between nodes.
    graph.add_edge(&node_a, &node_b, None)?;
    graph.add_edge(&node_b, &node_c, None)?;
    graph.add_edge(&node_c, &node_a, None)?;
    
    // Set a graph attribute to change the layout direction.
    graph.set_attribute("rankdir", "LR")?;
    
    // Customize node appearance via attributes.
    node_a.set_attribute("shape", "box")?;
    node_b.set_attribute("style", "filled")?;
    node_b.set_attribute("fillcolor", "lightblue")?;
    
    // Debug prints to check attribute values
    println!("Node A attributes:");
    println!("  shape: {:?}", node_a.get_attribute("shape")?);
    println!("  style: {:?}", node_a.get_attribute("style")?);
    println!("  fillcolor: {:?}", node_a.get_attribute("fillcolor")?);
    
    println!("Node B attributes:");
    println!("  shape: {:?}", node_b.get_attribute("shape")?);
    println!("  style: {:?}", node_b.get_attribute("style")?);
    println!("  fillcolor: {:?}", node_b.get_attribute("fillcolor")?);
    
    println!("Node C attributes:");
    println!("  shape: {:?}", node_c.get_attribute("shape")?);
    println!("  style: {:?}", node_c.get_attribute("style")?);
    println!("  fillcolor: {:?}", node_c.get_attribute("fillcolor")?);
    
    // Apply the layout using the Dot engine.
    apply_layout(&context, &mut graph, Engine::Dot)?;
    
    // Render the graph to an SVG file.
    render_to_file(&context, &graph, Format::Svg, "basic_graph.svg")?;
    
    Ok(())
}
