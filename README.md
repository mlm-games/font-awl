# font-awl

Cross-platform font provider for Rust, built on [parley](https://github.com/linebender/parley).

Used for font loading in [repose](https://github.com/mlm-games/repose) and apps using it: system fonts where available, optional bundled fallbacks, app-supplied font bytes, and a small `FontProvider` trait.

License: MIT OR Apache-2.0

## What it does

- Owns a fontique `Collection` and optional per-script fallback chains
- Loads system fonts on desktop at construction (when the `system` feature is on)
- Defers system loading on Android (NDK `ASystemFontIterator`, API 29+); WASM local fonts are opt-in (`local-fonts`)
- Registers optional bundled fonts controlled by Cargo features
- Registers arbitrary app fonts from bytes (TTF/OTF/TTC/OTC) with optional `FontInfoOverride`
- Builds a parley `FontContext` from the collection when the `parley` feature is enabled
- Tracks font `Blob`s and can drain them for external font systems

## Features (Cargo)

| Feature | Default | Role |
|---------|---------|------|
| `std` | yes | std support for fontique/parley |
| `system` | yes | Desktop system fonts at init (via fontique) |
| `parley` | yes | `new_parley_context()` and re-export |
| `fontconfig-dlopen` | yes | fontique fontconfig dlopen on Linux |
| `basic` | no | Bundle OpenSans + Noto Sans Symbols 2 (if files present) |
| `emoji` | no | Bundle Noto Color Emoji |
| `cjk` | no | Bundle Noto Sans CJK (if file present) |
| `monospace` | no | Bundle JetBrains Mono |
| `all-noto` | no | `basic` + `emoji` + `cjk` |
| `local-fonts` | no | WASM: compile `Provider::load_web_fonts()` (browser Local Font Access) |

Bundled files live under `fonts/`. `build.rs` sets cfg flags only when the file exists; enabling a feature without the file emits a cargo warning.

Present in-tree by default (when checked in): OpenSans, Noto Sans Symbols 2, Noto Color Emoji, JetBrains Mono. CJK file is optional (`NotoSansCJK-Regular.ttc`).

## Platform behavior

| Platform | System fonts |
|----------|----------------|
| Linux / macOS / Windows | Loaded in `Provider::new()` if `system` is enabled |
| Android | Call `load_system_fonts_best_effort()` (libandroid `ASystemFontIterator`, best-effort read of font paths) |
| WASM | Call `load_web_fonts().await` with the `local-fonts` feature (`queryLocalFonts`; needs secure context + user gesture + a document Permissions Policy that grants `local-fonts`). Falls back to bundled/app fonts if unavailable |

`load_system_fonts_best_effort` is idempotent (skips if already attempted). Failures for individual fonts are skipped where possible.

## Usage

```rust
use font_awl::{FontProvider, Provider};

let mut provider = Provider::new();

// Optional bundled sets (feature-gated)
provider.load_bundled_fonts();

// App fonts
provider.load_app_fonts(&ttf_bytes, None);

// Android / explicit system load
let _ = provider.load_system_fonts_best_effort();

// WASM
#[cfg(all(target_arch = "wasm32", feature = "local-fonts"))]
provider.load_web_fonts().await?;

// Script fallbacks
// provider.set_fallback(script, vec![family_id, ...]);

let collection = provider.collection();

#[cfg(feature = "parley")]
let font_cx = provider.new_parley_context();
```

### FontProvider trait

```rust
fn load_bundled_fonts(&mut self);
fn load_app_fonts(&mut self, bytes: &[u8], info: Option<FontInfoOverride<'_>>);
fn load_system_fonts_best_effort(&mut self) -> Result<(), Error>;
fn fallback_for_script(&self, script: Script) -> Vec<FontId>;
```

`Provider` also exposes `register_fonts`, `set_fallback`, `clear_fallback_cache`, `collection` / `collection_mut`, `drain_font_data`, and (with `parley`) `new_parley_context`.

## Errors

`font_awl::Error`: not supported on platform, web Local Font Access failures, Android NDK load failures, and wrapped platform errors.

## Development

```bash
cargo test
cargo test --features parley,basic,emoji,system
```

CI installs fontconfig dev headers on Linux. Release workflow publishes to crates.io on version tags.

## Related

Initially designed for repose, for font discovery across desktop, Android, and web.
