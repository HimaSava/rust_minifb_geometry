# minifb_geometry

Addon crate for the [minifb](https://github.com/emoon/rust_minifb/tree/master) crate, that enables you to add basic geometric shapes into the minifb window.

![Example](examples/example.png?raw=true)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
minifb_geometry = "0.1"
```

## Example

```rust
use minifb::{Window, WindowOptions};
use minifb_geometry::GeometryDrawer;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let geometry = GeometryDrawer::new(WIDTH);

    let mut window = Window::new(
        "minifb_geometry - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let _ = geometry.draw_box(&mut buffer, 120, 130, 220, 230, 0xffff00);
    let _ = geometry.draw_circle(&mut buffer, 320, 180, 50, 0xffffff);
    let _ = geometry.draw_rectangle(&mut buffer, 420, 130, 520, 230, 5, 0x00ff00);

    let _ = geometry.draw_line(&mut buffer, 10, 10, WIDTH - 10, 10,0xff00ff);
    let _ = geometry.draw_line(&mut buffer, 10, 10, 10, HEIGHT - 10,0xff00ff);
    let _ = geometry.draw_line(&mut buffer, WIDTH - 10, 10, WIDTH - 10, HEIGHT - 10,0xff00ff);
    let _ = geometry.draw_line(&mut buffer, 10, HEIGHT - 10, WIDTH - 10, HEIGHT - 10,0xff00ff);

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

```

## License

 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
