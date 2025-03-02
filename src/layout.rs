//! Layout algorithms and configurations for GraphViz.
//!
//! This module provides functions for applying layouts to graphs using
//! various GraphViz layout engines.

use std::ffi::CString;
use std::ptr;

use graphviz_sys as sys;
use crate::error::GraphvizError;
use crate::graph::Graph;

/// A GraphViz layout engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Engine {
    /// Hierarchical layout (default).
    Dot,
    /// Spring model layout.
    Neato,
    /// Force-directed layout.
    Fdp,
    /// Multiscale version of FDP.
    Sfdp,
    /// Radial layout.
    Twopi,
    /// Circular layout.
    Circo,
    /// Energy minimization layout.
    Osage,
    /// Patchwork tree layout.
    Patchwork,
}

impl Engine {
    /// Converts the engine to a C string representation.
    ///
    /// # Returns
    ///
    /// A Result containing the C string or an error
    pub(crate) fn as_cstr(&self) -> Result<CString, GraphvizError> {
        let name = match self {
            Engine::Dot => "dot",
            Engine::Neato => "neato",
            Engine::Fdp => "fdp",
            Engine::Sfdp => "sfdp",
            Engine::Twopi => "twopi",
            Engine::Circo => "circo",
            Engine::Osage => "osage",
            Engine::Patchwork => "patchwork",
        };
        
        CString::new(name).map_err(|_| GraphvizError::InvalidEngine)
    }
    
    /// Returns an iterator over all available layout engines.
    ///
    /// # Returns
    ///
    /// An iterator that yields all available layout engines
    pub fn all() -> impl Iterator<Item = Engine> {
        [
            Engine::Dot,
            Engine::Neato,
            Engine::Fdp,
            Engine::Sfdp,
            Engine::Twopi,
            Engine::Circo,
            Engine::Osage,
            Engine::Patchwork,
        ].iter().copied()
    }
}

/// A GraphViz context for layout and rendering operations.
pub struct Context {
    /// Pointer to the underlying GVC_t structure
    pub(crate) inner: *mut sys::GVC_t,
}

impl Context {
    /// Creates a new GraphViz context.
    ///
    /// # Returns
    ///
    /// A Result containing the new Context or an error
    pub fn new() -> Result<Self, GraphvizError> {
        let inner = unsafe { sys::gvContext() };
        
        if inner.is_null() {
            return Err(GraphvizError::ContextCreationFailed);
        }
        
        Ok(Context { inner })
    }
    
    /// Creates a new GraphViz context with custom plugins.
    ///
    /// # Arguments
    ///
    /// * `builtins` - Whether to include built-in plugins
    /// * `demand_loading` - Whether to load plugins on demand
    ///
    /// # Returns
    ///
    /// A Result containing the new Context or an error
    pub fn new_with_plugins(builtins: bool, demand_loading: bool) -> Result<Self, GraphvizError> {
        let builtins_ptr = if builtins {
            &raw const sys::lt_preloaded_symbols as *const _
        } else {
            ptr::null()
        };
        
        let inner = unsafe { 
            sys::gvContextPlugins(builtins_ptr, if demand_loading { 1 } else { 0 }) 
        };
        
        if inner.is_null() {
            return Err(GraphvizError::ContextCreationFailed);
        }
        
        Ok(Context { inner })
    }
}

// RAII implementation for Context
impl Drop for Context {
    fn drop(&mut self) {
        if !self.inner.is_null() {
            unsafe { sys::gvFreeContext(self.inner) };
        }
    }
}

/// Applies a layout to a graph using the specified engine.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to layout
/// * `engine` - The layout engine to use
///
/// # Returns
///
/// A Result indicating success or failure
pub fn apply_layout(
    context: &Context, 
    graph: &mut Graph, 
    engine: Engine
) -> Result<(), GraphvizError> {
    let engine_cstr = engine.as_cstr()?;
    
    let result = unsafe { 
        sys::gvLayout(context.inner, graph.inner, engine_cstr.as_ptr()) 
    };
    
    if result == 0 {
        Ok(())
    } else {
        Err(GraphvizError::LayoutFailed)
    }
}

/// Frees the layout resources associated with a graph.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to free layout resources for
///
/// # Returns
///
/// A Result indicating success or failure
pub fn free_layout(
    context: &Context,
    graph: &mut Graph,
) -> Result<(), GraphvizError> {
    let result = unsafe { 
        sys::gvFreeLayout(context.inner, graph.inner) 
    };
    
    if result == 0 {
        Ok(())
    } else {
        Err(GraphvizError::FreeLayoutFailed)
    }
}

/// Layout settings for configuring layout algorithms.
pub struct LayoutSettings {
    /// Size of the output (in inches).
    pub size: Option<(f64, f64)>,
    /// Ratio of height/width.
    pub ratio: Option<f64>,
    /// Direction of layout.
    pub rankdir: Option<String>,
    /// Overlap removal strategy.
    pub overlap: Option<String>,
    /// Separation between nodes.
    pub nodesep: Option<f64>,
    /// Separation between ranks.
    pub ranksep: Option<f64>,
    /// Spline configuration.
    pub splines: Option<String>,
    /// Margin around the layout.
    pub margin: Option<(f64, f64)>,
    /// Graph label.
    pub label: Option<String>,
    /// Font name.
    pub fontname: Option<String>,
    /// Font size.
    pub fontsize: Option<f64>,
    /// Minimum edge length.
    pub minlen: Option<i32>,
    /// Orientation (in degrees).
    pub orientation: Option<f64>,
    /// Edge concentration.
    pub concentrate: Option<bool>,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        LayoutSettings {
            size: None,
            ratio: None,
            rankdir: None,
            overlap: None,
            nodesep: None,
            ranksep: None,
            splines: None,
            margin: None,
            label: None,
            fontname: None,
            fontsize: None,
            minlen: None,
            orientation: None,
            concentrate: None,
        }
    }
}

impl LayoutSettings {
    /// Creates a new LayoutSettings with default values.
    ///
    /// # Returns
    ///
    /// A new LayoutSettings instance
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Applies the settings to a graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to apply settings to
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub fn apply(&self, graph: &Graph) -> Result<(), GraphvizError> {
        if let Some((width, height)) = self.size {
            graph.set_attribute("size", &format!("{},{}!", width, height))?;
        }
        
        if let Some(ratio) = self.ratio {
            graph.set_attribute("ratio", &ratio.to_string())?;
        }
        
        if let Some(ref rankdir) = self.rankdir {
            graph.set_attribute("rankdir", rankdir)?;
        }
        
        if let Some(ref overlap) = self.overlap {
            graph.set_attribute("overlap", overlap)?;
        }
        
        if let Some(nodesep) = self.nodesep {
            graph.set_attribute("nodesep", &nodesep.to_string())?;
        }
        
        if let Some(ranksep) = self.ranksep {
            graph.set_attribute("ranksep", &ranksep.to_string())?;
        }
        
        if let Some(ref splines) = self.splines {
            graph.set_attribute("splines", splines)?;
        }
        
        if let Some((x, y)) = self.margin {
            graph.set_attribute("margin", &format!("{},{}", x, y))?;
        }
        
        if let Some(ref label) = self.label {
            graph.set_attribute("label", label)?;
        }
        
        if let Some(ref fontname) = self.fontname {
            graph.set_attribute("fontname", fontname)?;
        }
        
        if let Some(fontsize) = self.fontsize {
            graph.set_attribute("fontsize", &fontsize.to_string())?;
        }
        
        if let Some(minlen) = self.minlen {
            graph.set_attribute("minlen", &minlen.to_string())?;
        }
        
        if let Some(orientation) = self.orientation {
            graph.set_attribute("orientation", &orientation.to_string())?;
        }
        
        if let Some(concentrate) = self.concentrate {
            graph.set_attribute("concentrate", if concentrate { "true" } else { "false" })?;
        }
        
        Ok(())
    }
    
    /// Sets the size of the output.
    ///
    /// # Arguments
    ///
    /// * `width` - The width in inches
    /// * `height` - The height in inches
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_size(mut self, width: f64, height: f64) -> Self {
        self.size = Some((width, height));
        self
    }
    
    /// Sets the ratio of height/width.
    ///
    /// # Arguments
    ///
    /// * `ratio` - The ratio value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_ratio(mut self, ratio: f64) -> Self {
        self.ratio = Some(ratio);
        self
    }
    
    /// Sets the direction of layout.
    ///
    /// # Arguments
    ///
    /// * `rankdir` - The direction (e.g., "TB", "LR", "BT", "RL")
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_rankdir(mut self, rankdir: &str) -> Self {
        self.rankdir = Some(rankdir.to_owned());
        self
    }
    
    /// Sets the overlap removal strategy.
    ///
    /// # Arguments
    ///
    /// * `overlap` - The strategy (e.g., "false", "scale", "prism")
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_overlap(mut self, overlap: &str) -> Self {
        self.overlap = Some(overlap.to_owned());
        self
    }
    
    /// Sets the node separation.
    ///
    /// # Arguments
    ///
    /// * `nodesep` - The separation value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_nodesep(mut self, nodesep: f64) -> Self {
        self.nodesep = Some(nodesep);
        self
    }
    
    /// Sets the rank separation.
    ///
    /// # Arguments
    ///
    /// * `ranksep` - The separation value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_ranksep(mut self, ranksep: f64) -> Self {
        self.ranksep = Some(ranksep);
        self
    }
    
    /// Sets the spline configuration.
    ///
    /// # Arguments
    ///
    /// * `splines` - The spline configuration (e.g., "line", "polyline", "ortho")
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_splines(mut self, splines: &str) -> Self {
        self.splines = Some(splines.to_owned());
        self
    }
    
    /// Sets the margin around the layout.
    ///
    /// # Arguments
    ///
    /// * `x` - The x margin
    /// * `y` - The y margin
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_margin(mut self, x: f64, y: f64) -> Self {
        self.margin = Some((x, y));
        self
    }
    
    /// Sets the graph label.
    ///
    /// # Arguments
    ///
    /// * `label` - The label text
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_owned());
        self
    }
    
    /// Sets the font name.
    ///
    /// # Arguments
    ///
    /// * `fontname` - The font name
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_fontname(mut self, fontname: &str) -> Self {
        self.fontname = Some(fontname.to_owned());
        self
    }
    
    /// Sets the font size.
    ///
    /// # Arguments
    ///
    /// * `fontsize` - The font size
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_fontsize(mut self, fontsize: f64) -> Self {
        self.fontsize = Some(fontsize);
        self
    }
    
    /// Sets the minimum edge length.
    ///
    /// # Arguments
    ///
    /// * `minlen` - The minimum length
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_minlen(mut self, minlen: i32) -> Self {
        self.minlen = Some(minlen);
        self
    }
    
    /// Sets the orientation (in degrees).
    ///
    /// # Arguments
    ///
    /// * `orientation` - The orientation value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_orientation(mut self, orientation: f64) -> Self {
        self.orientation = Some(orientation);
        self
    }
    
    /// Sets whether to concentrate edges.
    ///
    /// # Arguments
    ///
    /// * `concentrate` - Whether to concentrate edges
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_concentrate(mut self, concentrate: bool) -> Self {
        self.concentrate = Some(concentrate);
        self
    }
}

/// Creates a predefined set of layout settings for a hierarchical layout.
///
/// # Returns
///
/// A LayoutSettings instance configured for hierarchical layout
pub fn hierarchical_layout() -> LayoutSettings {
    LayoutSettings::new()
        .with_rankdir("TB")
        .with_splines("spline")
        .with_nodesep(0.5)
        .with_ranksep(0.5)
}

/// Creates a predefined set of layout settings for a left-to-right layout.
///
/// # Returns
///
/// A LayoutSettings instance configured for left-to-right layout
pub fn left_to_right_layout() -> LayoutSettings {
    LayoutSettings::new()
        .with_rankdir("LR")
        .with_splines("spline")
        .with_nodesep(0.5)
        .with_ranksep(0.5)
}

/// Creates a predefined set of layout settings for a radial layout.
///
/// # Returns
///
/// A LayoutSettings instance configured for radial layout
pub fn radial_layout() -> LayoutSettings {
    LayoutSettings::new()
        .with_overlap("false")
        .with_splines("spline")
}

/// Creates a predefined set of layout settings for a force-directed layout.
///
/// # Returns
///
/// A LayoutSettings instance configured for force-directed layout
pub fn force_directed_layout() -> LayoutSettings {
    LayoutSettings::new()
        .with_overlap("prism")
        .with_splines("spline")
}

/// Creates a predefined set of layout settings for a circular layout.
///
/// # Returns
///
/// A LayoutSettings instance configured for circular layout
pub fn circular_layout() -> LayoutSettings {
    LayoutSettings::new()
        .with_overlap("false")
        .with_splines("spline")
}
