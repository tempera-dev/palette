"""Framework integrations.

These adapters bridge agent frameworks into Palette spans. They are part of the
SDK's public surface, mirroring the TypeScript SDK which exports the same
adapters from its package root::

    from palette import PaletteCallbackHandler        # LangChain
    from palette import PaletteLlamaIndexHandler       # LlamaIndex

Both adapters import their framework lazily behind an import guard, so importing
this module never requires LangChain or LlamaIndex to be installed.
"""

from __future__ import annotations

from .langchain import PaletteCallbackHandler
from .llamaindex import PaletteLlamaIndexHandler

__all__ = [
    "PaletteCallbackHandler",
    "PaletteLlamaIndexHandler",
]
