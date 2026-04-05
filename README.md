# egui-system-fonts

System font loader helpers for [`egui`](https://github.com/emilk/egui).

This repository is a small workspace containing:

- **`egui-system-fonts`** — the library crate published on crates.io
- **`demo-egui`** — a small native demo app to test fonts and fallbacks

The core, UI-agnostic font discovery logic lives in a separate repository:

- **`system-fonts`**: https://github.com/yijehyung/system-fonts

## Crate

- crates.io: `egui-system-fonts`
- docs.rs: https://docs.rs/egui-system-fonts

Add to your project:

```toml
[dependencies]
egui-system-fonts = "0.34.0"
```

Minimal usage:

```rust,no_run
use egui_system_fonts::{set_auto, FontStyle};

fn setup_fonts(ctx: &egui::Context) {
    set_auto(ctx, FontStyle::Sans);
}
```

```rust,no_run
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

        eframe::run_native(
        "App Name",
        options,
        Box::new(|cc| {
            let mut defs = egui::FontDefinitions::default();
            egui_system_fonts::add_auto(&cc.egui_ctx, &mut defs, egui_system_fonts::FontStyle::Sans);
            cc.egui_ctx.set_fonts(defs);
            Ok(Box::new(MyApp::default()))
        }),
    )
}
```

## Demo app

From the workspace root:

```bash
cargo run -p demo-egui
```

## License

Licensed under either of:

- Apache License, Version 2.0
- MIT license
