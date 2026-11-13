# RB Deck: what we started with

An audit of the DeckCraft code RB Deck was forked from, done when the fork was made (November 2026),
and what was fixed straight away. Upstream's own documents are not taken on trust here: each finding
below was checked by building, running or reading the code.

## The starting point

- **DeckCraft 0.18.1**, upstream commit `9b1fe3f` (2026-11-12), MIT OR Apache-2.0.
- About 184,000 lines of Rust, of which about 48,000 are tests, in layered crates (geometry, model,
  fonts, text, render, `.pptx` and native formats, engine, MCP, egui UI) and three apps: desktop,
  command line, browser (WebAssembly).
- Its authors call it **alpha**, about 55% of the way to PowerPoint for real work (`ROADMAP.md`).
- Every action is a command, so the command line, MCP and the local control channel do exactly what
  the UI does. That is what RB Home needs: the Hub can build and convert decks without a window.

## What holds up

- **It builds cleanly.** The command-line tool builds in about 3 minutes (release, 4 cores).
- **Its tests pass:** 2,362 of them (see "Test run" below).
- **Crash discipline is real, not just written down.** `unsafe` is forbidden in the whole workspace,
  and every production crate denies `unwrap`, `expect`, `panic!` and the like (checked: every crate
  root and every `Cargo.toml` carries them). Commands run inside a panic guard.
- **Hostile files are refused safely.** Tried with the command-line tool: an empty file, 50 KB of
  random bytes and a half-cut `.pptx` are each refused in under 0.02 s with "not a presentation RB
  Deck can read". A `.pptx` with broken XML and one carrying a 2 GB zip bomb open without crashing or
  inflating the bomb (the reader has size and decompression budgets).
- **Doc Desk's decks work.** A 16:9 deck from RB Home's Doc Desk opens, renders the same as
  LibreOffice renders it, takes a new slide and saves back to `.pptx` with every slide's text and the
  speaker notes kept; LibreOffice and python-pptx both open what RB Deck wrote. A 31-slide deck at
  Doc Desk's limits converts to `.pptx` and PDF.
- **No telemetry, and no network use of its own** apart from the updater (below): no HTTP client in
  its own code; the control channel listens on 127.0.0.1 only and only when started with `--control`.

## Fixed in the fork

| Problem | Why it mattered | Fix |
|---|---|---|
| The desktop app checked upstream's releases at start-up and could download and install them | It would replace RB Deck with DeckCraft, and it called out to the internet | No update checks; Help › Check for Updates is gone and the command says RB Deck is updated with RB Home |
| The Windows installer kept upstream's upgrade code | Installing RB Deck would remove DeckCraft, and the other way round | New upgrade code; product, maker, shortcuts, file type (`RBDeck.Presentation`) and install folder (`Programs\Recking Ball\RB Deck`) are RB Deck's |
| The macOS bundle and document types used upstream's identifiers | macOS would treat RB Deck as DeckCraft | `com.reckingballresearch.rbdeck` |
| ArtCraft logos, name, website and Discord links in the app, README and packages; a Flatpak permission to the ArtCraft launcher's folder | The ArtCraft marks are not open source and forks must remove them (their brand licence) | All removed; links point at this repository; "based on DeckCraft by the ArtCraft team" in plain text, as allowed |
| Upstream's app icon | Not ours | New icon (`packaging/rb-icon.py`), also drawn in code for About |
| `rustix` 1.1.5 and `tokio` 1.53.2 flagged unsound; `async-executor` 1.14.0 and `clipboard-win` 5.4.1 withdrawn by their authors | Known defects in dependencies | Updated to `rustix` 1.1.6, `tokio` 1.54.1, `async-executor` 1.14.1, `clipboard-win` 5.4.3 |

## Found and not fixed yet

- **`zip` 3.0.0 has four security advisories** (RUSTSEC-2026-0367, -0368, -0369, -0429). All four are
  in `ZipArchive::extract`, which unpacks an archive onto disk; RB Deck never calls it, so they can't
  be reached. Moving to `zip` 9 changes about 50 places in the tests; it's its own change.
- **Unmaintained dependencies:** `rustybuzz` and `ttf-parser` (text shaping and fonts), and `zbus`
  with `zvariant` (Linux desktop portals). Nothing to fix today; watch them.
- **Windows start-up crashes on some Intel and AMD graphics** (upstream issues #40, #59). The HP Envy
  runs Windows: try RB Deck there before relying on it.
- **Gaps upstream lists that matter to RB Home:** SVG pictures don't render, PDF export rasterizes
  slide artwork, no printing, English only.
- **Upstream's documents are unreliable.** `docs/gaps.md` still lists "Close discards an edited
  presentation without asking" (#60), but the code asks, and a test checks it. `ROADMAP.md` opens with
  about 18 stacked "Last reviewed" lines. Trust the tests, not the prose.
- **CI runs upstream's shared workflows** (`storytold/craft-common-actions@main`) with upstream's
  secrets, so it won't run usefully for this fork. RB Deck needs its own.
- **Fonts:** release builds embed the fonts from `storytold/craft-fonts` (OFL-1.1). Without that
  checkout beside this one, the app builds but falls back to system fonts. RB Deck's builds need it
  (a fork or a pinned copy).
- **Still named DeckCraft inside:** crate names, the binaries (`deckcraft`, `deckcraft-cli`), the native
  `.deckcraft` format and the Linux app id (`ai.storyteller.deckcraft`). They change with RB Deck's own
  release pipeline, where each platform's package can be built and checked.

## Test run

On this branch, Linux x86_64, Rust 1.97, `cargo test --release --workspace`: **2,362 passed, 0 failed,
5 ignored** (43 test targets). Two things the machine needs: the ALSA headers (`libasound2-dev`) to
build, and a Vulkan driver for the 13 UI tests that draw through the graphics stack; on a machine
without a graphics card, Mesa's software driver (`mesa-vulkan-drivers`) does. Run with
`--test-threads=2` on a 16 GB machine: at full parallelism the suite ran out of memory.

The first run on this branch also caught mistakes in the rename itself (test fonts whose names
contain "DeckCraft", and the tests that check the About page and MCP server name); they are fixed.
