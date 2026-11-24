#!/usr/bin/env python3
"""Write OCR model completion markers missing from a downloaded build tree.

upload-artifact can omit dotfiles. snow-shot-macos-ocr.py verify reads
assets/ocr/models/<id>/.complete.json and expects {"schema": 1, "component": id}.
"""
import json
import sys
from pathlib import Path

root = Path(sys.argv[1] if len(sys.argv) > 1 else "build/snow-shot-macos-arm64-release")
written = 0
for models in root.rglob("assets/ocr/models"):
    if not models.is_dir():
        continue
    for model in models.iterdir():
        if not model.is_dir() or not any(model.iterdir()):
            continue
        marker = model / ".complete.json"
        if marker.exists():
            continue
        marker.write_text(json.dumps({"schema": 1, "component": model.name}) + "\n", encoding="utf-8")
        written += 1
        print(f"wrote {marker}")
print(f"markers_written={written}")
