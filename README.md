# i3blocks-battery-widget
Laptop battery widget for `i3blocks`.

![2022-11-23_12-07](https://user-images.githubusercontent.com/44941959/203508832-aa40c84d-e411-4415-9434-ec8af9137e06.png)
## Installation
- Install Cargo.
- Run `cargo build --release` in project directory.
- Move executable file `target/release/i3blocks-battery-widget` in a more handy location, e.g. `~/.local/bin`.
- Add new module in i3blocks config (`~/.config/i3blocks/config`):

  ```
  [battery]
  command=~/.local/bin/i3blocks-battery-widget
  interval=persist
  markup=pango
  ```

## Q&A
Q: Why do I see `<span>`s in my status bar and do not see any colors?

A: Possible causes:
- you're using old i3blocks or i3wm without pango markup support,
- your i3wm doesn't use pango font.
