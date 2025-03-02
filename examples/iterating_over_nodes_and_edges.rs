use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine};
use graphviz::render::{render_to_file, Format};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a GraphViz context and a simple directed graph.
    let context = Context::new()?;
    let mut graph = Graph::new("iteration_example", true)?;

    // Add several nodes.
    let node_a = graph.add_node("A")?;
    let node_b = graph.add_node("B")?;
    let node_c = graph.add_node("C")?;
    
    // Add edges connecting the nodes.
    graph.add_edge(&node_a, &node_b, None)?;
    graph.add_edge(&node_b, &node_c, None)?;
    graph.add_edge(&node_c, &node_a, None)?;

    // Iterate over all nodes in the graph.
    println!("Nodes in graph:");
    for node in graph.nodes() {
        let name = node.name()?;
        println!(" - Node: {}", name);
        
        // For each node, iterate over its outgoing edges.
        for edge in graph.out_edges(&node) {
            // 'from_node()' returns the source of the edge.
            let src = edge.from_node().name()?;
            println!("    Outgoing edge from {} (source detected as {})", name, src);
        }
    }

    // Apply a layout and render the graph to an SVG file.
    apply_layout(&context, &mut graph, Engine::Dot)?;
    render_to_file(&context, &graph, Format::Svg, "iteration_example.svg")?;
    
    Ok(())
}
