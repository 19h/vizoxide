use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine};
use graphviz::render::{render_to_file, Format};
use graphviz::attr::{graph, node, edge};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the GraphViz context.
    let context = Context::new()?;
    
    // Create a directed graph using the builder pattern.
    let mut graph = Graph::builder("complex_example")
        .directed(true)
        .attribute(graph::RANKDIR, "TB")
        .attribute(graph::FONTNAME, "Helvetica")
        .attribute(graph::NODE_SHAPE, "box")
        .build()?;
    
    // Create nodes with specific attributes.
    let start = graph.create_node("Start")
        .attribute(node::SHAPE, "ellipse")
        .attribute(node::STYLE, "filled")
        .attribute(node::FILLCOLOR, "lightgreen")
        .build()?;
    
    let process = graph.create_node("Process")
        .attribute(node::STYLE, "filled")
        .attribute(node::FILLCOLOR, "lightblue")
        .build()?;
    
    let decision = graph.create_node("Decision")
        .attribute(node::SHAPE, "diamond")
        .attribute(node::STYLE, "filled")
        .attribute(node::FILLCOLOR, "lightyellow")
        .build()?;
    
    let end = graph.create_node("End")
        .attribute(node::SHAPE, "ellipse")
        .attribute(node::STYLE, "filled")
        .attribute(node::FILLCOLOR, "lightcoral")
        .build()?;
    
    // Add edges with labels and other attributes.
    graph.create_edge(&start, &process, None)
        .attribute(edge::LABEL, "Begin")
        .build()?;
    
    graph.create_edge(&process, &decision, None)
        .attribute(edge::LABEL, "Continue")
        .build()?;
    
    graph.create_edge(&decision, &process, None)
        .attribute(edge::LABEL, "Retry")
        .attribute(edge::CONSTRAINT, "false")
        .build()?;
    
    graph.create_edge(&decision, &end, None)
        .attribute(edge::LABEL, "Done")
        .build()?;
    
    // Apply the layout using the Dot engine.
    apply_layout(&context, &mut graph, Engine::Dot)?;
    
    // Render the graph to an SVG file.
    render_to_file(&context, &graph, Format::Svg, "complex_graph.svg")?;
    
    Ok(())
}
