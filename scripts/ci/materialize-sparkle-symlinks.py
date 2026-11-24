#!/usr/bin/env python3
"""Replace Sparkle symlinks with real files before cmake install.

actions/cache restores framework symlinks as directories. cmake then refuses
to install the original symlink because that directory already exists.
"""
import shutil
import sys
from pathlib import Path

root = Path(sys.argv[1] if len(sys.argv) > 1 else ".tools/sparkle")
if not root.is_dir():
    sys.exit(0)
links = [path for path in root.rglob("*") if path.is_symlink()]
links.sort(key=lambda path: len(path.parts), reverse=True)
for link in links:
    target = link.resolve()
    link.unlink()
    if target.is_dir():
        shutil.copytree(target, link, symlinks=False)
    elif target.is_file():
        shutil.copy2(target, link)
