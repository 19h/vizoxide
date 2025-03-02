//! A Rust interface to the GraphViz graph visualization library.
//!
//! This crate provides an ergonomic, memory-safe, and idiomatic Rust interface to the
//! GraphViz library. It enables creation and manipulation of graphs, applying layouts,
//! and rendering to various formats.
//!
//! # Examples
//!
//! ```
//! use graphviz::{Graph, Context};
//! use graphviz::layout::{apply_layout, Engine};
//! use graphviz::render::{render_to_file, Format};
//!
//! // Create a new GraphViz context
//! let context = Context::new().unwrap();
//!
//! // Create a directed graph
//! let mut graph = Graph::new("example", true).unwrap();
//!
//! // Add nodes
//! let a = graph.add_node("A").unwrap();
//! let b = graph.add_node("B").unwrap();
//! let c = graph.add_node("C").unwrap();
//!
//! // Add edges
//! graph.add_edge(&a, &b, None).unwrap();
//! graph.add_edge(&b, &c, None).unwrap();
//! graph.add_edge(&c, &a, None).unwrap();
//!
//! // Set attributes
//! graph.set_attribute("rankdir", "LR").unwrap();
//! a.set_attribute("shape", "box").unwrap();
//! b.set_attribute("style", "filled").unwrap();
//! b.set_attribute("fillcolor", "lightblue").unwrap();
//!
//! // Apply layout
//! apply_layout(&context, &mut graph, Engine::Dot).unwrap();
//!
//! // Render to file
//! render_to_file(&context, &graph, Format::Svg, "example.svg").unwrap();
//! ```
//!
//! # Building complex graphs with the builder pattern
//!
//! ```
//! use graphviz::{Graph, Context};
//! use graphviz::layout::{apply_layout, Engine};
//! use graphviz::render::{render_to_file, Format};
//! use graphviz::attr::{graph, node, edge};
//!
//! // Create a new GraphViz context
//! let context = Context::new().unwrap();
//!
//! // Create a graph with the builder pattern
//! let mut graph = Graph::builder("complex_example")
//!     .directed(true)
//!     .attribute(graph::RANKDIR, "TB")
//!     .attribute(graph::FONTNAME, "Helvetica")
//!     .attribute(graph::NODE_SHAPE, "box")
//!     .build()
//!     .unwrap();
//!
//! // Add nodes with attributes
//! let start = graph.create_node("Start")
//!     .attribute(node::SHAPE, "ellipse")
//!     .attribute(node::STYLE, "filled")
//!     .attribute(node::FILLCOLOR, "lightgreen")
//!     .build()
//!     .unwrap();
//!
//! let process = graph.create_node("Process")
//!     .attribute(node::STYLE, "filled")
//!     .attribute(node::FILLCOLOR, "lightblue")
//!     .build()
//!     .unwrap();
//!
//! let decision = graph.create_node("Decision")
//!     .attribute(node::SHAPE, "diamond")
//!     .attribute(node::STYLE, "filled")
//!     .attribute(node::FILLCOLOR, "lightyellow")
//!     .build()
//!     .unwrap();
//!
//! let end = graph.create_node("End")
//!     .attribute(node::SHAPE, "ellipse")
//!     .attribute(node::STYLE, "filled")
//!     .attribute(node::FILLCOLOR, "lightcoral")
//!     .build()
//!     .unwrap();
//!
//! // Add edges with attributes
//! graph.create_edge(&start, &process, None)
//!     .attribute(edge::LABEL, "Begin")
//!     .build()
//!     .unwrap();
//!
//! graph.create_edge(&process, &decision, None)
//!     .attribute(edge::LABEL, "Continue")
//!     .build()
//!     .unwrap();
//!
//! graph.create_edge(&decision, &process, None)
//!     .attribute(edge::LABEL, "Retry")
//!     .attribute(edge::CONSTRAINT, "false")
//!     .build()
//!     .unwrap();
//!
//! graph.create_edge(&decision, &end, None)
//!     .attribute(edge::LABEL, "Done")
//!     .build()
//!     .unwrap();
//!
//! // Apply layout
//! apply_layout(&context, &mut graph, Engine::Dot).unwrap();
//!
//! // Render to file
//! render_to_file(&context, &graph, Format::Svg, "flowchart.svg").unwrap();
//! ```

// Re-export from modules
pub use crate::graph::{Graph, Node, Edge, GraphBuilder, NodeBuilder, EdgeBuilder};
pub use crate::layout::Context;
pub use crate::error::GraphvizError;

// Public modules
pub mod graph;
pub mod layout;
pub mod render;
pub mod attr;
pub mod error;