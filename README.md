Terminal OS

This project is a terminal based operating environment and multiplexer. It will be built in Rust. It functions similarly to tmux but extends the concept by baking in essential applications directly into the terminal experience. Users can manage persistent sessions, arrange windows, and configure automatic preloading of tools and scripts upon startup.

The core philosophy is to provide a unified command line desktop. Instead of relying on external terminal tools for everything, this workspace includes a native music player, a markdown editor, and a text based web browsing feature. This allows for a seamless workflow without ever leaving the terminal window or managing complex external dependencies for basic tasks.

The motivation behind this environment is the friction of constant context switching. While there are numerous standalone terminal utilities available for tasks like playing music or browsing the web, configuring and launching them individually for every session is tedious. This project solves that problem by integrating those disparate utilities into a single, cohesive interface where everything you need is available in one place immediately upon startup.