use vizoxide::{Graph, Context};
use vizoxide::layout::{apply_layout, Engine};
use vizoxide::render::{render_to_file, Format};
use vizoxide::attr::AttributeContainer;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let context = Context::new()?;
    let mut graph = Graph::new("modify_attribute", true)?;
    
    // Set an attribute on the graph.
    graph.set_attribute("bgcolor", "red")?;
    // Remove the attribute (GraphViz removal is implemented by setting an empty string).
    graph.remove_attribute("bgcolor")?;
    
    // Retrieve and print the attribute to verify removal.
    let bgcolor = graph.get_attribute("bgcolor")?;
    println!("Graph 'bgcolor' after removal: {:?}", bgcolor);  // Expected output: None or an empty string
    
    // Create a node, set an attribute, then remove it.
    let node = graph.add_node("NodeX")?;
    node.set_attribute("style", "filled")?;
    node.remove_attribute("style")?;
    let style = node.get_attribute("style")?;
    println!("Node 'style' after removal: {:?}", style);
    
    apply_layout(&context, &mut graph, Engine::Dot)?;
    render_to_file(&context, &graph, Format::Svg, "modify_attribute.svg")?;
    
    Ok(())
}
