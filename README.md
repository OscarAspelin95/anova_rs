# anova-rs

A TUI for monitoring and controlling one or multiple ANOVA precision cookers. Inspired by the original ANOVA [developer-project-wifi](https://github.com/anova-culinary/developer-project-wifi).

[![Built With Ratatui](https://img.shields.io/badge/Built_With_Ratatui-000?logo=ratatui&logoColor=fff)](https://ratatui.rs/)

## Usage
Until `v0.0.1` is released, build the binary with

```bash
cargo build --release
```

NOTE - requires an Anova token, obtained through the [Anova Oven](https://play.google.com/store/apps/details?id=com.anovaculinary.anovaoven) app. The easiest way is to use a `.env` file, which is automatically loaded on launch:

```
ANOVA_TOKEN="anova-ey........"
```

## License

Copyright (c) OscarAspelin <oscar.asp@hotmail.com>

This project is licensed under the MIT license ([LICENSE] or <http://opensource.org/licenses/MIT>)

[LICENSE]: ./LICENSE
