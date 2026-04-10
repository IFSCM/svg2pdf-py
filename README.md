# svg2pdf-py

Fast, accurate SVG → PDF conversion for Python, powered by the [`svg2pdf`](https://github.com/typst/svg2pdf) Rust crate from [Typst](https://typst.app). Built with [PyO3](https://github.com/PyO3/pyo3) and [Maturin](https://github.com/PyO3/maturin).

Unlike rasterization-based approaches, `svg2pdf-py` produces **true vector PDFs** — text stays selectable, paths stay sharp at any scale, and custom fonts are fully embedded.

---

## Features

- Vector-accurate SVG → PDF (no rasterization)
- Custom font support via `usvg` / `fontdb`
- Batch conversion for multi-page PDFs
- Exposes raw PDF chunks for embedding into existing documents
- Pre-built wheels for Windows, Linux, and macOS (Python ≥ 3.9)

---

## Installation

```bash
pip install svg2pdf-py
```

---

## Quick Start

```python
import svg2pdf_py

# Load custom fonts
db = svg2pdf_py.FontDatabase()
db.load_font_file("/path/to/MyFont.ttf")
db.load_font_file("/path/to/MyFont-Bold.ttf")

# Convert SVG string → PDF bytes
svg_str = open("diagram.svg", encoding="utf-8").read()
pdf_bytes = svg2pdf_py.svg_to_pdf(svg_str, db)

open("diagram.pdf", "wb").write(pdf_bytes)
```

---

## API

### `FontDatabase`

Manages fonts used during SVG rendering.

```python
# Empty database
db = svg2pdf_py.FontDatabase()

# Pre-loaded with system fonts
db = svg2pdf_py.FontDatabase.system()

db.load_font_file("/path/to/font.ttf")   # load a single font file
db.load_fonts_dir("/path/to/fonts/")     # load all fonts in a directory
db.load_system_fonts()                   # add system fonts to existing db
len(db)                                  # number of loaded font faces
```

---

### `ConversionOptions`

Controls PDF output quality.

```python
opts = svg2pdf_py.ConversionOptions(
    compress=True,      # deflate-compress streams (default: True)
    raster_scale=1.0,   # scale factor for rasterized elements (default: 1.0)
    embed_text=True,    # embed text as real PDF text, not outlines (default: True)
)
```

---

### `svg_to_pdf`

Convert a single SVG string to a standalone PDF.

```python
pdf_bytes: bytes = svg2pdf_py.svg_to_pdf(
    svg_str,   # str — SVG content
    db,        # FontDatabase
    options,   # ConversionOptions (optional)
)
```

---

### `svg_to_chunk`

Convert a single SVG string to a raw PDF chunk (XObject), suitable for embedding into an existing PDF document.

```python
chunk_bytes: bytes = svg2pdf_py.svg_to_chunk(
    svg_str,
    db,
    options,   # optional
)
```

---

### `svg_pages_to_pdfs`

Convert a list of SVG strings to individual PDF byte strings — one per page.

```python
pages: list[bytes] = svg2pdf_py.svg_pages_to_pdfs(
    svg_strings,   # list[str]
    db,
    options,       # optional
)
```

---

## Multi-Page Example

Combine multiple SVGs into a single PDF using [PyMuPDF](https://pymupdf.readthedocs.io/):

```python
import svg2pdf_py
import pymupdf

db = svg2pdf_py.FontDatabase()
db.load_fonts_dir("/path/to/fonts/")

svg_pages = [open(f"page{i}.svg").read() for i in range(1, 5)]
pdf_pages = svg2pdf_py.svg_pages_to_pdfs(svg_pages, db)

doc = pymupdf.open()
for pdf_bytes in pdf_pages:
    part = pymupdf.open("pdf", pdf_bytes)
    doc.insert_pdf(part)
    part.close()

doc.save("output.pdf", garbage=4, deflate=True)
doc.close()
```

---

## Credits

This package is a thin PyO3 binding over the excellent [`svg2pdf`](https://github.com/typst/svg2pdf) crate by [Typst](https://typst.app), which in turn uses [`usvg`](https://github.com/linebender/resvg) for SVG parsing and [`fontdb`](https://github.com/RazrFalcon/fontdb) for font resolution.

---

## License

MIT