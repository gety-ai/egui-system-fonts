# egui_system_fonts

System font loader helpers for `egui`.

- Auto-detects the system locale and picks a reasonable font fallback chain
- Can either replace `egui` fonts (set) or append fallback fonts only (add)
- Supports region presets (Korean/Japanese/Chinese/Cyrillic/Latin)

## Installation

```toml
[dependencies]
egui-system-fonts = "0.34.1"
```

## Usage

### Replace all egui fonts (auto-detect locale)

```rust,no_run
use egui_system_fonts::{set_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_auto(ctx, FontStyle::Sans);
}
```

### Fallback only (keep existing priorities)

```rust,no_run
use egui_system_fonts::{add_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    add_auto(ctx, &mut defs, FontStyle::Sans);
}
```

### Force a region

```rust,no_run
use egui_system_fonts::{set_with_region, FontRegion, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_with_region(ctx, FontRegion::Korean, FontStyle::Sans);
}
```

### Fallback only, force a region

```rust,no_run
use egui_system_fonts::{add_with_region, FontRegion, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    add_with_region(ctx, &mut defs, FontRegion::Japanese, FontStyle::Sans);
}
```

### Use custom presets

```rust,no_run
use egui_system_fonts::{set_with_presets, FontPreset, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let presets = [FontPreset::Korean, FontPreset::Latin];
    set_with_presets(ctx, presets, FontStyle::Sans);
}
```

### Fallback only, custom presets

```rust,no_run
use egui_system_fonts::{add_with_presets, FontPreset, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    let mut defs = egui::FontDefinitions::default();
    let presets = [FontPreset::TraditionalChinese, FontPreset::Latin];
    add_with_presets(ctx, &mut defs, presets, FontStyle::Serif);
}
```

## Notes

- If no matching system fonts are found, the functions return an empty list.
- `add_*` adds fonts to the `egui::Context` as fallback fonts and also updates the provided `FontDefinitions`.
- `set_*` overwrites the default `egui` fonts.

## License

MIT OR Apache-2.0
