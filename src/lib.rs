mod error;
mod platform;

pub use error::Error;
pub use fontique;

use std::collections::HashMap;

use fontique::{
    Blob, Collection, CollectionOptions, FamilyId, FontInfoOverride, Script, SourceCache,
};

#[cfg(feature = "parley")]
pub use parley;

/// Font identifier for fallback chains.
pub type FontId = FamilyId;

/// Cross-platform font provider trait.
pub trait FontProvider {
    /// Register bundled default fonts controlled by Cargo features.
    fn load_bundled_fonts(&mut self);

    /// Register font data from raw bytes (`.ttf`, `.otf`, `.ttc`, `.otc`).
    ///
    /// `info` allows overriding family name, weight, style, or stretch
    /// at registration time (equivalent to CSS `@font-face` descriptors).
    fn load_app_fonts(&mut self, bytes: &[u8], info: Option<FontInfoOverride<'_>>);

    /// Best-effort system font loading using platform-native APIs.
    ///
    /// Returns `Ok(())` even if some or all fonts fail to load.
    fn load_system_fonts_best_effort(&mut self) -> Result<(), Error>;

    /// Per-script fallback font chain.
    ///
    /// Returns font IDs ordered by preference. Empty if no script-specific
    /// fallback has been configured.
    fn fallback_for_script(&self, script: Script) -> Vec<FontId>;
}

/// Concrete font provider using fontique.
///
/// Manages a [`fontique::Collection`] and provides platform-specific
/// system font loading for desktop (native), WASM (Local Font Access),
/// and Android (NDK `ASystemFontIterator`).
///
/// # Desktop (Linux, macOS, Windows)
///
/// System fonts are loaded automatically at construction when the `system`
/// feature is enabled (default). No additional calls needed.
///
/// # WASM
///
/// System fonts are **not** loaded at construction. Call [`load_web_fonts()`]
/// after construction to query the browser's Local Font Access API.
/// Bundled fonts (via [`load_bundled_fonts()`]) provide fallback.
///
/// # Android
///
/// System fonts are **not** loaded at construction. Call
/// [`load_system_fonts_best_effort()`] to enumerate via the NDK
/// `ASystemFontIterator` API (requires API level 29+).
pub struct Provider {
    collection: Collection,
    fallback_map: HashMap<Script, Vec<FamilyId>>,
    tried_system: bool,
    font_data: Vec<Vec<u8>>,
}

impl FontProvider for Provider {
    fn load_bundled_fonts(&mut self) {
        let data = platform::register_default_fonts(&mut self.collection);
        self.font_data.extend(data);
    }

    fn load_app_fonts(&mut self, bytes: &[u8], info: Option<FontInfoOverride<'_>>) {
        self.register_fonts(bytes, info);
    }

    fn load_system_fonts_best_effort(&mut self) -> Result<(), Error> {
        if self.tried_system {
            return Ok(());
        }
        self.tried_system = true;
        let data = platform::load_system_fonts(&mut self.collection)?;
        self.font_data.extend(data);
        Ok(())
    }

    fn fallback_for_script(&self, script: Script) -> Vec<FontId> {
        self.fallback_map.get(&script).cloned().unwrap_or_default()
    }
}

impl Provider {
    /// Create a new font provider.
    ///
    /// On desktop, system fonts are loaded at construction when the
    /// `system` feature is enabled (default). On WASM and Android,
    /// system font loading is deferred to explicit calls.
    pub fn new() -> Self {
        let collection = Collection::new(CollectionOptions {
            system_fonts: platform::system_fonts_at_init(),
            shared: true,
        });

        Self {
            collection,
            fallback_map: HashMap::new(),
            tried_system: false,
            font_data: Vec::new(),
        }
    }

    /// Register raw font data.
    pub fn register_fonts(&mut self, data: &[u8], info: Option<FontInfoOverride<'_>>) {
        self.font_data.push(data.to_vec());
        let blob: Blob<u8> = data.to_vec().into();
        self.collection.register_fonts(blob, info);
    }

    /// Set a per-script fallback chain.
    ///
    /// `families` is an ordered list of font family names (or generic families)
    /// to try when text in `script` is encountered.
    pub fn set_fallback<S>(&mut self, script: Script, families: Vec<FamilyId>) {
        self.fallback_map.insert(script, families);
    }

    /// Rebuild per-script fallback cache from registered fonts.
    ///
    /// Should be called after registering fonts that may be the best choice
    /// for a particular script.
    pub fn refresh_fallback_cache(&mut self) {
        self.fallback_map.clear();
    }

    /// WASM: load system fonts via browser Local Font Access API.
    ///
    /// Requires a secure context (HTTPS) and a user activation gesture.
    /// Falls back to bundled fonts via [`load_bundled_fonts()`] if the
    /// API is unavailable in the current browser.
    #[cfg(target_arch = "wasm32")]
    pub async fn load_web_fonts(&mut self) -> Result<(), Error> {
        if self.tried_system {
            return Ok(());
        }
        self.tried_system = true;
        let data = platform::load_web_fonts(&mut self.collection).await?;
        self.font_data.extend(data);
        Ok(())
    }

    /// Reference to the underlying fontique collection.
    pub fn collection(&self) -> &Collection {
        &self.collection
    }

    /// Mutable reference to the underlying fontique collection.
    pub fn collection_mut(&mut self) -> &mut Collection {
        &mut self.collection
    }

    /// Drain all tracked font data bytes for use with an external font system
    pub fn drain_font_data(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.font_data)
    }

    /// Build a parley `FontContext` from this provider's collection.
    ///
    /// Clones the collection so the context is independent of the provider
    /// after creation.
    #[cfg(feature = "parley")]
    pub fn new_parley_context(&self) -> parley::FontContext {
        parley::FontContext {
            collection: self.collection.clone(),
            source_cache: SourceCache::default(),
        }
    }
}

impl Default for Provider {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Debug for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Provider")
            .field("tried_system", &self.tried_system)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_constructs() {
        let provider = Provider::new();
        assert!(!provider.tried_system);
    }

    #[test]
    fn system_fonts_resolve_generic_families() {
        let mut provider = Provider::new();
        let ids: Vec<_> = provider
            .collection_mut()
            .generic_families(fontique::GenericFamily::SansSerif)
            .collect();
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        assert!(!ids.is_empty(), "expected system fonts on desktop");
    }

    #[test]
    fn default_is_noop() {
        let provider = Provider::default();
        assert!(!provider.tried_system);
    }

    #[test]
    fn debug_doesnt_panic() {
        let provider = Provider::new();
        let _ = format!("{provider:?}");
    }

    #[test]
    fn trait_impl_compiles() {
        fn takes_trait(_p: &mut impl FontProvider) {}
        let mut provider = Provider::new();
        takes_trait(&mut provider);
    }

    #[test]
    fn font_provider_load_app_fonts() {
        let mut provider = Provider::new();
        provider.load_app_fonts(&[], None);
    }

    #[cfg(feature = "parley")]
    #[test]
    fn golden_parley_layout_produces_glyphs() {
        let provider = Provider::new();

        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            let mut font_cx = provider.new_parley_context();
            let mut layout_cx = parley::LayoutContext::<[u8; 4]>::new();
            let mut builder = layout_cx.ranged_builder(&mut font_cx, "Hello", 16.0, true);
            builder.push_default(parley::style::StyleProperty::FontSize(16.0));
            let mut layout = builder.build("Hello");
            layout.break_all_lines(None);

            let mut glyph_count = 0;
            for line in layout.lines() {
                for run in line.runs() {
                    for cluster in run.clusters() {
                        for _glyph in cluster.glyphs() {
                            glyph_count += 1;
                        }
                    }
                }
            }
            assert!(glyph_count > 0, "expected glyphs from layout");
        }

        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            let _ = provider;
            // No system fonts available, test is a no-op
        }
    }

    #[cfg(feature = "parley")]
    #[test]
    fn golden_parley_non_notdef_glyphs() {
        let provider = Provider::new();

        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            let mut font_cx = provider.new_parley_context();
            let mut layout_cx = parley::LayoutContext::<[u8; 4]>::new();
            let mut builder = layout_cx.ranged_builder(&mut font_cx, "A", 16.0, true);
            builder.push_default(parley::style::StyleProperty::FontSize(16.0));
            let mut layout = builder.build("A");
            layout.break_all_lines(None);

            let mut seen_notdef = false;
            let mut seen_valid = false;
            for line in layout.lines() {
                for run in line.runs() {
                    for cluster in run.clusters() {
                        for glyph in cluster.glyphs() {
                            if glyph.id == 0 {
                                seen_notdef = true;
                            } else {
                                seen_valid = true;
                            }
                        }
                    }
                }
            }
            assert!(
                seen_valid,
                "expected at least one non-.notdef glyph; notdef only: {seen_notdef}"
            );
        }

        #[cfg(any(target_arch = "wasm32", target_os = "android"))]
        {
            let _ = provider;
        }
    }
}
