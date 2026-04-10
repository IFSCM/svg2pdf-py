from __future__ import annotations

class FontDatabase:
    """Manages fonts used during SVG rendering."""

    def __init__(self) -> None: ...
    @staticmethod
    def system() -> FontDatabase:
        """Create a FontDatabase pre-loaded with system fonts."""
        ...
    def load_system_fonts(self) -> None:
        """Add system fonts to this database."""
        ...
    def load_font_file(self, path: str) -> None:
        """Load a single font file. Raises FileNotFoundError if not found."""
        ...
    def load_fonts_dir(self, dir: str) -> None:
        """Load all fonts from a directory."""
        ...
    def __len__(self) -> int: ...
    def __repr__(self) -> str: ...


class ConversionOptions:
    compress: bool
    raster_scale: float
    embed_text: bool

    def __init__(
        self,
        compress: bool = True,
        raster_scale: float = 1.0,
        embed_text: bool = True,
    ) -> None: ...
    def __repr__(self) -> str: ...


def svg_to_pdf(
    svg_str: str,
    font_db: FontDatabase,
    options: ConversionOptions | None = None,
) -> bytes:
    """Convert an SVG string to a standalone PDF. Returns PDF bytes."""
    ...

def svg_to_chunk(
    svg_str: str,
    font_db: FontDatabase,
    options: ConversionOptions | None = None,
) -> bytes:
    """Convert an SVG string to a raw PDF chunk (XObject) for embedding."""
    ...

def svg_pages_to_pdfs(
    svg_strings: list[str],
    font_db: FontDatabase,
    options: ConversionOptions | None = None,
) -> list[bytes]:
    """Convert a list of SVG strings to individual PDF byte strings."""
    ...