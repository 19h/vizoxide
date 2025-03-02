//! Attribute manipulation for GraphViz objects.
//!
//! This module provides utilities for working with attributes on GraphViz objects
//! (graphs, nodes, and edges).

use crate::error::GraphvizError;

/// A trait for types that can have attributes set on them.
pub trait AttributeContainer {
    /// Sets an attribute on the container.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn set_attribute(&self, name: &str, value: &str) -> Result<(), GraphvizError>;
    
    /// Gets an attribute value from the container.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    ///
    /// # Returns
    ///
    /// Option containing the attribute value if it exists
    fn get_attribute(&self, name: &str) -> Result<Option<String>, GraphvizError>;
    
    /// Checks if an attribute exists on the container.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    ///
    /// # Returns
    ///
    /// true if the attribute exists, false otherwise
    fn has_attribute(&self, name: &str) -> Result<bool, GraphvizError> {
        Ok(self.get_attribute(name)?.is_some())
    }
    
    /// Sets an attribute if it doesn't already exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    /// * `value` - The attribute value
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn set_attribute_if_absent(&self, name: &str, value: &str) -> Result<(), GraphvizError> {
        if !self.has_attribute(name)? {
            self.set_attribute(name, value)?;
        }
        Ok(())
    }
    
    /// Removes an attribute if it exists.
    ///
    /// Note: GraphViz doesn't actually support removing attributes,
    /// so this implementation sets the attribute to an empty string.
    ///
    /// # Arguments
    ///
    /// * `name` - The attribute name
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    fn remove_attribute(&self, name: &str) -> Result<(), GraphvizError> {
        if self.has_attribute(name)? {
            self.set_attribute(name, "")?;
        }
        Ok(())
    }
}

/// Common GraphViz attribute names for graphs.
pub mod graph {
    /// The direction of graph layout.
    pub const RANKDIR: &str = "rankdir";
    /// The size of the graph.
    pub const SIZE: &str = "size";
    /// The ratio of height to width.
    pub const RATIO: &str = "ratio";
    /// The font name for labels.
    pub const FONTNAME: &str = "fontname";
    /// The font size for labels.
    pub const FONTSIZE: &str = "fontsize";
    /// The font color for labels.
    pub const FONTCOLOR: &str = "fontcolor";
    /// The graph's label.
    pub const LABEL: &str = "label";
    /// The background color of the graph.
    pub const BGCOLOR: &str = "bgcolor";
    /// Control the width of the page for pagination.
    pub const PAGE: &str = "page";
    /// Control the margin around the graph.
    pub const MARGIN: &str = "margin";
    /// The style of the graph.
    pub const STYLE: &str = "style";
    /// Whether to concentrate edges.
    pub const CONCENTRATE: &str = "concentrate";
    /// The URL to associate with the graph.
    pub const URL: &str = "URL";
    /// The ordering of nodes.
    pub const ORDERING: &str = "ordering";
    /// The rank separation between nodes.
    pub const RANKSEP: &str = "ranksep";
    /// The node separation within a rank.
    pub const NODESEP: &str = "nodesep";
    /// The color of edges.
    pub const EDGE_COLOR: &str = "edge[color]";
    /// The style of edges.
    pub const EDGE_STYLE: &str = "edge[style]";
    /// The default direction for edges.
    pub const EDGE_DIR: &str = "edge[dir]";
    /// The color of nodes.
    pub const NODE_COLOR: &str = "node[color]";
    /// The style of nodes.
    pub const NODE_STYLE: &str = "node[style]";
    /// The shape of nodes.
    pub const NODE_SHAPE: &str = "node[shape]";
    /// Whether to rotate the graph.
    pub const ROTATE: &str = "rotate";
    /// The splines setting for edges.
    pub const SPLINES: &str = "splines";
    /// The overlap removal algorithm.
    pub const OVERLAP: &str = "overlap";
}

/// Common GraphViz attribute names for nodes.
pub mod node {
    /// The shape of the node.
    pub const SHAPE: &str = "shape";
    /// The label of the node.
    pub const LABEL: &str = "label";
    /// The color of the node.
    pub const COLOR: &str = "color";
    /// The fill color of the node.
    pub const FILLCOLOR: &str = "fillcolor";
    /// The style of the node.
    pub const STYLE: &str = "style";
    /// The font name for the node label.
    pub const FONTNAME: &str = "fontname";
    /// The font size for the node label.
    pub const FONTSIZE: &str = "fontsize";
    /// The font color for the node label.
    pub const FONTCOLOR: &str = "fontcolor";
    /// The width of the node.
    pub const WIDTH: &str = "width";
    /// The height of the node.
    pub const HEIGHT: &str = "height";
    /// The minimum width of the node.
    pub const FIXEDSIZE: &str = "fixedsize";
    /// The URL to associate with the node.
    pub const URL: &str = "URL";
    /// The tooltip for the node.
    pub const TOOLTIP: &str = "tooltip";
    /// Position of the node.
    pub const POS: &str = "pos";
    /// Group for the node.
    pub const GROUP: &str = "group";
    /// The image file to display in the node.
    pub const IMAGE: &str = "image";
    /// The shape's distortion.
    pub const DISTORTION: &str = "distortion";
    /// The shape's skew.
    pub const SKEW: &str = "skew";
    /// The pen width for drawing the node.
    pub const PENWIDTH: &str = "penwidth";
    /// The sides for polygon shapes.
    pub const SIDES: &str = "sides";
    /// The rotation for the node.
    pub const ORIENTATION: &str = "orientation";
    /// The peripheries count for the node.
    pub const PERIPHERIES: &str = "peripheries";
}

/// Common GraphViz attribute names for edges.
pub mod edge {
    /// The label of the edge.
    pub const LABEL: &str = "label";
    /// The color of the edge.
    pub const COLOR: &str = "color";
    /// The style of the edge.
    pub const STYLE: &str = "style";
    /// The direction of the edge.
    pub const DIR: &str = "dir";
    /// The font name for the edge label.
    pub const FONTNAME: &str = "fontname";
    /// The font size for the edge label.
    pub const FONTSIZE: &str = "fontsize";
    /// The font color for the edge label.
    pub const FONTCOLOR: &str = "fontcolor";
    /// The weight of the edge.
    pub const WEIGHT: &str = "weight";
    /// The minimum length of the edge.
    pub const MINLEN: &str = "minlen";
    /// The URL to associate with the edge.
    pub const URL: &str = "URL";
    /// The tooltip for the edge.
    pub const TOOLTIP: &str = "tooltip";
    /// Whether to constrain the edge.
    pub const CONSTRAINT: &str = "constraint";
    /// The pen width for drawing the edge.
    pub const PENWIDTH: &str = "penwidth";
    /// Label position on the edge.
    pub const LABELANGLE: &str = "labelangle";
    /// The distance of the label from the edge.
    pub const LABELDISTANCE: &str = "labeldistance";
    /// Label position along the edge.
    pub const LABELTOOLTIP: &str = "labeltooltip";
    /// Whether the edge is decorate.
    pub const DECORATE: &str = "decorate";
    /// The tail port for the edge.
    pub const TAILPORT: &str = "tailport";
    /// The head port for the edge.
    pub const HEADPORT: &str = "headport";
    /// The arrowhead style.
    pub const ARROWHEAD: &str = "arrowhead";
    /// The arrowtail style.
    pub const ARROWTAIL: &str = "arrowtail";
    /// The position of the edge.
    pub const POS: &str = "pos";
    /// The label position of the edge.
    pub const LPOS: &str = "lp";
}

/// Common GraphViz attribute values.
pub mod values {
    /// Common values for node shape.
    pub mod shape {
        /// A box shape.
        pub const BOX: &str = "box";
        /// A circle shape.
        pub const CIRCLE: &str = "circle";
        /// An ellipse shape.
        pub const ELLIPSE: &str = "ellipse";
        /// A point shape.
        pub const POINT: &str = "point";
        /// A diamond shape.
        pub const DIAMOND: &str = "diamond";
        /// A polygon shape.
        pub const POLYGON: &str = "polygon";
        /// A record shape.
        pub const RECORD: &str = "record";
        /// A table shape.
        pub const TABLE: &str = "table";
        /// A plaintext shape.
        pub const PLAINTEXT: &str = "plaintext";
        /// A house shape.
        pub const HOUSE: &str = "house";
        /// An inverted house shape.
        pub const INVHOUSE: &str = "invhouse";
        /// A triangle shape.
        pub const TRIANGLE: &str = "triangle";
        /// An inverted triangle shape.
        pub const INVTRIANGLE: &str = "invtriangle";
        /// A hexagon shape.
        pub const HEXAGON: &str = "hexagon";
        /// An octagon shape.
        pub const OCTAGON: &str = "octagon";
        /// A doublecircle shape.
        pub const DOUBLECIRCLE: &str = "doublecircle";
        /// A doubleoctagon shape.
        pub const DOUBLEOCTAGON: &str = "doubleoctagon";
        /// A tripleoctagon shape.
        pub const TRIPLEOCTAGON: &str = "tripleoctagon";
        /// A trapezium shape.
        pub const TRAPEZIUM: &str = "trapezium";
        /// An inverted trapezium shape.
        pub const INVTRAPEZIUM: &str = "invtrapezium";
        /// A parallelogram shape.
        pub const PARALLELOGRAM: &str = "parallelogram";
        /// A folder shape.
        pub const FOLDER: &str = "folder";
        /// A box with 3D effect.
        pub const BOX3D: &str = "box3d";
        /// A component shape.
        pub const COMPONENT: &str = "component";
        /// A cylinder shape.
        pub const CYLINDER: &str = "cylinder";
        /// A note shape.
        pub const NOTE: &str = "note";
        /// A tab shape.
        pub const TAB: &str = "tab";
        /// A Minimum Description Length shape.
        pub const MDL: &str = "Mdl";
        /// A database shape.
        pub const DATABASE: &str = "database";
        /// A signature shape.
        pub const SIGNATURE: &str = "signature";
    }

    /// Common values for edge and node style.
    pub mod style {
        /// A solid style.
        pub const SOLID: &str = "solid";
        /// A dashed style.
        pub const DASHED: &str = "dashed";
        /// A dotted style.
        pub const DOTTED: &str = "dotted";
        /// A bold style.
        pub const BOLD: &str = "bold";
        /// A filled style.
        pub const FILLED: &str = "filled";
        /// A rounded style.
        pub const ROUNDED: &str = "rounded";
        /// A diagonals style.
        pub const DIAGONALS: &str = "diagonals";
        /// An invis style.
        pub const INVIS: &str = "invis";
        /// A tapered style.
        pub const TAPERED: &str = "tapered";
    }

    /// Common values for edge direction.
    pub mod dir {
        /// Forward direction.
        pub const FORWARD: &str = "forward";
        /// Backward direction.
        pub const BACK: &str = "back";
        /// Both directions.
        pub const BOTH: &str = "both";
        /// No direction.
        pub const NONE: &str = "none";
    }

    /// Common values for graph rank direction.
    pub mod rankdir {
        /// Top to bottom direction.
        pub const TB: &str = "TB";
        /// Left to right direction.
        pub const LR: &str = "LR";
        /// Bottom to top direction.
        pub const BT: &str = "BT";
        /// Right to left direction.
        pub const RL: &str = "RL";
    }

    /// Common values for edge arrowhead styles.
    pub mod arrowhead {
        /// Normal arrowhead.
        pub const NORMAL: &str = "normal";
        /// Box arrowhead.
        pub const BOX: &str = "box";
        /// Crow arrowhead.
        pub const CROW: &str = "crow";
        /// Diamond arrowhead.
        pub const DIAMOND: &str = "diamond";
        /// Dot arrowhead.
        pub const DOT: &str = "dot";
        /// Inverted arrowhead.
        pub const INV: &str = "inv";
        /// No arrowhead.
        pub const NONE: &str = "none";
        /// Tee arrowhead.
        pub const TEE: &str = "tee";
        /// Vee arrowhead.
        pub const VEE: &str = "vee";
    }

    /// Common values for graph splines setting.
    pub mod splines {
        /// True (default).
        pub const TRUE: &str = "true";
        /// False.
        pub const FALSE: &str = "false";
        /// None.
        pub const NONE: &str = "none";
        /// Line.
        pub const LINE: &str = "line";
        /// Polyline.
        pub const POLYLINE: &str = "polyline";
        /// Curved.
        pub const CURVED: &str = "curved";
        /// Orthogonal.
        pub const ORTHO: &str = "ortho";
        /// Spline.
        pub const SPLINE: &str = "spline";
    }

    /// Common values for graph overlap removal.
    pub mod overlap {
        /// True.
        pub const TRUE: &str = "true";
        /// False.
        pub const FALSE: &str = "false";
        /// Scale.
        pub const SCALE: &str = "scale";
        /// ScaleXY.
        pub const SCALEXY: &str = "scalexy";
        /// Prism.
        pub const PRISM: &str = "prism";
        /// Compress.
        pub const COMPRESS: &str = "compress";
        /// VPrism.
        pub const VPRISM: &str = "vpsc";
        /// fdp.
        pub const FDP: &str = "fdp";
    }
}
