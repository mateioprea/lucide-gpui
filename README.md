[![Crates.io](https://img.shields.io/crates/v/lucide-gpui)](https://crates.io/crates/lucide-gpui) [![docs.rs](https://img.shields.io/docsrs/lucide-gpui)](https://docs.rs/lucide-gpui)

## Lucide GPUI

Implementation of the [Lucide icon library](https://github.com/lucide-icons/lucide) for [GPUI](https://www.gpui.rs) and [gpui-kit](https://crates.io/crates/gpui-kit).

Use Lucide icons in your GPUI applications with ease!

## Features

**📦 Embedded in your binary**

All 2,108 icons are compiled into your application. There are no asset folders to ship and nothing to load from disk at runtime.

**🔤 Type-safe names**

Every icon is a variant of the `LucideIcon` enum. A typo such as `LucideIcon::AlarmClok` is a compile error, and your editor can autocomplete the full list.

**🎨 Works with gpui-component**

`LucideIcon` implements gpui-component's `IconNamed`, so it plugs straight into `Icon::new(...)` and picks up your theme's colors and sizing.

**🔄 Easy to sync with Lucide**

The icon set comes from the [`lucide-static`](https://www.npmjs.com/package/lucide-static) npm package. Updating is one script and a rebuild (see [Updating the icons](#updating-the-icons)).

## Installation

### Requirements

lucide-gpui builds on **gpui-kit 0.6** and uses Rust edition 2024, so it needs **Rust 1.85 or newer**.

### Rust (Cargo)

In an existing GPUI project, add lucide-gpui as a dependency:

```
cargo add lucide-gpui
```

## Usage

Register `LucideAssets` on your application, then draw icons with gpui-component's `Icon`:

```rust
use gpui_kit::component::*;
use gpui_kit::*;
use lucide_gpui::{LucideAssets, LucideIcon};

struct Example;

impl Render for Example {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(Icon::new(LucideIcon::Play).text_color(cx.theme().red))
            .child(Icon::new(LucideIcon::Armchair))
    }
}

fn main() {
    // Register the icons with your GPUI application.
    let app = gpui_kit::application().with_assets(LucideAssets);

    app.run(move |cx| {
        // gpui-kit must be initialized before using its components.
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Example);
                // The first level on the window must be Root.
                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
```

You can run this example from a clone of the repository:

```
cargo run --example icons_list
```

### Using Lucide together with other assets

A GPUI application registers a single asset source. To use Lucide icons next to gpui-kit's built-in icons (or your own), write a small source that asks each one in turn:

```rust
use gpui_kit::*;
use lucide_gpui::LucideAssets;
use std::borrow::Cow;

struct AppAssets;

impl AssetSource for AppAssets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        if let Some(bytes) = LucideAssets.load(path)? {
            return Ok(Some(bytes));
        }
        gpui_kit::assets::Assets.load(path)
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        let mut paths = gpui_kit::assets::Assets.list(path)?;
        paths.extend(LucideAssets.list(path)?);
        paths.sort();
        paths.dedup();
        Ok(paths)
    }
}

// Then register it instead of `LucideAssets`:
// let app = gpui_kit::application().with_assets(AppAssets);
```

Lucide icons live under the `lucide_icons/` path prefix, so they never collide with the paths of other asset sources.

## Reference

### Library Exports

```rust
use lucide_gpui::{LucideAssets, LucideIcon};
```

#### LucideIcon

An enum with one variant for every Lucide icon. It implements gpui-component's `IconNamed`, so it can be passed to `Icon::new(...)` and to anything else that accepts an icon name.

It derives `Clone`, `Copy`, `Debug`, `PartialEq` and `Eq`.

#### LucideAssets

A GPUI `AssetSource` that serves the embedded SVG files. Register it with `.with_assets(LucideAssets)`, or combine it with other sources as shown [above](#using-lucide-together-with-other-assets).

## Available Icons

You can search for an icon on the [Lucide Icons](https://lucide.dev/icons/) website.

Variant names are the same PascalCase names Lucide uses for its own components, so you can copy them straight from the website:

1. Click an icon to open its details. `a-arrow-down`
2. Click the **ChevronUp button** to expand the copy menu.
3. Select **Copy Component Name** to get the icon's PascalCase name. `AArrowDown`

Or convert the name yourself: split it at every hyphen, capitalize the first letter of each part, and join the parts together.

| Lucide name | Variant |
|-------------|---------|
| `alarm-clock` | `LucideIcon::AlarmClock` |
| `a-arrow-down` | `LucideIcon::AArrowDown` |
| `heart` | `LucideIcon::Heart` |
| `grid-2x2` | `LucideIcon::Grid2x2` |

Then use the variant with `Icon`:

```rust
Icon::new(LucideIcon::AlarmClock)
```

### Icons that share a name

Four old alias names turn into the same PascalCase name as the icon they replaced, and they draw exactly the same shape. Only the current icon is included:

| Variant | Included file | Not included (alias) |
|---------|---------------|----------------------|
| `ArrowDown01` | `arrow-down-0-1` | `arrow-down-01` |
| `ArrowDown10` | `arrow-down-1-0` | `arrow-down-10` |
| `ArrowUp01` | `arrow-up-0-1` | `arrow-up-01` |
| `ArrowUp10` | `arrow-up-1-0` | `arrow-up-10` |

## Updating the icons

The SVG files in `lucide_icons/` are copied from the `lucide-static` npm package. To update them:

```
yarn add lucide-static@latest
yarn generate-icons
cargo build
```

`yarn generate-icons` copies the SVG files into `lucide_icons/`. `cargo build` then regenerates the `LucideIcon` enum from whatever files are in that folder, so new icons appear as new variants automatically.

This version was generated from `lucide-static` **1.47.0**.

## A note on AI

The code in this package was written without AI. Only this README file was written with AI assistance.

## License

This project is licensed under the [MIT license](LICENSE).

The Lucide icons are licensed under the [ISC License](https://github.com/lucide-icons/lucide/blob/main/LICENSE).
