use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine};
use graphviz::render::{render_to_writer, Format};
use std::error::Error;
use std::io::Cursor;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new context and a simple graph.
    let context = Context::new()?;
    let mut graph = Graph::new("writer_example", true)?;
    let node1 = graph.add_node("N1")?;
    let node2 = graph.add_node("N2")?;
    graph.add_edge(&node1, &node2, None)?;
    
    // Apply a force-directed layout.
    apply_layout(&context, &mut graph, Engine::Fdp)?;
    
    // Render the graph to an in-memory buffer.
    let mut buffer = Cursor::new(Vec::new());
    render_to_writer(&context, &graph, Format::Svg, &mut buffer)?;
    
    // Convert the rendered bytes to a UTF-8 string.
    let output = String::from_utf8(buffer.into_inner())?;
    println!("Rendered SVG Output:\n{}", output);
    
    Ok(())
}
