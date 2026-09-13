# pordo

Ein kleines Kommandozeilen-Tool, das eine PDF-Datei nimmt und sie seitenweise in einzelne PDF-Dateien aufteilt.

## Funktionsweise

Für jede Seite der Eingabe-PDF erzeugt `pordo` im aktuellen Arbeitsverzeichnis eine eigene Datei `page_<N>.pdf`, die nur diese eine Seite enthält. Alle anderen Seiten werden aus einer Kopie des Dokuments entfernt und nicht mehr benötigte Objekte werden anschließend bereinigt (`prune_objects`).

## Installation

Voraussetzung ist ein installiertes Rust-Toolchain (inkl. `cargo`).

```sh
cargo build --release
```

Das fertige Binary liegt danach unter `target/release/pordo` (bzw. `pordo.exe` unter Windows).

## Verwendung

```sh
pordo <FILE>
```

- `<FILE>`: Pfad zur PDF-Datei, die aufgeteilt werden soll.

### Beispiel

```sh
pordo dokument.pdf
```

Ausgabe:

```
File dokument.pdf has 3 pages
Page 1
Page 2
Page 3
```

Im aktuellen Verzeichnis liegen danach `page_1.pdf`, `page_2.pdf` und `page_3.pdf`.

## Verwendete Crates

- [clap](https://crates.io/crates/clap) – Parsen der Kommandozeilenargumente
- [lopdf](https://crates.io/crates/lopdf) – Lesen, Bearbeiten und Schreiben von PDF-Dateien
- [anyhow](https://crates.io/crates/anyhow) – Fehlerbehandlung

## Lizenz

Dieses Projekt steht unter der [MIT](LICENSE_MIT)- oder [Apache-2.0](LICENSE_APACHE)-Lizenz, nach eigener Wahl.
