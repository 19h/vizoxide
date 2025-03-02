# vizoxide - An idiomatic GraphViz wrapper for Rust

[![Crates.io](https://img.shields.io/crates/v/vizoxide.svg)](https://crates.io/crates/vizoxide)
[![Documentation](https://docs.rs/vizoxide/badge.svg)](https://docs.rs/vizoxide)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

A comprehensive, memory-safe Rust wrapper for the GraphViz graph visualization library. This crate provides an ergonomic, idiomatic Rust interface for creating, manipulating, and rendering graphs.

## Features

- **Memory-safe wrapping** of GraphViz's C API with proper RAII patterns
- **Builder patterns** for fluent, expressive graph creation
- **Complete layout control** with support for all GraphViz engines
- **Extensive rendering options** including SVG, PNG, PDF, and more
- **Rich attribute handling** with predefined constants for common graph, node, and edge attributes
- **Comprehensive error handling** with descriptive error types
- **Iterator support** for traversing nodes and edges

## Prerequisites

This crate requires the GraphViz library to be installed on your system. Installation instructions vary by platform:

### Linux
```
sudo apt-get install graphviz libgraphviz-dev
```

### macOS
```
brew install graphviz
```

### Windows
Download and install from [GraphViz's official site](https://graphviz.org/download/) or use:
```
choco install graphviz
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
graphviz = "1.0.0"
```

## Basic Usage

```rust
use graphviz::{Graph, Context};
use graphviz::layout::{apply_layout, Engine};
use graphviz::render::{render_to_file, Format};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a new GraphViz context
    let context = Context::new()?;
    
    // Create a directed graph
    let mut graph = Graph::new("example", true)?;
    
    // Add nodes
    let a = graph.add_node("A")?;
    let b = graph.add_node("B")?;
    let c = graph.add_node("C")?;
    
    // Add edges
    graph.add_edge(&a, &b, None)?;
    graph.add_edge(&b, &c, None)?;
    graph.add_edge(&c, &a, None)?;
    
    // Set attributes
    graph.set_attribute("rankdir", "LR")?;
    a.set_attribute("shape", "box")?;
    b.set_attribute("style", "filled")?;
    b.set_attribute("fillcolor", "lightblue")?;
    
    // Apply layout
    apply_layout(&context, &mut graph, Engine::Dot)?;
    
    // Render to a file
    render_to_file(&context, &graph, Format::Svg, "example.svg")?;
    
    Ok(())
}
```

## Core Concepts

### Context

A `Context` represents the GraphViz runtime environment. It's required for layout and rendering operations:

```rust
let context = Context::new()?;
```

### Graphs

Create directed or undirected graphs:

```rust
// Directed graph
let directed = Graph::new("my_graph", true)?;

// Undirected graph
let undirected = Graph::new("my_graph", false)?;

// Strict graph (no duplicate edges)
let strict = Graph::new_with_strictness("my_graph", true, true)?;
```

### Nodes & Edges

Nodes and edges are the building blocks of graphs:

```rust
// Add nodes
let node1 = graph.add_node("Node1")?;
let node2 = graph.add_node("Node2")?;

// Add an edge
let edge = graph.add_edge(&node1, &node2, None)?;
```

### Attributes

Customize the appearance of graphs, nodes, and edges with attributes:

```rust
// Set attributes directly
graph.set_attribute("bgcolor", "lightgray")?;
node.set_attribute("shape", "box")?;
edge.set_attribute("color", "red")?;

// Check and remove attributes
if node.has_attribute("color")? {
    node.remove_attribute("color")?;
}

// Get attribute value
let shape = node.get_attribute("shape")?;
```

The `attr` module provides constants for common attribute names:

```rust
use graphviz::attr::{graph, node, edge, values};

// Use predefined attribute constants
graph_obj.set_attribute(graph::RANKDIR, "LR")?;
node_obj.set_attribute(node::SHAPE, values::shape::BOX)?;
edge_obj.set_attribute(edge::STYLE, values::style::DASHED)?;
```

## Builder Pattern

The crate supports a fluent builder pattern for creating graph elements:

```rust
// Create a graph with the builder
let graph = Graph::builder("flowchart")
    .directed(true)
    .strict(false)
    .attribute("rankdir", "TD")
    .build()?;

// Create a node with attributes
let start = graph.create_node("start")
    .attribute("shape", "ellipse")
    .attribute("style", "filled")
    .attribute("fillcolor", "lightblue")
    .build()?;

// Create an edge with attributes
let edge = graph.create_edge(&start, &process, None)
    .attribute("label", "Begin")
    .attribute("color", "blue")
    .build()?;
```

## Layout Engines

GraphViz supports various layout algorithms:

```rust
// Basic layout application
apply_layout(&context, &mut graph, Engine::Dot)?;

// Other available engines
apply_layout(&context, &mut graph, Engine::Neato)?;  // Spring model
apply_layout(&context, &mut graph, Engine::Fdp)?;    // Force-directed
apply_layout(&context, &mut graph, Engine::Circo)?;  // Circular
apply_layout(&context, &mut graph, Engine::Twopi)?;  // Radial
apply_layout(&context, &mut graph, Engine::Sfdp)?;   // Multiscale
```

### Layout Settings

Customize layout behavior with predefined or custom settings:

```rust
// Using predefined layout settings
let radial_settings = radial_layout();
radial_settings.apply(&graph)?;

// Using custom layout settings
let custom_settings = LayoutSettings::new()
    .with_overlap("false")
    .with_splines("ortho")
    .with_nodesep(0.75)
    .with_ranksep(1.0);
custom_settings.apply(&graph)?;
```

## Rendering

### Render to File

```rust
render_to_file(&context, &graph, Format::Svg, "output.svg")?;
render_to_file(&context, &graph, Format::Png, "output.png")?;
render_to_file(&context, &graph, Format::Pdf, "output.pdf")?;
```

### Render to Memory

```rust
// Render to string (for text formats like SVG, DOT)
let svg_string = render_to_string(&context, &graph, Format::Svg)?;

// Render to bytes (for any format)
let png_bytes = render_to_bytes(&context, &graph, Format::Png)?;

// Render to a writer
let mut buffer = Cursor::new(Vec::new());
render_to_writer(&context, &graph, Format::Svg, &mut buffer)?;
```

### Supported Formats

The crate supports all GraphViz output formats including:

- Vector formats: SVG, PDF, PS, EPS
- Raster formats: PNG, JPEG, GIF, BMP
- Text formats: DOT, XDOT, JSON, Plain
- And many more accessible via `Format` enum

## Graph Traversal

Iterate over nodes and edges:

```rust
// Iterate through all nodes
for node in graph.nodes() {
    println!("Node: {}", node.name()?);
    
    // Iterate through outgoing edges
    for edge in graph.out_edges(&node) {
        let target = edge.to_node().name()?;
        println!("  -> {}", target);
    }
}

// Check graph properties
println!("Node count: {}", graph.node_count());
println!("Edge count: {}", graph.edge_count());
println!("Directed: {}", graph.is_directed());
```

## Advanced Usage

### Custom Plugins

```rust
// Create a context with custom plugin settings
let context = Context::new_with_plugins(true, false)?;
```

### Render Options

```rust
// Customize rendering with options
let options = RenderOptions::new()
    .with_anti_alias(true)
    .with_transparency(true)
    .with_dpi(300.0)
    .with_background("white")
    .with_scale(2.0);

// Apply options when rendering
render_with_options(&context, &graph, Format::Png, "high_quality.png", &options)?;
```

## Examples

The crate includes several examples demonstrating various features:

- `simple_graph` - Basic graph creation and rendering
- `complex_graph_with_build_pattern` - Using the builder pattern
- `edge_builder_with_multiple_attributes` - Working with edge attributes
- `predefined_layout_settings` - Using predefined layouts
- `creating_context_with_plugins` - Working with GraphViz plugins
- `iterating_over_nodes_and_edges` - Graph traversal
- `render_graph_to_string_and_bytes` - In-memory rendering
- `custom_layout_settings` - Advanced layout configuration
- `modifying_attributes_and_removing_them` - Attribute manipulation
- `render_graph_to_writer` - Rendering to custom output

Run any example with:

```
cargo run --example example_name
```

## Error Handling

The crate uses the `GraphvizError` type to represent various error conditions:

```rust
match result {
    Ok(graph) => println!("Graph created successfully"),
    Err(GraphvizError::GraphCreationFailed) => println!("Failed to create graph"),
    Err(GraphvizError::LayoutFailed) => println!("Failed to apply layout"),
    Err(GraphvizError::RenderFailed) => println!("Failed to render graph"),
    Err(e) => println!("Other error: {}", e),
}
```

## License

This crate is available under the MIT License. See the LICENSE file for more details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
