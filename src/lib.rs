use pyo3::prelude::*;
use pyo3::types::PyBytes;
use pyo3::exceptions::{PyValueError, PyRuntimeError, PyFileNotFoundError};
use std::path::Path;
use std::sync::{Arc, Mutex};
use svg2pdf::usvg;

#[pyclass(frozen)]
#[derive(Clone)]
struct FontDatabase {
    inner: Arc<Mutex<usvg::fontdb::Database>>,
}

impl FontDatabase {
    fn make_options(&self) -> usvg::Options<'_> {
        let mut opts = usvg::Options::default();
        opts.fontdb = Arc::new(self.inner.lock().unwrap().clone());
        opts
    }
}

#[pymethods]
impl FontDatabase {
    #[new]
    fn new() -> Self {
        FontDatabase {
            inner: Arc::new(Mutex::new(usvg::fontdb::Database::new())),
        }
    }

    #[staticmethod]
    fn system() -> Self {
        let db = FontDatabase::new();
        db.load_system_fonts();
        db
    }

    fn load_system_fonts(&self) {
        self.inner.lock().unwrap().load_system_fonts();
    }

    fn load_font_file(&self, path: &str) -> PyResult<()> {
        if !Path::new(path).exists() {
            return Err(PyFileNotFoundError::new_err(
                format!("Font file not found: {path}")
            ));
        }
        self.inner
            .lock()
            .unwrap()
            .load_font_file(path)
            .map_err(|e| PyRuntimeError::new_err(
                format!("Failed to load font file '{path}': {e}")
            ))
    }

    fn load_fonts_dir(&self, dir: &str) {
        self.inner.lock().unwrap().load_fonts_dir(dir);
    }

    fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }

    fn __repr__(&self) -> String {
        format!("FontDatabase(faces={})", self.len())
    }
}

#[pyclass]
#[derive(Clone)]
struct ConversionOptions {
    #[pyo3(get, set)]
    compress: bool,
    #[pyo3(get, set)]
    raster_scale: f32,
    #[pyo3(get, set)]
    embed_text: bool,
}

#[pymethods]
impl ConversionOptions {
    #[new]
    #[pyo3(signature = (compress=true, raster_scale=1.0, embed_text=true))]
    fn new(compress: bool, raster_scale: f32, embed_text: bool) -> Self {
        ConversionOptions { compress, raster_scale, embed_text }
    }

    fn __repr__(&self) -> String {
        format!(
            "ConversionOptions(compress={}, raster_scale={}, embed_text={})",
            self.compress, self.raster_scale, self.embed_text
        )
    }
}

fn build_conv_options(options: Option<&ConversionOptions>) -> svg2pdf::ConversionOptions {
    let mut opts = svg2pdf::ConversionOptions::default();
    if let Some(o) = options {
        opts.compress = o.compress;
        opts.raster_scale = o.raster_scale;
        opts.embed_text = o.embed_text;
    }
    opts
}

fn parse_tree(svg_str: &str, font_db: &FontDatabase) -> PyResult<usvg::Tree> {
    let opts = font_db.make_options();
    usvg::Tree::from_str(svg_str, &opts)
        .map_err(|e| PyValueError::new_err(format!("SVG parse error: {e}")))
}

#[pyfunction]
#[pyo3(signature = (svg_str, font_db, options=None))]
fn svg_to_pdf<'py>(
    py: Python<'py>,
    svg_str: &str,
    font_db: &FontDatabase,
    options: Option<&ConversionOptions>,
) -> PyResult<Bound<'py, PyBytes>> {
    let tree = parse_tree(svg_str, font_db)?;
    let pdf = svg2pdf::to_pdf(&tree, build_conv_options(options), svg2pdf::PageOptions::default())
        .map_err(|e| PyRuntimeError::new_err(format!("PDF conversion error: {e}")))?;
    Ok(PyBytes::new_bound(py, &pdf))
}

#[pyfunction]
#[pyo3(signature = (svg_str, font_db, options=None))]
fn svg_to_chunk<'py>(
    py: Python<'py>,
    svg_str: &str,
    font_db: &FontDatabase,
    options: Option<&ConversionOptions>,
) -> PyResult<Bound<'py, PyBytes>> {
    let tree = parse_tree(svg_str, font_db)?;
    let (chunk, _id) = svg2pdf::to_chunk(&tree, build_conv_options(options))
        .map_err(|e| PyRuntimeError::new_err(format!("Chunk conversion error: {e}")))?;
    Ok(PyBytes::new_bound(py, chunk.as_bytes()))
}

#[pyfunction]
#[pyo3(signature = (svg_strings, font_db, options=None))]
fn svg_pages_to_pdfs<'py>(
    py: Python<'py>,
    svg_strings: Vec<String>,
    font_db: &FontDatabase,
    options: Option<&ConversionOptions>,
) -> PyResult<Vec<Bound<'py, PyBytes>>> {
    let conv_opts = build_conv_options(options);
    let opts = font_db.make_options();

    svg_strings
        .iter()
        .enumerate()
        .map(|(i, svg_str)| {
            let tree = usvg::Tree::from_str(svg_str, &opts)
                .map_err(|e| PyValueError::new_err(
                    format!("SVG parse error on page {i}: {e}")
                ))?;
            let pdf = svg2pdf::to_pdf(&tree, conv_opts.clone(), svg2pdf::PageOptions::default())
                .map_err(|e| PyRuntimeError::new_err(
                    format!("PDF conversion error on page {i}: {e}")
                ))?;
            Ok(PyBytes::new_bound(py, &pdf))
        })
        .collect()
}

#[pymodule]
fn svg2pdf_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FontDatabase>()?;
    m.add_class::<ConversionOptions>()?;
    m.add_function(wrap_pyfunction!(svg_to_pdf, m)?)?;
    m.add_function(wrap_pyfunction!(svg_to_chunk, m)?)?;
    m.add_function(wrap_pyfunction!(svg_pages_to_pdfs, m)?)?;
    Ok(())
}