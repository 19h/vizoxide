use vizoxide::{Graph, Context};
use vizoxide::layout::{apply_layout, Engine};
use vizoxide::render::{render_to_file, Format};
use vizoxide::attr::edge;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize context and create a simple directed graph.
    let context = Context::new()?;
    let mut graph = Graph::new("edge_builder_example", true)?;
    
    let n1 = graph.add_node("N1")?;
    let n2 = graph.add_node("N2")?;
    
    // Create an edge using the builder with multiple attribute settings.
    graph.create_edge(&n1, &n2, Some("edge1"))
         .attribute(edge::LABEL, "Edge from N1 to N2")
         .attribute(edge::COLOR, "blue")
         .attribute(edge::STYLE, "dashed")
         .build()?;
    
    apply_layout(&context, &mut graph, Engine::Dot)?;
    render_to_file(&context, &graph, Format::Svg, "edge_builder_example.svg")?;
    
    Ok(())
}
