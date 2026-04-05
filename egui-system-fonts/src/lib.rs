//! System font helpers for `egui`.
//!
//! This crate resolves platform-installed font families and applies them to an `egui::Context`.
//!
//! # Quick start
//!
//! Replace `egui`'s default fonts using the current system locale:
//!
//! ```no_run
//! # use egui_system_fonts::{set_auto, FontStyle};
//! # fn demo(ctx: &egui::Context) {
//! set_auto(ctx, FontStyle::Sans);
//! # }
//! ```
//!
//! Add system fonts as fallback only (keeps existing font priority):
//!
//! ```no_run
//! # use egui_system_fonts::{add_auto, FontStyle};
//! # fn demo(ctx: &egui::Context) {
//! let mut defs = egui::FontDefinitions::default();
//! add_auto(ctx, &mut defs, FontStyle::Sans);
//! # }
//! ```
//!
use egui::{
    epaint::text::{FontInsert, FontPriority, InsertFontFamily},
    FontData, FontDefinitions, FontFamily,
};
use std::collections::BTreeMap;
use std::sync::Arc;
use system_fonts::FoundFontSource;
pub use system_fonts::{FontPreset, FontRegion, FontStyle};

/// Replaces `egui` font definitions with system fonts detected from the current system locale.
///
/// This overwrites the default `egui` fonts. If no matching fonts are found, the context is left unchanged
/// and an empty list is returned.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_auto, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// set_auto(ctx, FontStyle::Sans);
/// # }
/// ```
pub fn set_auto(ctx: &egui::Context, style: FontStyle) -> Vec<String> {
    let (locale, region, fonts) = system_fonts::find_for_system_locale(style);
    log::info!(
        "Detected locale: {:?}, region: {:?}, style: {:?}, candidates: {}",
        locale,
        region,
        style,
        fonts.len()
    );
    set_found_fonts(ctx, fonts)
}

/// Replaces `egui` font definitions with system fonts for the given region.
///
/// This overwrites the default `egui` fonts. If no matching fonts are found, the context is left unchanged
/// and an empty list is returned.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_with_region, FontRegion, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// set_with_region(ctx, FontRegion::Korean, FontStyle::Sans);
/// # }
/// ```
pub fn set_with_region(ctx: &egui::Context, region: FontRegion, style: FontStyle) -> Vec<String> {
    let presets = system_fonts::presets_for_region(region);
    set_with_presets(ctx, presets, style)
}

/// Replaces `egui` font definitions with system fonts resolved from the given presets.
///
/// Presets are evaluated in priority order. If no matching fonts are found, the context is left unchanged
/// and an empty list is returned.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{set_with_presets, FontPreset, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let presets = [FontPreset::Korean, FontPreset::Latin];
/// set_with_presets(ctx, presets, FontStyle::Sans);
/// # }
/// ```
pub fn set_with_presets<I>(ctx: &egui::Context, presets: I, style: FontStyle) -> Vec<String>
where
    I: IntoIterator<Item = FontPreset>,
{
    let fonts = system_fonts::find_from_presets(presets, style);
    set_found_fonts(ctx, fonts)
}

/// Appends system fonts as fallback families to an existing `FontDefinitions`.
///
/// This keeps existing font priority and only adds additional fallback families at the end.
/// If at least one font is added, the updated definitions are applied to `ctx`.
///
/// Returns the newly added font family names (in priority order). If nothing is added, returns an empty list
/// and does not modify the context.
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_auto, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let mut defs = egui::FontDefinitions::default();
/// add_auto(ctx, &mut defs, FontStyle::Sans);
/// # }
/// ```
pub fn add_auto(ctx: &egui::Context, defs: &mut FontDefinitions, style: FontStyle) -> Vec<String> {
    let (locale, region, fonts) = system_fonts::find_for_system_locale(style);
    log::info!(
        "Detected locale: {:?}, region: {:?}, style: {:?}, candidates: {}",
        locale,
        region,
        style,
        fonts.len()
    );
    add_found_fonts(ctx, defs, fonts)
}
#[deprecated(
    since = "0.34.0",
    note = "Renamed to `add_auto` for egui 0.34. Use `add_auto` instead."
)]
pub fn extend_auto(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    style: FontStyle,
) -> Vec<String> {
    add_auto(ctx, defs, style)
}

/// Appends system fonts for the given region as fallback families to an existing `FontDefinitions`.
///
/// If at least one font is added, the updated definitions are applied to `ctx`.
/// Returns the newly added font family names (in priority order).
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_with_region, FontRegion, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let mut defs = egui::FontDefinitions::default();
/// add_with_region(ctx, &mut defs, FontRegion::Japanese, FontStyle::Sans);
/// # }
/// ```
pub fn add_with_region(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    region: FontRegion,
    style: FontStyle,
) -> Vec<String> {
    let presets = system_fonts::presets_for_region(region);
    add_with_presets(ctx, defs, presets, style)
}
#[deprecated(
    since = "0.34.0",
    note = "Renamed to `add_with_region` for egui 0.34. Use `add_with_region` instead."
)]
pub fn extend_with_region(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    region: FontRegion,
    style: FontStyle,
) -> Vec<String> {
    add_with_region(ctx, defs, region, style)
}

/// Appends system fonts resolved from the given presets as fallback families to an existing `FontDefinitions`.
///
/// Presets are evaluated in priority order. If at least one font is added, the updated definitions are applied
/// to `ctx`. Returns the newly added font family names (in priority order).
///
/// # Examples
///
/// ```no_run
/// # use egui_system_fonts::{add_with_presets, FontPreset, FontStyle};
/// # fn demo(ctx: &egui::Context) {
/// let mut defs = egui::FontDefinitions::default();
/// let presets = [FontPreset::TraditionalChinese, FontPreset::Latin];
/// add_with_presets(ctx, &mut defs, presets, FontStyle::Serif);
/// # }
/// ```
pub fn add_with_presets<I>(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    presets: I,
    style: FontStyle,
) -> Vec<String>
where
    I: IntoIterator<Item = FontPreset>,
{
    let fonts = system_fonts::find_from_presets(presets, style);
    add_found_fonts(ctx, defs, fonts)
}
#[deprecated(
    since = "0.34.0",
    note = "Renamed to `add_with_presets` for egui 0.34. Use `add_with_presets` instead."
)]
pub fn extend_with_presets<I>(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    presets: I,
    style: FontStyle,
) -> Vec<String>
where
    I: IntoIterator<Item = FontPreset>,
{
    add_with_presets(ctx, defs, presets, style)
}

fn set_found_fonts(ctx: &egui::Context, fonts: Vec<system_fonts::FoundFont>) -> Vec<String> {
    let mut defs = FontDefinitions::default();

    let mut installed_names: Vec<String> = Vec::new();
    let mut keys_in_priority: Vec<String> = Vec::new();

    for f in fonts {
        let Some(bytes) = read_font_bytes(f.source) else {
            continue;
        };

        let data = FontData::from_owned(bytes);

        defs.font_data.insert(f.key.clone(), Arc::new(data.clone()));

        keys_in_priority.push(f.key.clone());
        installed_names.push(f.family);
    }

    if installed_names.is_empty() {
        log::warn!("No matching system fonts found.");
        return vec![];
    }

    for key in keys_in_priority.into_iter().rev() {
        insert_front(&mut defs.families, FontFamily::Proportional, key.clone());
        insert_front(&mut defs.families, FontFamily::Monospace, key);
    }

    ctx.set_fonts(defs);
    log::info!("Set fonts (family names): {:?}", installed_names);

    installed_names
}

fn add_found_fonts(
    ctx: &egui::Context,
    defs: &mut FontDefinitions,
    fonts: Vec<system_fonts::FoundFont>,
) -> Vec<String> {
    let mut installed_names: Vec<String> = Vec::new();

    for f in fonts {
        if defs.font_data.contains_key(&f.key) {
            continue;
        }

        let Some(bytes) = read_font_bytes(f.source) else {
            continue;
        };

        let data = FontData::from_owned(bytes);

        defs.font_data.insert(f.key.clone(), Arc::new(data.clone()));

        insert_back(&mut defs.families, FontFamily::Proportional, f.key.clone());
        insert_back(&mut defs.families, FontFamily::Monospace, f.key.clone());

        ctx.add_font(FontInsert {
            name: f.key.clone(),
            data,
            families: vec![
                InsertFontFamily {
                    family: FontFamily::Proportional,
                    priority: FontPriority::Lowest,
                },
                InsertFontFamily {
                    family: FontFamily::Monospace,
                    priority: FontPriority::Lowest,
                },
            ],
        });

        installed_names.push(f.family);
    }

    installed_names
}

fn read_font_bytes(source: FoundFontSource) -> Option<Vec<u8>> {
    match source {
        FoundFontSource::Path(path) => match std::fs::read(&path) {
            Ok(b) => Some(b),
            Err(e) => {
                log::debug!("Failed to read font file {:?}: {}", path, e);
                None
            }
        },
        FoundFontSource::Bytes(b) => Some(b.as_ref().to_vec()),
    }
}

fn insert_front(families: &mut BTreeMap<FontFamily, Vec<String>>, family: FontFamily, key: String) {
    let list = families.entry(family).or_default();
    if list.iter().any(|k| k == &key) {
        return;
    }
    list.insert(0, key);
}

fn insert_back(families: &mut BTreeMap<FontFamily, Vec<String>>, family: FontFamily, key: String) {
    let list = families.entry(family).or_default();
    if list.iter().any(|k| k == &key) {
        return;
    }
    list.push(key);
}
