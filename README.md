# anova_rs
Command line interface for interacting with the WiFi controlled Anova Precision Cooker. Inspired by the Anova Culinary [developer-project-wifi](https://github.com/anova-culinary/developer-project-wifi).

**NOTE** - only APC devices are supported.

# Current State
This project is in very early stages (it barely runs). Before a `v0.0.1` release, the plan is to streamline the code and make it more robust. I would also like to make this project into a TUI, using [ratatui](https://ratatui.rs/). 


# Setup
Requires a .env file containing the Anova token. The token can be obtained via the [Anova Oven ](https://play.google.com/store/apps/details?id=com.anovaculinary.anovaoven) app.

```
ANOVA_TOKEN="anova-ey..."
```

# Usage
Before the release of `v0.0.1`, one has to build the binary locally:

```bash
cargo build --release
```
