#!/usr/bin/env python3
"""Russian names for the assistant's command search.

Reads AI-PowerPoint's interface catalog (crates/ui-egui/i18n/ru.tsv: `English <TAB> Russian`) and
writes studio/ru-labels.json:

    {"text": {"New Slide": "Создать слайд", ...}, "id": {}}

`text` translates command labels and ribbon-path segments; the assistant panel uses it to show
and search commands in Russian. Run by studio/build/package.sh.
"""

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def unescape(s: str) -> str:
    return s.replace("\\n", "\n").replace("\\t", "\t").replace("\\\\", "\\")


def main() -> int:
    src = ROOT / "crates" / "ui-egui" / "i18n" / "ru.tsv"
    dst = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("ru-labels.json")
    text: dict[str, str] = {}
    for line in src.read_text(encoding="utf-8").splitlines():
        if not line or line.startswith("#"):
            continue
        cols = line.split("\t")
        if len(cols) < 2 or not cols[1].strip():
            continue
        text.setdefault(unescape(cols[0]), unescape(cols[1]))
    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text(json.dumps({"text": text, "id": {}}, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    print(f"{dst}: {len(text)} labels")
    return 0


if __name__ == "__main__":
    sys.exit(main())
