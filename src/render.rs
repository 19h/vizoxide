//! Rendering capabilities for GraphViz.
//!
//! This module provides functions for rendering GraphViz graphs to various formats.

use std::ffi::CString;
use std::path::Path;
use std::io::Write;
use std::slice;
use std::str;

use base64::Engine;
use graphviz_sys as sys;
use crate::error::GraphvizError;
use crate::graph::Graph;
use crate::layout::Context;

/// A GraphViz output format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// Portable Network Graphics format.
    Png,
    /// Scalable Vector Graphics format.
    Svg,
    /// Portable Document Format.
    Pdf,
    /// PostScript format.
    Ps,
    /// Encapsulated PostScript format.
    Eps,
    /// Graphics Interchange Format.
    Gif,
    /// JPEG format.
    Jpeg,
    /// JSON format.
    Json,
    /// DOT format (GraphViz's native format).
    Dot,
    /// Extended DOT format.
    Xdot,
    /// Plain text format.
    Plain,
    /// Canonical DOT format.
    Canon,
    /// Fig format.
    Fig,
    /// VRML format.
    Vrml,
    /// CMAPx format (client-side image map).
    Cmapx,
    /// IMAP format (server-side image map).
    Imap,
    /// BMP format.
    Bmp,
    /// SVG with embedded XHTML format.
    Svgz,
}

impl Format {
    /// Converts the format to a C string representation.
    ///
    /// # Returns
    ///
    /// A Result containing the C string or an error
    pub(crate) fn as_cstr(&self) -> Result<CString, GraphvizError> {
        let name = match self {
            Format::Png => "png",
            Format::Svg => "svg",
            Format::Pdf => "pdf",
            Format::Ps => "ps",
            Format::Eps => "eps",
            Format::Gif => "gif",
            Format::Jpeg => "jpeg",
            Format::Json => "json",
            Format::Dot => "dot",
            Format::Xdot => "xdot",
            Format::Plain => "plain",
            Format::Canon => "canon",
            Format::Fig => "fig",
            Format::Vrml => "vrml",
            Format::Cmapx => "cmapx",
            Format::Imap => "imap",
            Format::Bmp => "bmp",
            Format::Svgz => "svgz",
        };
        
        CString::new(name).map_err(|_| GraphvizError::InvalidFormat)
    }
    
    /// Checks if the format is binary.
    ///
    /// # Returns
    ///
    /// true if the format is binary, false if it's text-based
    pub fn is_binary(&self) -> bool {
        match self {
            Format::Png | Format::Gif | Format::Jpeg | Format::Pdf |
            Format::Bmp | Format::Svgz => true,
            Format::Svg | Format::Dot | Format::Xdot | Format::Plain |
            Format::Canon | Format::Json | Format::Ps | Format::Eps |
            Format::Fig | Format::Vrml | Format::Cmapx | Format::Imap => false,
        }
    }
    
    /// Returns an iterator over all available output formats.
    ///
    /// # Returns
    ///
    /// An iterator that yields all available output formats
    pub fn all() -> impl Iterator<Item = Format> {
        [
            Format::Png,
            Format::Svg,
            Format::Pdf,
            Format::Ps,
            Format::Eps,
            Format::Gif,
            Format::Jpeg,
            Format::Json,
            Format::Dot,
            Format::Xdot,
            Format::Plain,
            Format::Canon,
            Format::Fig,
            Format::Vrml,
            Format::Cmapx,
            Format::Imap,
            Format::Bmp,
            Format::Svgz,
        ].iter().copied()
    }
    
    /// Gets the MIME type for the format.
    ///
    /// # Returns
    ///
    /// The MIME type as a string
    pub fn mime_type(&self) -> &'static str {
        match self {
            Format::Png => "image/png",
            Format::Svg => "image/svg+xml",
            Format::Pdf => "application/pdf",
            Format::Ps => "application/postscript",
            Format::Eps => "application/postscript",
            Format::Gif => "image/gif",
            Format::Jpeg => "image/jpeg",
            Format::Json => "application/json",
            Format::Dot => "text/vnd.graphviz",
            Format::Xdot => "text/vnd.graphviz",
            Format::Plain => "text/plain",
            Format::Canon => "text/vnd.graphviz",
            Format::Fig => "image/x-xfig",
            Format::Vrml => "model/vrml",
            Format::Cmapx => "text/html",
            Format::Imap => "application/x-httpd-imap",
            Format::Bmp => "image/bmp",
            Format::Svgz => "image/svg+xml",
        }
    }
    
    /// Gets the file extension for the format.
    ///
    /// # Returns
    ///
    /// The file extension as a string
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Png => "png",
            Format::Svg => "svg",
            Format::Pdf => "pdf",
            Format::Ps => "ps",
            Format::Eps => "eps",
            Format::Gif => "gif",
            Format::Jpeg => "jpg",
            Format::Json => "json",
            Format::Dot => "dot",
            Format::Xdot => "xdot",
            Format::Plain => "txt",
            Format::Canon => "dot",
            Format::Fig => "fig",
            Format::Vrml => "wrl",
            Format::Cmapx => "map",
            Format::Imap => "map",
            Format::Bmp => "bmp",
            Format::Svgz => "svgz",
        }
    }
}

/// Renders a graph to a file with the specified format.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to render
/// * `format` - The output format
/// * `path` - The output file path
///
/// # Returns
///
/// A Result indicating success or failure
pub fn render_to_file<P: AsRef<Path>>(
    context: &Context,
    graph: &Graph,
    format: Format,
    path: P,
) -> Result<(), GraphvizError> {
    let format_cstr = format.as_cstr()?;
    let path_str = path.as_ref().to_string_lossy();
    let path_cstr = CString::new(path_str.as_bytes())?;
    
    let result = unsafe {
        sys::gvRenderFilename(
            context.inner,
            graph.inner,
            format_cstr.as_ptr(),
            path_cstr.as_ptr(),
        )
    };
    
    if result == 0 {
        Ok(())
    } else {
        Err(GraphvizError::RenderFailed)
    }
}

/// Renders a graph to a string with the specified format.
///
/// For binary formats, the result is base64-encoded.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to render
/// * `format` - The output format
///
/// # Returns
///
/// A Result containing the rendered string or an error
pub fn render_to_string(
    context: &Context,
    graph: &Graph,
    format: Format,
) -> Result<String, GraphvizError> {
    // Convert format to C string representation
    let format_cstr = format.as_cstr()?;
    
    // Prepare pointers to receive rendered data and length
    let mut buffer_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
    let mut length: std::os::raw::c_uint = 0;
    
    // Call GraphViz rendering function to generate in-memory representation
    let result = unsafe {
        sys::gvRenderData(
            context.inner,
            graph.inner,
            format_cstr.as_ptr(),
            &mut buffer_ptr,
            &mut length,
        )
    };
    
    // Validate rendering operation completed successfully
    if result != 0 {
        return Err(GraphvizError::RenderFailed);
    }
    
    // Ensure buffer was allocated properly
    if buffer_ptr.is_null() {
        return Err(GraphvizError::NullPointer("Render buffer is null"));
    }
    
    // Convert data to Rust string, handling different formats appropriately
    let rendered_string = if format.is_binary() {
        // For binary formats, encode as base64
        let data_slice = unsafe { 
            slice::from_raw_parts(buffer_ptr as *const u8, length as usize) 
        };
        base64::engine::general_purpose::STANDARD.encode(data_slice)
    } else {
        // For text formats, convert directly to UTF-8 string
        let data_slice = unsafe { 
            slice::from_raw_parts(buffer_ptr as *const u8, length as usize) 
        };
        match str::from_utf8(data_slice) {
            Ok(s) => s.to_owned(),
            Err(_) => {
                // Clean up memory before returning error
                unsafe { sys::gvFreeRenderData(buffer_ptr) };
                return Err(GraphvizError::InvalidUtf8);
            }
        }
    };
    
    // Release memory allocated by GraphViz
    unsafe { sys::gvFreeRenderData(buffer_ptr) };
    
    Ok(rendered_string)
}

/// Renders a graph to a byte vector with the specified format.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to render
/// * `format` - The output format
///
/// # Returns
///
/// A Result containing the rendered bytes or an error
pub fn render_to_bytes(
    context: &Context,
    graph: &Graph,
    format: Format,
) -> Result<Vec<u8>, GraphvizError> {
    // Convert format to C string representation
    let format_cstr = format.as_cstr()?;
    
    // Prepare pointers to receive rendered data and length
    let mut buffer_ptr: *mut std::os::raw::c_char = std::ptr::null_mut();
    let mut length: std::os::raw::c_uint = 0;
    
    // Call GraphViz rendering function to generate in-memory representation
    let result = unsafe {
        sys::gvRenderData(
            context.inner,
            graph.inner,
            format_cstr.as_ptr(),
            &mut buffer_ptr,
            &mut length,
        )
    };
    
    // Validate rendering operation completed successfully
    if result != 0 {
        return Err(GraphvizError::RenderFailed);
    }
    
    // Ensure buffer was allocated properly
    if buffer_ptr.is_null() {
        return Err(GraphvizError::NullPointer("Render buffer is null"));
    }
    
    // Copy data into a Vec<u8>
    let data_slice = unsafe { 
        slice::from_raw_parts(buffer_ptr as *const u8, length as usize) 
    };
    let bytes = data_slice.to_vec();
    
    // Release memory allocated by GraphViz
    unsafe { sys::gvFreeRenderData(buffer_ptr) };
    
    Ok(bytes)
}

/// Renders a graph to a writer with the specified format.
///
/// # Arguments
///
/// * `context` - The GraphViz context
/// * `graph` - The graph to render
/// * `format` - The output format
/// * `writer` - The writer to write to
///
/// # Returns
///
/// A Result indicating success or failure
pub fn render_to_writer<W: Write>(
    context: &Context,
    graph: &Graph,
    format: Format,
    mut writer: W,
) -> Result<(), GraphvizError> {
    let bytes = render_to_bytes(context, graph, format)?;
    writer.write_all(&bytes)?;
    Ok(())
}

/// Options for rendering graphs.
pub struct RenderOptions {
    /// Whether to render with anti-aliasing.
    pub anti_alias: bool,
    /// Whether to render with transparency.
    pub transparent: bool,
    /// Resolution in DPI.
    pub dpi: Option<f64>,
    /// Background color.
    pub background: Option<String>,
    /// Whether to include the graph's bounding box.
    pub show_bb: bool,
    /// Scale factor for rendering.
    pub scale: Option<f64>,
    /// Fit to specific dimensions.
    pub size: Option<(f64, f64)>,
    /// Output quality (0-100) for formats like JPEG.
    pub quality: Option<u32>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        RenderOptions {
            anti_alias: true,
            transparent: false,
            dpi: None,
            background: None,
            show_bb: false,
            scale: None,
            size: None,
            quality: None,
        }
    }
}

impl RenderOptions {
    /// Creates a new RenderOptions with default values.
    ///
    /// # Returns
    ///
    /// A new RenderOptions instance
    pub fn new() -> Self {
        Default::default()
    }
    
    /// Applies the options to a graph.
    ///
    /// # Arguments
    ///
    /// * `graph` - The graph to apply options to
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure
    pub fn apply(&self, graph: &Graph) -> Result<(), GraphvizError> {
        if !self.anti_alias {
            graph.set_attribute("smoothing", "0")?;
        }
        
        if self.transparent {
            graph.set_attribute("bgcolor", "transparent")?;
        }
        
        if let Some(dpi) = self.dpi {
            graph.set_attribute("dpi", &dpi.to_string())?;
        }
        
        if let Some(ref background) = self.background {
            graph.set_attribute("bgcolor", background)?;
        }
        
        if self.show_bb {
            graph.set_attribute("bb", "true")?;
        }
        
        if let Some(scale) = self.scale {
            graph.set_attribute("scale", &scale.to_string())?;
        }
        
        if let Some((width, height)) = self.size {
            graph.set_attribute("size", &format!("{},{}!", width, height))?;
        }
        
        if let Some(quality) = self.quality {
            graph.set_attribute("quality", &quality.to_string())?;
        }
        
        Ok(())
    }
    
    /// Sets whether to render with anti-aliasing.
    ///
    /// # Arguments
    ///
    /// * `anti_alias` - Whether to use anti-aliasing
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_anti_alias(mut self, anti_alias: bool) -> Self {
        self.anti_alias = anti_alias;
        self
    }
    
    /// Sets whether to render with transparency.
    ///
    /// # Arguments
    ///
    /// * `transparent` - Whether to use transparency
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_transparency(mut self, transparent: bool) -> Self {
        self.transparent = transparent;
        self
    }
    
    /// Sets the resolution in DPI.
    ///
    /// # Arguments
    ///
    /// * `dpi` - The resolution value
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_dpi(mut self, dpi: f64) -> Self {
        self.dpi = Some(dpi);
        self
    }
    
    /// Sets the background color.
    ///
    /// # Arguments
    ///
    /// * `color` - The background color
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_background(mut self, color: &str) -> Self {
        self.background = Some(color.to_owned());
        self
    }
    
    /// Sets whether to show the bounding box.
    ///
    /// # Arguments
    ///
    /// * `show_bb` - Whether to show the bounding box
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_show_bb(mut self, show_bb: bool) -> Self {
        self.show_bb = show_bb;
        self
    }
    
    /// Sets the scale factor.
    ///
    /// # Arguments
    ///
    /// * `scale` - The scale factor
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_scale(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }
    
    /// Sets the output size.
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
    
    /// Sets the output quality.
    ///
    /// # Arguments
    ///
    /// * `quality` - The quality (0-100)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn with_quality(mut self, quality: u32) -> Self {
        self.quality = Some(quality);
        self
    }
}
