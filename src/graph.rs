//! Core graph abstractions for interacting with GraphViz.
//!
//! This module provides Rust-idiomatic interfaces for creating and manipulating
//! graphs, nodes, and edges while ensuring safe memory management.

use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr;
use std::iter::Iterator;
use std::collections::HashMap;

use graphviz_sys as sys;
use crate::error::GraphvizError;
use crate::attr::AttributeContainer;

/// A GraphViz graph structure with RAII-based memory management.
pub struct Graph {
    /// Pointer to the underlying Agraph_t structure
    pub(crate) inner: *mut sys::Agraph_t,
    /// Indicates if this Graph owns the inner pointer and should free it on drop
    owned: bool,
}

/// A node within a GraphViz graph.
///
/// The lifetime parameter 'a ensures that the Node cannot outlive its parent Graph.
pub struct Node<'a> {
    /// Pointer to the underlying Agnode_t structure
    pub(crate) inner: *mut sys::Agnode_t,
    /// Phantom data to tie the Node's lifetime to the Graph
    _phantom: PhantomData<&'a Graph>,
}

/// An edge within a GraphViz graph.
///
/// The lifetime parameter 'a ensures that the Edge cannot outlive its parent Graph.
pub struct Edge<'a> {
    /// Pointer to the underlying Agedge_t structure
    pub(crate) inner: *mut sys::Agedge_t,
    /// Phantom data to tie the Edge's lifetime to the Graph
    _phantom: PhantomData<&'a Graph>,
}

/// Iterator over the nodes in a graph.
pub struct NodeIter<'a> {
    /// Reference to the parent graph
    graph: &'a Graph,
    /// Pointer to the next node in the iteration sequence
    next: *mut sys::Agnode_t,
}

/// Iterator over the edges in a graph.
pub struct EdgeIter<'a> {
    /// Reference to the parent graph
    graph: &'a Graph,
    /// Reference to the current node (for all-edges iteration)
    node: Option<&'a Node<'a>>,
    /// Pointer to the next edge in the iteration sequence
    next: *mut sys::Agedge_t,
    /// Current node for iteration (when iterating all edges)
    current_node: *mut sys::Agnode_t,
    /// Indicates if we're iterating outgoing edges
    outgoing: bool,
}

/// A builder for creating nodes with attributes.
pub struct NodeBuilder<'a> {
    /// Reference to the parent graph
    graph: &'a Graph,
    /// Name of the node to create
    name: String,
    /// Attributes to set on the node
    attributes: HashMap<String, String>,
}

/// A builder for creating edges with attributes.
pub struct EdgeBuilder<'a> {
    /// Reference to the parent graph
    graph: &'a Graph,
    /// Source node of the edge
    from: &'a Node<'a>,
    /// Target node of the edge
    to: &'a Node<'a>,
    /// Optional name for the edge
    name: Option<String>,
    /// Attributes to set on the edge
    attributes: HashMap<String, String>,
}

/// A builder for creating graphs with attributes.
pub struct GraphBuilder {
    /// Name of the graph to create
    name: String,
    /// Whether the graph is directed
    directed: bool,
    /// Whether the graph is strict (no duplicate edges)
    strict: bool,
    /// Attributes to set on the graph
    attributes: HashMap<String, String>,
}

// Graph implementation
impl Graph {
    /// Creates a new GraphViz graph with the specified name and direction.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the graph
    /// * `directed` - Whether the graph is directed or undirected
    ///
    /// # Returns
    ///
    /// A Result containing the new Graph or an error
    pub fn new(name: &str, directed: bool) -> Result<Self, GraphvizError> {
        let name = CString::new(name)?;
        let desc = if directed { 
            unsafe { sys::Agdirected } 
        } else { 
            unsafe { sys::Agundirected } 
        };
        
        let inner = unsafe { 
            sys::agopen(name.as_ptr() as *mut _, desc, ptr::null_mut()) 
        };
        
        if inner.is_null() {
            return Err(GraphvizError::GraphCreationFailed);
        }
        
        Ok(Graph { inner, owned: true })
    }
    
    /// Creates a new GraphViz graph with the specified name, direction, and strictness.
    ///
    /// Strict graphs do not allow multiple edges between the same pair of nodes.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the graph
    /// * `directed` - Whether the graph is directed or undirected
    /// * `strict` - Whether the graph is strict (no duplicate edges)
    ///
    /// # Returns
    ///
    /// A Result containing the new Graph or an error
    pub fn new_with_strictness(name: &str, directed: bool, strict: bool) -> Result<Self, GraphvizError> {
        let name = CString::new(name)?;
        let desc = match (directed, strict) {
            (true, true) => unsafe { sys::Agstrictdirected },
            (true, false) => unsafe { sys::Agdirected },
            (false, true) => unsafe { sys::Agstrictundirected },
            (false, false) => unsafe { sys::Agundirected },
        };
        
        let inner = unsafe { 
            sys::agopen(name.as_ptr() as *mut _, desc, ptr::null_mut()) 
        };
        
        if inner.is_null() {
            return Err(GraphvizError::GraphCreationFailed);
        }
        
        Ok(Graph { inner, owned: true })
    }
    
    /// Creates a new builder for configuring and creating a graph.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the graph to create
    ///
    /// # Returns
    ///
    /// A GraphBuilder instance
    pub fn builder(name: &str) -> GraphBuilder {
        GraphBuilder::new(name)
    }

    /// Adds a node to the graph with the specified name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node
    ///
    /// # Returns
    ///
    /// A Result containing the new Node or an error
    pub fn add_node(&self, name: &str) -> Result<Node, GraphvizError> {
        let name = CString::new(name)?;
        let inner = unsafe { 
            sys::agnode(self.inner, name.as_ptr() as *mut _, 1) 
        };
        
        if inner.is_null() {
            return Err(GraphvizError::NodeCreationFailed);
        }
        
        Ok(Node { inner, _phantom: PhantomData })
    }
    
    /// Creates a builder for configuring and adding a node.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node to create
    ///
    /// # Returns
    ///
    /// A NodeBuilder instance
    pub fn create_node(&self, name: &str) -> NodeBuilder {
        NodeBuilder::new(self, name)
    }
    
    /// Adds an edge between two nodes, optionally with a name.
    ///
    /// # Arguments
    ///
    /// * `from` - The source node
    /// * `to` - The target node
    /// * `name` - Optional name for the edge
    ///
    /// # Returns
    ///
    /// A Result containing the new Edge or an error
    pub fn add_edge(&self, from: &Node, to: &Node, name: Option<&str>) -> Result<Edge, GraphvizError> {
        let name_cstr = name.map(CString::new).transpose()?;
        let name_ptr = name_cstr.as_ref()
            .map_or(ptr::null_mut(), |cs| cs.as_ptr() as *mut _);
        
        let inner = unsafe { 
            sys::agedge(self.inner, from.inner, to.inner, name_ptr, 1) 
        };
        
        if inner.is_null() {
            return Err(GraphvizError::EdgeCreationFailed);
        }
        
        Ok(Edge { inner, _phantom: PhantomData })
    }
    
    /// Creates a builder for configuring and adding an edge.
    ///
    /// # Arguments
    ///
    /// * `from` - The source node
    /// * `to` - The target node
    /// * `name` - Optional name for the edge
    ///
    /// # Returns
    ///
    /// An EdgeBuilder instance
    pub fn create_edge<'a>(&'a self, from: &'a Node, to: &'a Node, name: Option<&str>) -> EdgeBuilder<'a> {
        EdgeBuilder::new(self, from, to, name)
    }
    
    /// Gets a node by name, returning None if the node doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the node to find
    ///
    /// # Returns
    ///
    /// Option containing the node if found
    pub fn get_node(&self, name: &str) -> Result<Option<Node>, GraphvizError> {
        let name = CString::new(name)?;
        let inner = unsafe { 
            sys::agnode(self.inner, name.as_ptr() as *mut _, 0) 
        };
        
        if inner.is_null() {
            Ok(None)
        } else {
            Ok(Some(Node { inner, _phantom: PhantomData }))
        }
    }
    
    /// Finds an edge between two nodes, returning None if it doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `from` - The source node
    /// * `to` - The target node
    ///
    /// # Returns
    ///
    /// Option containing the edge if found
    pub fn find_edge(&self, from: &Node, to: &Node) -> Option<Edge> {
        let inner = unsafe { 
            sys::agedge(self.inner, from.inner, to.inner, ptr::null_mut(), 0) 
        };
        
        if inner.is_null() {
            None
        } else {
            Some(Edge { inner, _phantom: PhantomData })
        }
    }
    
    /// Creates an iterator over all nodes in the graph.
    ///
    /// # Returns
    ///
    /// A NodeIter that iterates over all nodes
    pub fn nodes(&self) -> NodeIter {
        NodeIter {
            graph: self,
            next: unsafe { sys::agfstnode(self.inner) },
        }
    }
    
    /// Creates an iterator over all edges in the graph.
    ///
    /// # Returns
    ///
    /// An EdgeIter that iterates over all edges
    pub fn edges(&self) -> EdgeIter {
        let first_node = unsafe { sys::agfstnode(self.inner) };
        let first_edge = if !first_node.is_null() {
            unsafe { sys::agfstedge(self.inner, first_node) }
        } else {
            ptr::null_mut()
        };
        
        EdgeIter {
            graph: self,
            node: None,
            next: first_edge,
            current_node: first_node,
            outgoing: true,
        }
    }
    
    /// Creates an iterator over all outgoing edges from a node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to get outgoing edges from
    ///
    /// # Returns
    ///
    /// An EdgeIter that iterates over outgoing edges
    pub fn out_edges<'a>(&'a self, node: &'a Node) -> EdgeIter<'a> {
        EdgeIter {
            graph: self,
            node: Some(node),
            next: unsafe { sys::agfstout(self.inner, node.inner) },
            current_node: node.inner,
            outgoing: true,
        }
    }
    
    /// Creates an iterator over all incoming edges to a node.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to get incoming edges to
    ///
    /// # Returns
    ///
    /// An EdgeIter that iterates over incoming edges
    pub fn in_edges<'a>(&'a self, node: &'a Node) -> EdgeIter<'a> {
        EdgeIter {
            graph: self,
            node: Some(node),
            next: unsafe { sys::agfstin(self.inner, node.inner) },
            current_node: node.inner,
            outgoing: false,
        }
    }
    
    /// Gets the number of nodes in the graph.
    ///
    /// # Returns
    ///
    /// The number of nodes in the graph
    pub fn node_count(&self) -> i32 {
        unsafe { sys::agnnodes(self.inner) }
    }
    
    /// Gets the number of edges in the graph.
    ///
    /// # Returns
    ///
    /// The number of edges in the graph
    pub fn edge_count(&self) -> i32 {
        unsafe { sys::agnedges(self.inner) }
    }
    
    /// Sets an attribute on the graph.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn set_attribute(&self, name: &str, value: &str) -> Result<(), GraphvizError> {
        let name = CString::new(name)?;
        let value = CString::new(value)?;
        
        let sym = unsafe {
            sys::agattr(
                self.inner,
                sys::AGRAPH as i32,
                name.as_ptr() as *mut _,
                value.as_ptr() as *mut _,
            )
        };
        
        if sym.is_null() {
            return Err(GraphvizError::AttributeSetFailed);
        }
        
        let result = unsafe { 
            sys::agxset(self.inner as *mut _, sym, value.as_ptr() as *mut _) 
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(GraphvizError::AttributeSetFailed)
        }
    }
    
    /// Gets an attribute value from the graph.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    ///
    /// # Returns
    ///
    /// Option containing the attribute value if it exists
    pub fn get_attribute(&self, name: &str) -> Result<Option<String>, GraphvizError> {
        let name = CString::new(name)?;
        
        let value = unsafe { 
            sys::agget(self.inner as *mut _, name.as_ptr() as *mut _) 
        };
        
        if value.is_null() {
            return Ok(None);
        }
        
        let c_str = unsafe { CStr::from_ptr(value) };
        let value_str = c_str.to_str()
            .map_err(|_| GraphvizError::InvalidUtf8)?
            .to_owned();
        
        Ok(Some(value_str))
    }
    
    /// Removes a node from the graph.
    ///
    /// # Arguments
    ///
    /// * `node` - The node to remove
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn remove_node(&self, node: Node) -> Result<(), GraphvizError> {
        let result = unsafe { 
            sys::agdelnode(self.inner, node.inner) 
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(GraphvizError::NodeCreationFailed)
        }
    }
    
    /// Removes an edge from the graph.
    ///
    /// # Arguments
    ///
    /// * `edge` - The edge to remove
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    pub fn remove_edge(&self, edge: Edge) -> Result<(), GraphvizError> {
        let result = unsafe { 
            sys::agdeledge(self.inner, edge.inner) 
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(GraphvizError::EdgeCreationFailed)
        }
    }
    
    /// Gets the name of the graph.
    ///
    /// # Returns
    ///
    /// The name of the graph as a String
    pub fn name(&self) -> Result<String, GraphvizError> {
        let name = unsafe { 
            sys::agnameof(self.inner as *mut _) 
        };
        
        if name.is_null() {
            return Err(GraphvizError::NullPointer("Graph name is null"));
        }
        
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = c_str.to_str()
            .map_err(|_| GraphvizError::InvalidUtf8)?
            .to_owned();
        
        Ok(name_str)
    }
    
    /// Checks if the graph is directed.
    ///
    /// # Returns
    ///
    /// true if the graph is directed, false otherwise
    pub fn is_directed(&self) -> bool {
        unsafe { sys::agisdirected(self.inner) != 0 }
    }
    
    /// Checks if the graph is strict (no duplicate edges).
    ///
    /// # Returns
    ///
    /// true if the graph is strict, false otherwise
    pub fn is_strict(&self) -> bool {
        unsafe { sys::agisstrict(self.inner) != 0 }
    }
}

// NodeIter implementation
impl<'a> Iterator for NodeIter<'a> {
    type Item = Node<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        
        let current = self.next;
        self.next = unsafe { sys::agnxtnode(self.graph.inner, current) };
        
        Some(Node {
            inner: current,
            _phantom: PhantomData,
        })
    }
}

// EdgeIter implementation
impl<'a> Iterator for EdgeIter<'a> {
    type Item = Edge<'a>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            // If we're iterating over all edges in the graph and current node is exhausted
            if self.node.is_none() && !self.current_node.is_null() {
                // Move to the next node
                self.current_node = unsafe { sys::agnxtnode(self.graph.inner, self.current_node) };
                
                if self.current_node.is_null() {
                    return None;
                }
                
                // Get the first edge for the new node
                self.next = unsafe { sys::agfstedge(self.graph.inner, self.current_node) };
                
                if self.next.is_null() {
                    return self.next();  // Recursively try the next node
                }
            } else {
                return None;
            }
        }
        
        let current = self.next;
        
        // Get the next edge
        self.next = if let Some(_node) = self.node {
            if self.outgoing {
                unsafe { sys::agnxtout(self.graph.inner, current) }
            } else {
                unsafe { sys::agnxtin(self.graph.inner, current) }
            }
        } else {
            unsafe { sys::agnxtedge(self.graph.inner, current, self.current_node) }
        };
        
        Some(Edge {
            inner: current,
            _phantom: PhantomData,
        })
    }
}

// NodeBuilder implementation
impl<'a> NodeBuilder<'a> {
    /// Creates a new NodeBuilder.
    ///
    /// # Arguments
    ///
    /// * `graph` - The parent graph
    /// * `name` - The name of the node to create
    ///
    /// # Returns
    ///
    /// A new NodeBuilder instance
    pub fn new(graph: &'a Graph, name: &str) -> Self {
        NodeBuilder {
            graph,
            name: name.to_owned(),
            attributes: HashMap::new(),
        }
    }
    
    /// Sets an attribute on the node.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        self.attributes.insert(name.to_owned(), value.to_owned());
        self
    }
    
    /// Builds and creates the node with the configured attributes.
    ///
    /// # Returns
    ///
    /// Result containing the new Node or an error
    pub fn build(self) -> Result<Node<'a>, GraphvizError> {
        let node = self.graph.add_node(&self.name)?;
        
        for (name, value) in self.attributes {
            node.set_attribute(&name, &value)?;
        }
        
        Ok(node)
    }
}

// EdgeBuilder implementation
impl<'a> EdgeBuilder<'a> {
    /// Creates a new EdgeBuilder.
    ///
    /// # Arguments
    ///
    /// * `graph` - The parent graph
    /// * `from` - The source node
    /// * `to` - The target node
    /// * `name` - Optional name for the edge
    ///
    /// # Returns
    ///
    /// A new EdgeBuilder instance
    pub fn new(graph: &'a Graph, from: &'a Node, to: &'a Node, name: Option<&str>) -> Self {
        EdgeBuilder {
            graph,
            from,
            to,
            name: name.map(String::from),
            attributes: HashMap::new(),
        }
    }
    
    /// Sets an attribute on the edge.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        self.attributes.insert(name.to_owned(), value.to_owned());
        self
    }
    
    /// Builds and creates the edge with the configured attributes.
    ///
    /// # Returns
    ///
    /// Result containing the new Edge or an error
    pub fn build(self) -> Result<Edge<'a>, GraphvizError> {
        let name_ref = self.name.as_deref();
        let edge = self.graph.add_edge(self.from, self.to, name_ref)?;
        
        for (name, value) in self.attributes {
            edge.set_attribute(&name, &value)?;
        }
        
        Ok(edge)
    }
}

// GraphBuilder implementation
impl GraphBuilder {
    /// Creates a new GraphBuilder.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the graph to create
    ///
    /// # Returns
    ///
    /// A new GraphBuilder instance
    pub fn new(name: &str) -> Self {
        GraphBuilder {
            name: name.to_owned(),
            directed: true,
            strict: false,
            attributes: HashMap::new(),
        }
    }
    
    /// Sets whether the graph is directed.
    ///
    /// # Arguments
    ///
    /// * `directed` - Whether the graph is directed
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn directed(mut self, directed: bool) -> Self {
        self.directed = directed;
        self
    }
    
    /// Sets whether the graph is strict (no duplicate edges).
    ///
    /// # Arguments
    ///
    /// * `strict` - Whether the graph is strict
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }
    
    /// Sets an attribute on the graph.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn attribute(mut self, name: &str, value: &str) -> Self {
        self.attributes.insert(name.to_owned(), value.to_owned());
        self
    }
    
    /// Builds and creates the graph with the configured attributes.
    ///
    /// # Returns
    ///
    /// Result containing the new Graph or an error
    pub fn build(self) -> Result<Graph, GraphvizError> {
        let graph = Graph::new_with_strictness(&self.name, self.directed, self.strict)?;
        
        for (name, value) in self.attributes {
            graph.set_attribute(&name, &value)?;
        }
        
        Ok(graph)
    }
}

// RAII implementation for Graph
impl Drop for Graph {
    fn drop(&mut self) {
        if self.owned && !self.inner.is_null() {
            unsafe { sys::agclose(self.inner) };
        }
    }
}

// Node implementation
impl<'a> Node<'a> {
    /// Gets the name of the node.
    ///
    /// # Returns
    ///
    /// The name of the node as a String
    pub fn name(&self) -> Result<String, GraphvizError> {
        let name = unsafe { sys::agnameof(self.inner as *mut _) };
        
        if name.is_null() {
            return Err(GraphvizError::NullPointer("Node name is null"));
        }
        
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = c_str.to_str()
            .map_err(|_| GraphvizError::InvalidUtf8)?
            .to_owned();
        
        Ok(name_str)
    }
    
    /// Gets the parent graph of this node.
    ///
    /// # Returns
    ///
    /// The parent graph as an unowned Graph reference
    pub fn graph(&self) -> Graph {
        let graph_ptr = unsafe { sys::agraphof(self.inner as *mut _) };
        
        Graph {
            inner: graph_ptr,
            owned: false,  // We don't own this graph, just referencing it
        }
    }
}

// Edge implementation
impl<'a> Edge<'a> {
    /// Retrieves the source node (tail) of this edge.
    ///
    /// # Returns
    ///
    /// The source node of the edge.
    pub fn from_node(&self) -> Node<'a> {
        // Directed graphs utilize a specific edge representation model
        let graph_ptr = unsafe { sys::agraphof(self.inner as *mut _) };
        let is_directed = unsafe { sys::agisdirected(graph_ptr) != 0 };
        
        if is_directed {
            // Determine edge type to identify proper node extraction approach
            let edge_type = unsafe { (*self.inner).base.tag.objtype() };
            
            if edge_type == sys::AGOUTEDGE as u32 {
                // For outgoing edges, obtain a reference to the edge pair
                // and extract source node by utilizing structural knowledge
                unsafe {
                    // AGOUTEDGE: The current node field represents destination;
                    // source must be determined through alternative means
                    let source_node = self.determine_source_through_graph_traversal(graph_ptr);
                    if !source_node.is_null() {
                        return Node {
                            inner: source_node,
                            _phantom: PhantomData,
                        };
                    }
                    
                    // Fallback: Return a node reference with available information
                    let node_ref = self.get_opposite_node((*self.inner).node, graph_ptr);
                    Node {
                        inner: node_ref,
                        _phantom: PhantomData,
                    }
                }
            } else if edge_type == sys::AGINEDGE as u32 {
                // For incoming edges, the node field represents the source
                unsafe {
                    Node {
                        inner: (*self.inner).node,
                        _phantom: PhantomData,
                    }
                }
            } else {
                // Default case for unexpected edge configuration
                unsafe {
                    Node {
                        inner: (*self.inner).node,
                        _phantom: PhantomData,
                    }
                }
            }
        } else {
            // Undirected graph edge node access
            // In undirected graphs, the convention is to return node as origin
            unsafe {
                // Determine source through graph investigation
                let potential_source = self.determine_source_for_undirected(graph_ptr);
                if !potential_source.is_null() {
                    Node {
                        inner: potential_source,
                        _phantom: PhantomData,
                    }
                } else {
                    // Fallback to available node reference
                    Node {
                        inner: (*self.inner).node,
                        _phantom: PhantomData,
                    }
                }
            }
        }
    }
    
    // Auxiliary methods for node determination
    
    /// Determines source node through graph traversal for directed edges.
    unsafe fn determine_source_through_graph_traversal(&self, graph: *mut sys::Agraph_t) -> *mut sys::Agnode_t {
        let target_node = (*self.inner).node;
        let mut current_node = sys::agfstnode(graph);
        
        // Systematically examine all nodes to identify source
        while !current_node.is_null() {
            if current_node != target_node {
                // Check if current_node has an edge to target_node
                let edge = sys::agedge(graph, current_node, target_node, std::ptr::null_mut(), 0);
                if !edge.is_null() && edge == self.inner {
                    return current_node;
                }
            }
            current_node = sys::agnxtnode(graph, current_node);
        }
        
        std::ptr::null_mut()
    }
    
    /// Determines source for undirected edges through structural analysis.
    unsafe fn determine_source_for_undirected(&self, graph: *mut sys::Agraph_t) -> *mut sys::Agnode_t {
        // For undirected edges, the designation of source is somewhat arbitrary
        // This implementation identifies a logical "source" based on internal edge structure
        let node_target = (*self.inner).node;
        let mut node_iter = sys::agfstnode(graph);
        
        while !node_iter.is_null() {
            if node_iter != node_target {
                let edge_check = sys::agedge(graph, node_iter, node_target, std::ptr::null_mut(), 0);
                if edge_check == self.inner {
                    return node_iter;
                }
            }
            node_iter = sys::agnxtnode(graph, node_iter);
        }
        
        std::ptr::null_mut()
    }
    
    /// Retrieves opposite node when one endpoint is known.
    unsafe fn get_opposite_node(&self, known_node: *mut sys::Agnode_t, graph: *mut sys::Agraph_t) -> *mut sys::Agnode_t {
        let mut node_scan = sys::agfstnode(graph);
        
        while !node_scan.is_null() {
            if node_scan != known_node {
                let test_edge = sys::agedge(graph, node_scan, known_node, std::ptr::null_mut(), 0);
                if test_edge == self.inner || 
                   test_edge == self.inner.cast::<sys::Agedgepair_s>().offset(1).cast() {
                    return node_scan;
                }
            }
            node_scan = sys::agnxtnode(graph, node_scan);
        }
        
        // Default behavior if opposite node cannot be determined
        known_node
    }
}

// AttributeContainer implementations for Graph, Node, and Edge
impl AttributeContainer for Graph {
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), GraphvizError> {
        self.set_attribute(name, value)
    }
    
    fn get_attribute(&self, name: &str) -> Result<Option<String>, GraphvizError> {
        self.get_attribute(name)
    }
}

impl<'a> AttributeContainer for Node<'a> {
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), GraphvizError> {
        let graph = unsafe { sys::agraphof(self.inner as *mut _) };
        let name_cstr = CString::new(name)?;
        let value_cstr = CString::new(value)?;
        let empty_str = CString::new("")?;
        
        // First create/get the attribute with empty string as default
        // This avoids setting a meaningful default for all nodes
        let sym = unsafe {
            sys::agattr(
                graph,
                sys::AGNODE as i32,
                name_cstr.as_ptr() as *mut _,
                empty_str.as_ptr()
            )
        };
        
        if sym.is_null() {
            return Err(GraphvizError::AttributeSetFailed);
        }
        
        // Now set the value only on this specific node
        let result = unsafe { 
            sys::agxset(self.inner as *mut _, sym, value_cstr.as_ptr() as *mut _) 
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(GraphvizError::AttributeSetFailed)
        }
    }
    
    fn get_attribute(&self, name: &str) -> Result<Option<String>, GraphvizError> {
        let name = CString::new(name)?;
        
        let value = unsafe { sys::agget(self.inner as *mut _, name.as_ptr() as *mut _) };
        
        if value.is_null() {
            return Ok(None);
        }
        
        let c_str = unsafe { CStr::from_ptr(value) };
        let value_str = c_str.to_str()
            .map_err(|_| GraphvizError::InvalidUtf8)?
            .to_owned();
        
        Ok(Some(value_str))
    }
}

impl<'a> AttributeContainer for Edge<'a> {
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), GraphvizError> {
        let graph = unsafe { sys::agraphof(self.inner as *mut _) };
        let name_cstr = CString::new(name)?;
        let value_cstr = CString::new(value)?;
        let empty_str = CString::new("")?;
        
        // First create/get the attribute with empty string as default
        // This avoids setting a meaningful default for all edges
        let sym = unsafe {
            sys::agattr(
                graph,
                sys::AGEDGE as i32,
                name_cstr.as_ptr() as *mut _,
                empty_str.as_ptr()
            )
        };
        
        if sym.is_null() {
            return Err(GraphvizError::AttributeSetFailed);
        }
        
        // Now set the value only on this specific edge
        let result = unsafe { 
            sys::agxset(self.inner as *mut _, sym, value_cstr.as_ptr() as *mut _) 
        };
        
        if result == 0 {
            Ok(())
        } else {
            Err(GraphvizError::AttributeSetFailed)
        }
    }
    
    fn get_attribute(&self, name: &str) -> Result<Option<String>, GraphvizError> {
        let name = CString::new(name)?;
        
        let value = unsafe { sys::agget(self.inner as *mut _, name.as_ptr() as *mut _) };
        
        if value.is_null() {
            return Ok(None);
        }
        
        let c_str = unsafe { CStr::from_ptr(value) };
        let value_str = c_str.to_str()
            .map_err(|_| GraphvizError::InvalidUtf8)?
            .to_owned();
        
        Ok(Some(value_str))
    }
}
