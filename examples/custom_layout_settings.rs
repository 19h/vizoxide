use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine, left_to_right_layout};
use graphviz::render::{render_to_file, Format, RenderOptions};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new GraphViz context.
    let context = Context::new()?;
    
    // Create a directed graph using the builder.
    let mut graph = Graph::builder("custom_layout")
        .directed(true)
        .attribute("label", "Custom Layout Graph")
        .build()?;
    
    // Add nodes and connect them with edges.
    let n1 = graph.add_node("N1")?;
    let n2 = graph.add_node("N2")?;
    let n3 = graph.add_node("N3")?;
    graph.add_edge(&n1, &n2, None)?;
    graph.add_edge(&n2, &n3, None)?;
    
    // Apply a left-to-right layout with custom separations.
    let layout_settings = left_to_right_layout()
        .with_nodesep(1.0)
        .with_ranksep(1.0);
    layout_settings.apply(&graph)?;
    
    // Apply layout using the Dot engine.
    apply_layout(&context, &mut graph, Engine::Dot)?;
    
    // Define and apply rendering options.
    let render_opts = RenderOptions::new()
        .with_transparency(true)
        .with_dpi(300.0)
        .with_background("white")
        .with_scale(1.5);
    render_opts.apply(&graph)?;
    
    // Render the graph to a PDF file.
    render_to_file(&context, &graph, Format::Pdf, "custom_layout.pdf")?;
    
    Ok(())
}
