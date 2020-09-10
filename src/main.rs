[nisse@localhost hello-vulkan]$ cargo --help
Rust's package manager

USAGE:
    cargo [+toolchain] [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -V, --version           Print version info and exit
        --list              List installed commands
        --explain <CODE>    Run `rustc --explain CODE`
    -v, --verbose           Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet             No output printed to stdout
        --color <WHEN>      Coloring: auto, always, never
        --frozen            Require Cargo.lock and cache are up to date
        --locked            Require Cargo.lock is up to date
        --offline           Run without accessing the network
    -Z <FLAG>...            Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    -h, --help              Prints help information

Some common cargo commands are (see all commands with --list):
    build, b    Compile the current package
    check, c    Analyze the current package and report errors, but don't build object files
    clean       Remove the target directory
    doc         Build this package's and its dependencies' documentation
    new         Create a new cargo package
    init        Create a new cargo package in an existing directory
    run, r      Run a binary or example of the local package
    test, t     Run the tests
    bench       Run the benchmarks
    update      Update dependencies listed in Cargo.lock
    search      Search registry for crates
    publish     Package and upload this package to the registry
    install     Install a Rust binary. Default location is $HOME/.cargo/bin
    uninstall   Uninstall a Rust binary

See 'cargo help <command>' for more information on a specific command.

[nisse@localhost hello-vulkan]$ cargo doc --help
cargo-doc 
Build a package's documentation

USAGE:
    cargo doc [OPTIONS]

OPTIONS:
    -q, --quiet                                      No output printed to stdout
        --open                                       Opens the docs in a browser after the operation
    -p, --package <SPEC>...                          Package to document
        --all                                        Alias for --workspace (deprecated)
        --workspace                                  Document all packages in the workspace
        --exclude <SPEC>...                          Exclude packages from the build
        --no-deps                                    Don't build documentation for dependencies
        --document-private-items                     Document private items
    -j, --jobs <N>                                   Number of parallel jobs, defaults to # of CPUs
        --lib                                        Document only this package's library
        --bin <NAME>...                              Document only the specified binary
        --bins                                       Document all binaries
        --release                                    Build artifacts in release mode, with optimizations
        --profile <PROFILE-NAME>                     Build artifacts with the specified profile
        --features <FEATURES>...                     Space or comma separated list of features to activate
        --all-features                               Activate all available features
        --no-default-features                        Do not activate the `default` feature
        --target <Build for the target triple>...    TRIPLE
        --target-dir <DIRECTORY>                     Directory for all generated artifacts
        --manifest-path <PATH>                       Path to Cargo.toml
        --message-format <FMT>...                    Error format
    -v, --verbose                                    Use verbose output (-vv very verbose/build.rs output)
        --color <WHEN>                               Coloring: auto, always, never
        --frozen                                     Require Cargo.lock and cache are up to date
        --locked                                     Require Cargo.lock is up to date
        --offline                                    Run without accessing the network
    -Z <FLAG>...
            Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details

    -h, --help                                       Prints help information

By default the documentation for the local package and all dependencies is
built. The output is all placed in `target/doc` in rustdoc's usual format.

All packages in the workspace are documented if the `--workspace` flag is
supplied. The `--workspace` flag is automatically assumed for a virtual
manifest. Note that `--exclude` has to be specified in conjunction with the
`--workspace` flag.

If the `--package` argument is given, then SPEC is a package ID specification
which indicates which package should be documented. If it is not given, then the
current package is documented. For more information on SPEC and its format, see
the `cargo help pkgid` command.
[nisse@localhost hello-vulkan]$ cargo doc --open
warning: output filename collision.
The lib target `rand` in package `rand v0.7.3` has the same output filename as the lib target `rand` in package `rand v0.6.5`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_chacha` in package `rand_chacha v0.2.2` has the same output filename as the lib target `rand_chacha` in package `rand_chacha v0.1.1`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_chacha/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_core` in package `rand_core v0.4.2` has the same output filename as the lib target `rand_core` in package `rand_core v0.3.1`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_core/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_core` in package `rand_core v0.5.1` has the same output filename as the lib target `rand_core` in package `rand_core v0.4.2`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_core/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rusttype` in package `rusttype v0.8.3` has the same output filename as the lib target `rusttype` in package `rusttype v0.7.9`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rusttype/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
    Checking cfg-if v0.1.10
    Checking lazy_static v1.4.0
 Documenting cfg-if v0.1.10
    Checking rand_core v0.4.2
 Documenting lazy_static v1.4.0
    Checking scopeguard v1.1.0
    Checking slab v0.4.2
    Checking void v1.0.2
    Checking lazycell v1.3.0
    Checking smallvec v1.4.2
    Checking downcast-rs v1.2.0
 Documenting rand_core v0.4.2
    Checking same-file v1.0.6
 Documenting void v1.0.2
    Checking xdg v2.2.0
 Documenting slab v0.4.2
    Checking xml-rs v0.8.3
 Documenting scopeguard v1.1.0
 Documenting lazycell v1.3.0
    Checking unicode-xid v0.2.1
 Documenting same-file v1.0.6
 Documenting downcast-rs v1.2.0
 Documenting smallvec v1.4.2
    Checking ppv-lite86 v0.2.9
    Checking percent-encoding v2.1.0
    Checking half v1.6.0
    Checking fnv v1.0.7
 Documenting xml-rs v0.8.3
    Checking instant v0.1.6
    Checking vk-sys v0.5.2
 Documenting xdg v2.2.0
 Documenting unicode-xid v0.2.1
 Documenting percent-encoding v2.1.0
    Checking cc v1.0.59
 Documenting vk-sys v0.5.2
 Documenting instant v0.1.6
 Documenting ppv-lite86 v0.2.9
 Documenting fnv v1.0.7
 Documenting half v1.6.0
 Documenting cc v1.0.59
    Checking lock_api v0.3.4
    Checking walkdir v2.3.1
    Checking rand_core v0.3.1
    Checking rand_jitter v0.1.4
 Documenting lock_api v0.3.4
 Documenting rand_core v0.3.1
 Documenting rand_jitter v0.1.4
 Documenting walkdir v2.3.1
    Checking libc v0.2.76
 Documenting libc v0.2.76
    Checking bitflags v1.2.1
 Documenting bitflags v1.2.1
    Checking maybe-uninit v2.0.0
 Documenting maybe-uninit v2.0.0
    Checking libloading v0.6.3
 Documenting libloading v0.6.3
    Checking log v0.4.11
 Documenting log v0.4.11
    Checking byteorder v1.3.4
 Documenting byteorder v1.3.4
    Checking proc-macro2 v1.0.20
    Checking rand_isaac v0.1.1
    Checking rand_hc v0.1.0
    Checking rand_xorshift v0.1.1
 Documenting proc-macro2 v1.0.20
    Checking num-traits v0.2.12
 Documenting num-traits v0.2.12
    Checking cmake v0.1.44
    Checking dlib v0.4.2
    Checking crossbeam-utils v0.7.2
 Documenting crossbeam-utils v0.7.2
    Checking memoffset v0.5.5
 Documenting memoffset v0.5.5
    Checking rand_chacha v0.1.1
    Checking rand_pcg v0.1.2
 Documenting rand_pcg v0.1.2
 Documenting cmake v0.1.44
 Documenting rand_hc v0.1.0
 Documenting rand_xorshift v0.1.1
 Documenting rand_isaac v0.1.1
 Documenting rand_chacha v0.1.1
    Checking stb_truetype v0.3.1
    Checking net2 v0.2.35
    Checking iovec v0.1.4
    Checking nix v0.14.1
    Checking parking_lot_core v0.7.2
    Checking memmap v0.7.0
    Checking getrandom v0.1.14
    Checking shared_library v0.1.9
    Checking rand_os v0.1.3
    Checking x11-dl v2.18.5
    Checking raw-window-handle v0.3.3
    Checking wayland-sys v0.23.6
    Checking shaderc-sys v0.6.2
 Documenting dlib v0.4.2
    Checking quote v1.0.7
 Documenting stb_truetype v0.3.1
    Checking crossbeam-epoch v0.8.2
    Checking crossbeam-queue v0.2.3
    Checking crossbeam-channel v0.4.4
    Checking mio v0.6.22
    Checking parking_lot v0.10.2
    Checking rand_core v0.5.1
    Checking rand v0.6.5
 Documenting quote v1.0.7
    Checking shaderc v0.6.2
 Documenting crossbeam-epoch v0.8.2
 Documenting crossbeam-channel v0.4.4
 Documenting crossbeam-queue v0.2.3
    Checking approx v0.3.2
    Checking ordered-float v1.1.0
    Checking line_drawing v0.7.0
    Checking syn v1.0.40
    Checking rand_chacha v0.2.2
    Checking crossbeam-deque v0.7.3
    Checking rusttype v0.8.3
 Documenting iovec v0.1.4
 Documenting net2 v0.2.35
 Documenting nix v0.14.1
 Documenting parking_lot_core v0.7.2
 Documenting memmap v0.7.0
 Documenting getrandom v0.1.14
 Documenting shaderc-sys v0.6.2
 Documenting raw-window-handle v0.3.3
 Documenting rand_os v0.1.3
 Documenting shared_library v0.1.9
 Documenting x11-dl v2.18.5
 Documenting wayland-sys v0.23.6
 Documenting approx v0.3.2
 Documenting ordered-float v1.1.0
 Documenting line_drawing v0.7.0
    Checking crossbeam v0.7.3
    Checking rand v0.7.3
    Checking mio-extras v2.0.6
    Checking rusttype v0.7.9
 Documenting crossbeam-deque v0.7.3
 Documenting syn v1.0.40
    Checking cgmath v0.17.0
 Documenting parking_lot v0.10.2
 Documenting rand_core v0.5.1
    Checking vulkano v0.19.0
 Documenting shaderc v0.6.2
    Checking andrew v0.2.1
 Documenting rand v0.6.5
 Documenting mio v0.6.22
 Documenting rusttype v0.8.3
 Documenting crossbeam v0.7.3
 Documenting rand_chacha v0.2.2
 Documenting rusttype v0.7.9
 Documenting vulkano v0.19.0
 Documenting cgmath v0.17.0
 Documenting rand v0.7.3
 Documenting mio-extras v2.0.6
    Checking calloop v0.4.4
    Checking wayland-commons v0.23.6
    Checking wayland-client v0.23.6
 Documenting wayland-commons v0.23.6
 Documenting andrew v0.2.1
 Documenting calloop v0.4.4
 Documenting wayland-client v0.23.6
 Documenting vulkano-shaders v0.19.0
    Checking wayland-protocols v0.23.6
 Documenting wayland-protocols v0.23.6
    Checking smithay-client-toolkit v0.6.6
    Checking winit v0.22.2
 Documenting smithay-client-toolkit v0.6.6
    Checking vulkano-win v0.19.0
 Documenting winit v0.22.2
 Documenting vulkano-win v0.19.0
 Documenting hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
    Finished dev [unoptimized + debuginfo] target(s) in 25.79s
     Opening /home/nisse/Code/rust/hello-vulkan/target/doc/hello_vulkan/index.html
[nisse@localhost hello-vulkan]$ ls -la
total 72
drwxrwxr-x. 5 nisse nisse  4096 Sep  9 00:29 .
drwxrwxr-x. 3 nisse nisse  4096 Sep  7 22:04 ..
-rw-rw-r--. 1 nisse nisse 31892 Sep  9 00:27 Cargo.lock
-rw-rw-r--. 1 nisse nisse   396 Sep  9 00:29 Cargo.toml
-rw-rw-r--. 1 nisse nisse   380 Sep  8 18:52 Cargo.toml~
drwxrwxr-x. 7 nisse nisse  4096 Sep  9 22:55 .git
-rw-rw-r--. 1 nisse nisse    10 Sep  8 17:30 .gitignore
-rw-rw-r--. 1 nisse nisse     8 Sep  7 16:37 .gitignore~
-rw-rw-r--. 1 nisse nisse    61 Sep  7 23:18 README.md
drwxrwxr-x. 2 nisse nisse  4096 Sep  9 01:35 src
drwxrwxr-x. 5 nisse nisse  4096 Sep  9 22:58 target
[nisse@localhost hello-vulkan]$ ls -la target/
total 24
drwxrwxr-x.  5 nisse nisse 4096 Sep  9 22:58 .
drwxrwxr-x.  5 nisse nisse 4096 Sep  9 00:29 ..
drwxrwxr-x.  7 nisse nisse 4096 Sep  9 00:56 debug
drwxrwxr-x. 85 nisse nisse 4096 Sep  9 22:58 doc
-rw-rw-r--.  1 nisse nisse 1209 Sep  9 00:58 .rustc_info.json
drwxrwxr-x.  3 nisse nisse 4096 Sep  9 00:46 x86_64-pc-windows-gnu
[nisse@localhost hello-vulkan]$ ls -la target/doc/
total 5196
drwxrwxr-x. 85 nisse nisse    4096 Sep  9 22:58 .
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 ..
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 andrew
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 approx
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 bitflags
-rw-rw-r--.  1 nisse nisse     455 Sep  9 22:58 brush.svg
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 byteorder
drwxrwxr-x.  8 nisse nisse    4096 Sep  9 22:58 calloop
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 cc
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 cfg_if
drwxrwxr-x. 15 nisse nisse    4096 Sep  9 22:58 cgmath
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 cmake
-rw-rw-r--.  1 nisse nisse    1792 Sep  9 22:58 COPYRIGHT.txt
drwxrwxr-x. 11 nisse nisse    4096 Sep  9 22:58 crossbeam
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 crossbeam_channel
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 crossbeam_deque
drwxrwxr-x.  6 nisse nisse    4096 Sep  9 22:58 crossbeam_epoch
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 crossbeam_queue
drwxrwxr-x.  7 nisse nisse    4096 Sep  9 22:58 crossbeam_utils
-rw-rw-r--.  1 nisse nisse    8481 Sep  9 22:58 dark.css
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 dlib
-rw-rw-r--.  1 nisse nisse     510 Sep  9 22:58 down-arrow.svg
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 downcast_rs
-rw-rw-r--.  1 nisse nisse   23229 Sep  9 22:58 favicon.ico
-rw-rw-r--.  1 nisse nisse    4421 Sep  9 22:58 FiraSans-LICENSE.txt
-rw-rw-r--.  1 nisse nisse  186824 Sep  9 22:58 FiraSans-Medium.woff
-rw-rw-r--.  1 nisse nisse  183268 Sep  9 22:58 FiraSans-Regular.woff
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 fnv
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 getrandom
drwxrwxr-x.  7 nisse nisse    4096 Sep  9 22:58 half
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 hello_vulkan
drwxrwxr-x. 40 nisse nisse    4096 Sep  9 22:58 implementors
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 instant
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 iovec
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 lazycell
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 lazy_static
drwxrwxr-x.  4 nisse nisse  249856 Sep  9 22:58 libc
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 libloading
-rw-rw-r--.  1 nisse nisse   10847 Sep  9 22:58 LICENSE-APACHE.txt
-rw-rw-r--.  1 nisse nisse    1023 Sep  9 22:58 LICENSE-MIT.txt
-rw-rw-r--.  1 nisse nisse    8227 Sep  9 22:58 light.css
drwxrwxr-x. 11 nisse nisse    4096 Sep  9 22:58 line_drawing
-rwx------.  1 nisse nisse       0 Sep  9 22:58 .lock
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 lock_api
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 log
-rw-rw-r--.  1 nisse nisse   55458 Sep  9 22:58 main.js
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 maybe_uninit
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 memmap
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 memoffset
drwxrwxr-x. 10 nisse nisse    4096 Sep  9 22:58 mio
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 22:58 mio_extras
drwxrwxr-x.  6 nisse nisse    4096 Sep  9 22:58 net2
drwxrwxr-x. 17 nisse nisse    4096 Sep  9 22:58 nix
-rw-rw-r--.  1 nisse nisse    1995 Sep  9 22:58 normalize.css
-rw-rw-r--.  1 nisse nisse     151 Sep  9 22:58 noscript.css
drwxrwxr-x. 11 nisse nisse    4096 Sep  9 22:58 num_traits
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 ordered_float
drwxrwxr-x. 11 nisse nisse    4096 Sep  9 22:58 parking_lot
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 parking_lot_core
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 percent_encoding
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 22:58 ppv_lite86
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 proc_macro2
drwxrwxr-x.  5 nisse nisse    4096 Sep  9 22:58 quote
drwxrwxr-x.  8 nisse nisse    4096 Sep  9 22:58 rand
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 rand_chacha
drwxrwxr-x.  7 nisse nisse    4096 Sep  9 22:58 rand_core
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 rand_hc
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 22:58 rand_isaac
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 rand_jitter
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 rand_os
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 22:58 rand_pcg
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 rand_xorshift
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 raw_window_handle
-rw-rw-r--.  1 nisse nisse   20474 Sep  9 22:58 rustdoc.css
-rw-rw-r--.  1 nisse nisse    5758 Sep  9 22:58 rust-logo.png
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 rusttype
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 same_file
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 scopeguard
-rw-rw-r--.  1 nisse nisse 3694395 Sep  9 22:58 search-index.js
-rw-rw-r--.  1 nisse nisse     939 Sep  9 22:58 settings.css
-rw-rw-r--.  1 nisse nisse    4594 Sep  9 22:58 settings.html
-rw-rw-r--.  1 nisse nisse     589 Sep  9 22:58 settings.js
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 shaderc
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 shaderc_sys
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 shared_library
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 slab
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 smallvec
drwxrwxr-x. 12 nisse nisse    4096 Sep  9 22:58 smithay_client_toolkit
-rw-rw-r--.  1 nisse nisse    4528 Sep  9 22:58 SourceCodePro-LICENSE.txt
-rw-rw-r--.  1 nisse nisse   55472 Sep  9 22:58 SourceCodePro-Regular.woff
-rw-rw-r--.  1 nisse nisse   55360 Sep  9 22:58 SourceCodePro-Semibold.woff
-rw-rw-r--.  1 nisse nisse   15538 Sep  9 22:58 source-files.js
-rw-rw-r--.  1 nisse nisse    3235 Sep  9 22:58 source-script.js
-rw-rw-r--.  1 nisse nisse   93248 Sep  9 22:58 SourceSerifPro-Bold.ttf.woff
-rw-rw-r--.  1 nisse nisse   36200 Sep  9 22:58 SourceSerifPro-It.ttf.woff
-rw-rw-r--.  1 nisse nisse    4485 Sep  9 22:58 SourceSerifPro-LICENSE.md
-rw-rw-r--.  1 nisse nisse   88596 Sep  9 22:58 SourceSerifPro-Regular.ttf.woff
drwxrwxr-x. 83 nisse nisse    4096 Sep  9 22:58 src
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 stb_truetype
-rw-rw-r--.  1 nisse nisse    2132 Sep  9 22:58 storage.js
drwxrwxr-x. 24 nisse nisse   12288 Sep  9 22:58 syn
-rw-rw-r--.  1 nisse nisse    1174 Sep  9 22:58 theme.js
drwxrwxr-x.  3 nisse nisse    4096 Sep  9 22:58 unicode_xid
drwxrwxr-x.  2 nisse nisse   69632 Sep  9 22:58 vk_sys
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 void
drwxrwxr-x. 18 nisse nisse    4096 Sep  9 22:58 vulkano
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 vulkano_shaders
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 vulkano_win
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 22:58 walkdir
drwxrwxr-x. 13 nisse nisse    4096 Sep  9 22:58 wayland_client
drwxrwxr-x.  7 nisse nisse    4096 Sep  9 22:58 wayland_commons
drwxrwxr-x.  9 nisse nisse    4096 Sep  9 22:58 wayland_protocols
drwxrwxr-x.  7 nisse nisse    4096 Sep  9 22:58 wayland_sys
-rw-rw-r--.  1 nisse nisse    3764 Sep  9 22:58 wheel.svg
drwxrwxr-x. 11 nisse nisse    4096 Sep  9 22:58 winit
drwxrwxr-x. 23 nisse nisse    4096 Sep  9 22:58 x11_dl
drwxrwxr-x.  2 nisse nisse    4096 Sep  9 22:58 xdg
drwxrwxr-x. 10 nisse nisse    4096 Sep  9 22:58 xml
[nisse@localhost hello-vulkan]$ cargo doc --open
warning: output filename collision.
The lib target `rand` in package `rand v0.7.3` has the same output filename as the lib target `rand` in package `rand v0.6.5`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_chacha` in package `rand_chacha v0.2.2` has the same output filename as the lib target `rand_chacha` in package `rand_chacha v0.1.1`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_chacha/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_core` in package `rand_core v0.4.2` has the same output filename as the lib target `rand_core` in package `rand_core v0.3.1`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_core/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rand_core` in package `rand_core v0.5.1` has the same output filename as the lib target `rand_core` in package `rand_core v0.4.2`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rand_core/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
warning: output filename collision.
The lib target `rusttype` in package `rusttype v0.8.3` has the same output filename as the lib target `rusttype` in package `rusttype v0.7.9`.
Colliding filename is: /home/nisse/Code/rust/hello-vulkan/target/doc/rusttype/index.html
The targets should have unique names.
This is a known bug where multiple crates with the same name use
the same path; see <https://github.com/rust-lang/cargo/issues/6313>.
 Documenting rand_jitter v0.1.4
 Documenting rand_isaac v0.1.1
 Documenting rand_hc v0.1.0
 Documenting rand_xorshift v0.1.1
 Documenting rand_os v0.1.3
 Documenting rand_pcg v0.1.2
 Documenting rand v0.6.5
 Documenting cgmath v0.17.0
 Documenting hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
    Finished dev [unoptimized + debuginfo] target(s) in 8.31s
     Opening /home/nisse/Code/rust/hello-vulkan/target/doc/hello_vulkan/index.html
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected one of `;` or `where`, found keyword `struct`
  --> src/main.rs:27:1
   |
26 | struct Cord(f32, f32, f32)
   |                           - expected one of `;` or `where`
27 | struct Vertex(f32, f32, f32)
   | ^^^^^^ unexpected token

error: aborting due to previous error

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected `,`, or `}`, found `[`
  --> src/main.rs:36:22
   |
36 |     vertecies: Vertex[12],
   |                      ^

error: expected `:`, found `,`
  --> src/main.rs:48:45
   |
48 |     Create_Box {w: f32, d: f32, l: f32, Cord, thetaxy: f32, thetaxz: f32},
   |                                             ^ expected `:`

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:46:6
   |
46 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:48:5
   |
48 |     Create_Box {w: f32, d: f32, l: f32, Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error: aborting due to 2 previous errors; 3 warnings emitted

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:46:6
   |
46 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:48:5
   |
48 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

warning: struct is never constructed: `Cord`
  --> src/main.rs:26:8
   |
26 | struct Cord(f32, f32, f32);
   |        ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Vertex`
  --> src/main.rs:27:8
   |
27 | struct Vertex(f32, f32, f32);
   |        ^^^^^^

warning: struct is never constructed: `Box`
  --> src/main.rs:29:8
   |
29 | struct Box {
   |        ^^^

warning: associated function is never used: `volume`
  --> src/main.rs:40:8
   |
40 |     fn volume(&self) -> f32 {
   |        ^^^^^^

warning: enum is never used: `Debug_Action`
  --> src/main.rs:46:6
   |
46 | enum Debug_Action {
   |      ^^^^^^^^^^^^

warning: 8 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.55s
[nisse@localhost hello-vulkan]$ cargo run
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:46:6
   |
46 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:48:5
   |
48 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

warning: struct is never constructed: `Cord`
  --> src/main.rs:26:8
   |
26 | struct Cord(f32, f32, f32);
   |        ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: struct is never constructed: `Vertex`
  --> src/main.rs:27:8
   |
27 | struct Vertex(f32, f32, f32);
   |        ^^^^^^

warning: struct is never constructed: `Box`
  --> src/main.rs:29:8
   |
29 | struct Box {
   |        ^^^

warning: associated function is never used: `volume`
  --> src/main.rs:40:8
   |
40 |     fn volume(&self) -> f32 {
   |        ^^^^^^

warning: enum is never used: `Debug_Action`
  --> src/main.rs:46:6
   |
46 | enum Debug_Action {
   |      ^^^^^^^^^^^^

warning: 8 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/hello-vulkan`
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected item, found keyword `let`
  --> src/main.rs:45:1
   |
45 | let Cord0 = Cord(0.0, 0.0, 0.0);
   | ^^^ expected item

error: aborting due to previous error

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected item, found keyword `let`
  --> src/main.rs:45:1
   |
45 | let Cord0 = Cord(0.0, 0.0, 0.0);
   | ^^^ expected item

error: aborting due to previous error

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected item, found keyword `let`
  --> src/main.rs:47:1
   |
47 | let Box_def = Box {
   | ^^^ expected item

error: aborting due to previous error

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:47:6
   |
47 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:49:5
   |
49 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error[E0308]: mismatched types
  --> src/main.rs:59:8
   |
59 |     cord: (0.0, 0.0, 0.0),
   |           ^^^^^^^^^^^^^^^ expected struct `Cord`, found tuple
   |
   = note: expected struct `Cord`
               found tuple `({float}, {float}, {float})`

error[E0560]: struct `Box` has no field named `verticies`
  --> src/main.rs:62:2
   |
62 |     verticies: (0.1, 0.0, 0.0),    
   |     ^^^^^^^^^ help: a field with a similar name exists: `vertecies`

error[E0308]: mismatched types
  --> src/main.rs:66:5
   |
66 |     w: 2,
   |        ^
   |        |
   |        expected `f32`, found integer
   |        help: use a float literal: `2.0`

error[E0308]: mismatched types
  --> src/main.rs:67:5
   |
67 |     d: 1,
   |        ^
   |        |
   |        expected `f32`, found integer
   |        help: use a float literal: `1.0`

error[E0308]: mismatched types
  --> src/main.rs:68:5
   |
68 |     l: 1,
   |        ^
   |        |
   |        expected `f32`, found integer
   |        help: use a float literal: `1.0`

error: aborting due to 5 previous errors; 3 warnings emitted

Some errors have detailed explanations: E0308, E0560.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:47:6
   |
47 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:49:5
   |
49 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error[E0308]: mismatched types
  --> src/main.rs:59:8
   |
59 |     cord: (0.0, 0.0, 0.0),
   |           ^^^^^^^^^^^^^^^ expected struct `Cord`, found tuple
   |
   = note: expected struct `Cord`
               found tuple `({float}, {float}, {float})`

error[E0560]: struct `Box` has no field named `verticies`
  --> src/main.rs:62:2
   |
62 |     verticies: (0.1, 0.0, 0.0),    
   |     ^^^^^^^^^ help: a field with a similar name exists: `vertecies`

error: aborting due to 2 previous errors; 3 warnings emitted

Some errors have detailed explanations: E0308, E0560.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:47:6
   |
47 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:49:5
   |
49 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error[E0308]: mismatched types
  --> src/main.rs:59:8
   |
59 |     cord: (0.0, 0.0, 0.0),
   |           ^^^^^^^^^^^^^^^ expected struct `Cord`, found tuple
   |
   = note: expected struct `Cord`
               found tuple `({float}, {float}, {float})`

error[E0308]: mismatched types
  --> src/main.rs:62:15
   |
62 |     vertex_list: (0.1, 0.0, 0.0),    
   |                  ^^^^^^^^^^^^^^^ expected struct `Vertex1`, found tuple
   |
   = note: expected struct `Vertex1`
               found tuple `({float}, {float}, {float})`

error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0308`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:47:6
   |
47 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:49:5
   |
49 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

warning: field is never read: `cord`
  --> src/main.rs:33:5
   |
33 |     cord: Cord,
   |     ^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: field is never read: `thetaxy`
  --> src/main.rs:34:5
   |
34 |     thetaxy: f32,
   |     ^^^^^^^^^^^^

warning: field is never read: `thetaxz`
  --> src/main.rs:35:5
   |
35 |     thetaxz: f32,
   |     ^^^^^^^^^^^^

warning: field is never read: `vertex_list`
  --> src/main.rs:36:5
   |
36 |     vertex_list: Vertex1,
   |     ^^^^^^^^^^^^^^^^^^^^

warning: enum is never used: `Debug_Action`
  --> src/main.rs:47:6
   |
47 | enum Debug_Action {
   |      ^^^^^^^^^^^^

warning: variable `Box_def` should have a snake case name
  --> src/main.rs:58:9
   |
58 |     let Box_def = Box {
   |         ^^^^^^^ help: convert the identifier to snake case: `box_def`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: variable `Box1` should have a snake case name
  --> src/main.rs:68:9
   |
68 |     let Box1 = Box {
   |         ^^^^ help: convert the identifier to snake case: `box1`

warning: 10 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.15s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:48:6
   |
48 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:50:5
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error[E0277]: `Cord` doesn't implement `std::fmt::Debug`
  --> src/main.rs:50:41
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |                                         ^^^^^^^^^^ `Cord` cannot be formatted using `{:?}`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `Cord`
   = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
   = note: required because of the requirements on the impl of `std::fmt::Debug` for `&Cord`
   = note: required for the cast to the object type `dyn std::fmt::Debug`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `Debug_Action` doesn't implement `std::fmt::Display`
  --> src/main.rs:77:49
   |
77 |     println!("the color will be changed to {}", D_action1);
   |                                                 ^^^^^^^^^ `Debug_Action` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Debug_Action`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: required by `std::fmt::Display::fmt`
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0277`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:48:6
   |
48 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:50:5
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

error[E0277]: `Cord` doesn't implement `std::fmt::Debug`
  --> src/main.rs:50:41
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |                                         ^^^^^^^^^^ `Cord` cannot be formatted using `{:?}`
   |
   = help: the trait `std::fmt::Debug` is not implemented for `Cord`
   = note: add `#[derive(Debug)]` or manually implement `std::fmt::Debug`
   = note: required because of the requirements on the impl of `std::fmt::Debug` for `&Cord`
   = note: required for the cast to the object type `dyn std::fmt::Debug`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error; 3 warnings emitted

For more information about this error, try `rustc --explain E0277`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:50:6
   |
50 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:52:5
   |
52 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

warning: variant is never constructed: `Quit`
  --> src/main.rs:51:5
   |
51 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `Create_Box`
  --> src/main.rs:52:5
   |
52 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable `D_action1` should have a snake case name
  --> src/main.rs:58:9
   |
58 |     let D_action1 = Debug_Action::ChangeColor(50,50,30);
   |         ^^^^^^^^^ help: convert the identifier to snake case: `d_action1`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: variable `Box_def` should have a snake case name
  --> src/main.rs:62:9
   |
62 |     let Box_def = Box {
   |         ^^^^^^^ help: convert the identifier to snake case: `box_def`

warning: variable `Box1` should have a snake case name
  --> src/main.rs:72:9
   |
72 |     let Box1 = Box {
   |         ^^^^ help: convert the identifier to snake case: `box1`

warning: 8 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.06s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
the color will be changed to ChangeColor(
    50,
    50,
    30,
)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: type `Debug_Action` should have an upper camel case name
  --> src/main.rs:48:6
   |
48 | enum Debug_Action {
   |      ^^^^^^^^^^^^ help: convert the identifier to upper camel case: `DebugAction`
   |
   = note: `#[warn(non_camel_case_types)]` on by default

warning: variant `Create_Box` should have an upper camel case name
  --> src/main.rs:50:5
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^ help: convert the identifier to upper camel case: `CreateBox`

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `Create_Box`
  --> src/main.rs:50:5
   |
50 |     Create_Box {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: variable `D_action1` should have a snake case name
  --> src/main.rs:56:9
   |
56 |     let D_action1 = Debug_Action::ChangeColor(50,50,30);
   |         ^^^^^^^^^ help: convert the identifier to snake case: `d_action1`
   |
   = note: `#[warn(non_snake_case)]` on by default

warning: variable `Box_def` should have a snake case name
  --> src/main.rs:60:9
   |
60 |     let Box_def = Box {
   |         ^^^^^^^ help: convert the identifier to snake case: `box_def`

warning: variable `Box1` should have a snake case name
  --> src/main.rs:70:9
   |
70 |     let Box1 = Box {
   |         ^^^^ help: convert the identifier to snake case: `box1`

warning: 8 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.14s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
the color will be changed to ChangeColor(
    50,
    50,
    30,
)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error[E0433]: failed to resolve: use of undeclared type or module `Debug_Action`
  --> src/main.rs:56:21
   |
56 |     let D_action1 = Debug_Action::ChangeColor(50,50,30);
   |                     ^^^^^^^^^^^^ use of undeclared type or module `Debug_Action`

error[E0425]: cannot find value `Box_def` in this scope
  --> src/main.rs:74:4
   |
74 |     ..Box_def
   |       ^^^^^^^ help: a local variable with a similar name exists: `box_def`

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0425, E0433.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error[E0433]: failed to resolve: use of undeclared type or module `Debug_Action`
  --> src/main.rs:56:21
   |
56 |     let D_action1 = Debug_Action::ChangeColor(50,50,30);
   |                     ^^^^^^^^^^^^ use of undeclared type or module `Debug_Action`

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.22s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
the color will be changed to ChangeColor(
    50,
    50,
    30,
)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ git commit -a -m "experimenting with structs and enums"
[master b19e576] experimenting with structs and enums
 1 file changed, 55 insertions(+)
[nisse@localhost hello-vulkan]$ git push
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
Enumerating objects: 7, done.
Counting objects: 100% (7/7), done.
Delta compression using up to 16 threads
Compressing objects: 100% (3/3), done.
Writing objects: 100% (4/4), 781 bytes | 781.00 KiB/s, done.
Total 4 (delta 2), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (2/2), completed with 2 local objects.        
To https://github.com/nilssab/hello-vulkan.git
   31ff122..b19e576  master -> master
[nisse@localhost hello-vulkan]$ cd ..
[nisse@localhost rust]$ cd ..
[nisse@localhost Code]$ git clone https://github.com/nilssab/nissesettings
Cloning into 'nissesettings'...
remote: Enumerating objects: 31, done.        
remote: Total 31 (delta 0), reused 0 (delta 0), pack-reused 31        
Receiving objects: 100% (31/31), 16.22 KiB | 5.41 MiB/s, done.
Resolving deltas: 100% (8/8), done.
[nisse@localhost Code]$ cd nissesettings/
[nisse@localhost nissesettings]$ ls
emacs_other.el	LICENSE  README.md
[nisse@localhost nissesettings]$ ls -la
total 60
drwxrwxr-x. 3 nisse nisse  4096 Sep 10 00:17 .
drwxrwxr-x. 4 nisse nisse  4096 Sep 10 00:17 ..
-rw-rw-r--. 1 nisse nisse  1517 Sep 10 00:17 .emacs
-rw-rw-r--. 1 nisse nisse   520 Sep 10 00:17 emacs_other.el
drwxrwxr-x. 8 nisse nisse  4096 Sep 10 00:17 .git
-rw-rw-r--. 1 nisse nisse 35141 Sep 10 00:17 LICENSE
-rw-rw-r--. 1 nisse nisse   101 Sep 10 00:17 README.md
[nisse@localhost nissesettings]$ cp ~/.emacs ./
[nisse@localhost nissesettings]$ ls -la
total 60
drwxrwxr-x. 3 nisse nisse  4096 Sep 10 00:17 .
drwxrwxr-x. 4 nisse nisse  4096 Sep 10 00:17 ..
-rw-rw-r--. 1 nisse nisse  2446 Sep 10 00:18 .emacs
-rw-rw-r--. 1 nisse nisse   520 Sep 10 00:17 emacs_other.el
drwxrwxr-x. 8 nisse nisse  4096 Sep 10 00:17 .git
-rw-rw-r--. 1 nisse nisse 35141 Sep 10 00:17 LICENSE
-rw-rw-r--. 1 nisse nisse   101 Sep 10 00:17 README.md
[nisse@localhost nissesettings]$ ls ~/
Code  Desktop  Documents  Downloads  mountButterLocal  Music  orig2.txt  Pictures  Public  Templates  Videos
[nisse@localhost nissesettings]$ ls -la~/
ls: invalid option -- '~'
Try 'ls --help' for more information.
[nisse@localhost nissesettings]$ ls -la ~/
total 2412
drwx------. 26 nisse nisse    4096 Sep  9 23:32 .
drwxr-xr-x.  3 root  root     4096 Jul  2 17:29 ..
-rw-------.  1 nisse nisse    7963 Sep  9 02:03 .bash_history
-rw-r--r--.  1 nisse nisse      18 Jan 28  2020 .bash_logout
-rw-r--r--.  1 nisse nisse     179 Sep  9 00:49 .bash_profile
-rw-r--r--.  1 nisse nisse     376 Jan 28  2020 .bashrc
drwx------. 24 nisse nisse    4096 Aug  7 10:46 .cache
drwxrwxr-x.  4 nisse nisse    4096 Sep  9 00:49 .cargo
drwxrwxr-x.  3 nisse nisse    4096 Jul  2 17:42 .cinnamon
drwxrwxr-x.  4 nisse nisse    4096 Sep 10 00:17 Code
drwxr-xr-x. 30 nisse nisse    4096 Aug 18 18:04 .config
drwxr-xr-x.  2 nisse nisse    4096 Jul  2 17:42 Desktop
drwxr-xr-x. 13 nisse nisse    4096 Aug 26 00:51 Documents
drwxr-xr-x.  3 nisse nisse    4096 Aug 25 21:17 Downloads
-rw-rw-r--.  1 nisse nisse    2446 Sep  9 23:32 .emacs
-rw-rw-r--.  1 nisse nisse    2445 Sep  9 02:00 .emacs~
drwx------.  4 nisse nisse    4096 Sep  7 17:17 .emacs.d
-rw-------.  1 nisse nisse      16 Jul  2 17:42 .esd_auth
-rw-rw-r--.  1 nisse nisse     153 Sep  7 23:15 .gitconfig
-rw-rw-r--.  1 nisse nisse     165 Sep  7 23:14 .gitconfig~
drwx------.  3 nisse nisse    4096 Jul  3 16:43 .gnome
drwxr-xr-x.  2 nisse nisse    4096 Aug  7 21:31 .gphoto
drwx------.  3 nisse nisse    4096 Jul  2 17:42 .local
-rwxrwxr-x.  1 nisse nisse      60 Jul  3 16:01 mountButterLocal
drwxr-xr-x.  6 nisse nisse    4096 Jul  3 15:38 .mozilla
drwxr-xr-x. 18 nisse nisse    4096 Jul 12 18:17 Music
-rw-rw-r--.  1 nisse nisse 2291589 Jul  5 08:33 orig2.txt
drwxr-xr-x.  3 nisse nisse    4096 Aug 25 20:49 Pictures
drwx------.  3 nisse nisse    4096 Jul  3 16:43 .pki
-rw-rw-r--.  1 nisse nisse      38 Sep  9 00:49 .profile
drwxr-xr-x.  2 nisse nisse    4096 Jul  2 17:42 Public
drwxrwxr-x.  6 nisse nisse    4096 Sep  9 00:49 .rustup
drwx------.  2 nisse nisse    4096 Jul  3 16:17 .ssh
drwxrwxr-x.  2 nisse nisse    4096 Aug 17 15:18 .steam
lrwxrwxrwx.  1 nisse nisse      30 Aug 17 15:17 .steampath -> /home/nisse/.steam/sdk32/steam
lrwxrwxrwx.  1 nisse nisse      28 Aug 17 15:17 .steampid -> /home/nisse/.steam/steam.pid
drwxr-xr-x.  2 nisse nisse    4096 Jul  2 17:42 Templates
drwxrwxr-x.  3 nisse nisse    4096 Jul 20 23:02 .texlive2019
drwx------.  4 nisse nisse    4096 Jul 22 18:10 .thunderbird
drwxr-xr-x.  2 nisse nisse    4096 Jul  2 17:42 Videos
-rw-------.  1 nisse nisse     894 Jul  5 08:37 .viminfo
-rw-------.  1 nisse nisse    6429 Sep  9 23:04 .xsession-errors
-rw-------.  1 nisse nisse    6933 Sep  9 02:03 .xsession-errors.old
[nisse@localhost nissesettings]$ ls -la
total 60
drwxrwxr-x. 3 nisse nisse  4096 Sep 10 00:17 .
drwxrwxr-x. 4 nisse nisse  4096 Sep 10 00:17 ..
-rw-rw-r--. 1 nisse nisse  2446 Sep 10 00:18 .emacs
-rw-rw-r--. 1 nisse nisse   520 Sep 10 00:17 emacs_other.el
drwxrwxr-x. 8 nisse nisse  4096 Sep 10 00:17 .git
-rw-rw-r--. 1 nisse nisse 35141 Sep 10 00:17 LICENSE
-rw-rw-r--. 1 nisse nisse   101 Sep 10 00:17 README.md
[nisse@localhost nissesettings]$ cp -v ~/.emacs ./.emacs 
'/home/nisse/.emacs' -> './.emacs'
[nisse@localhost nissesettings]$ ls -la
total 60
drwxrwxr-x. 3 nisse nisse  4096 Sep 10 00:17 .
drwxrwxr-x. 4 nisse nisse  4096 Sep 10 00:17 ..
-rw-rw-r--. 1 nisse nisse  2446 Sep 10 00:19 .emacs
-rw-rw-r--. 1 nisse nisse   520 Sep 10 00:17 emacs_other.el
drwxrwxr-x. 8 nisse nisse  4096 Sep 10 00:17 .git
-rw-rw-r--. 1 nisse nisse 35141 Sep 10 00:17 LICENSE
-rw-rw-r--. 1 nisse nisse   101 Sep 10 00:17 README.md
[nisse@localhost nissesettings]$ cat .emacs 
;;interface related
(server-start)
(load-theme 'tango-dark)
(tool-bar-mode -1)
(menu-bar-mode -1)
(scroll-bar-mode -1)
(setq visible-bell 1)
(set-default 'truncate-lines t)

(require 'package)
;(add-to-list 'package-archives
;             '("melpa" . "https://melpa.org/packages/") t)
;(package-initialize)
;(package-refresh-contents)

(require 'rust-mode)

;;Note on some removed movement bindings
;this config removes most movement keys, since I have them on separate layers
;on my keyboard instead

;;Global shortcuts keymap related
(global-set-key (kbd "C-f") 'Control-X-prefix)
(global-set-key (kbd "C-z") 'undo)

(global-set-key (kbd "C-a") 'isearch-backward)
(global-set-key (kbd "C-j") 'yank)
(global-set-key (kbd "M-j") 'yank-pop)
(global-set-key (kbd "C-k") 'kill-region)
(global-set-key (kbd "M-k") 'copy-region-as-kill)
(global-set-key (kbd "C-l") 'kill-whole-line)
(global-set-key (kbd "M-l") 'kill-line)
(global-set-key (kbd "C-x") 'eval-defun)
(global-set-key (kbd "M-SPC") 'rectangle-mark-mode)

;open C- keys:b d e n r t v
;open M- keys:a b c d e f 
;open C-M- keys:

;buffer related
(global-set-key (kbd "C-b") 'switch-to-buffer)
(global-set-key (kbd "C-f b") 'buffer-menu)
(global-set-key (kbd "C-w") 'visual-line-mode);toggle linewrap
(global-set-key (kbd "C-y") 'recenter-top-bottom)

(global-set-key (kbd "<f5>") 'shell)
(global-set-key (kbd "<f6>") 'shell-command)

;window/frame management related	       
(global-set-key (kbd "<C-home>") 'enlarge-window-horizontally)
(global-set-key (kbd "<C-end>") 'shrink-window-horizontally)
(global-set-key (kbd "<C-prior>") 'enlarge-window)
(global-set-key (kbd "<C-next>") 'shrink-window)
(global-set-key (kbd "C-o") 'other-window)
(global-set-key (kbd "C-p") (kbd "C-u -1 C-o"))
(custom-set-variables
 ;; custom-set-variables was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 '(package-selected-packages '(rust-mode)))
(custom-set-faces
 ;; custom-set-faces was added by Custom.
 ;; If you edit it by hand, you could mess it up, so be careful.
 ;; Your init file should contain only one such instance.
 ;; If there is more than one, they won't work right.
 )
(find-file '"~/Code/rust/hello-vulkan/src/main.rs")
(split-window-right)
(other-window -1)
(find-file '"~/Code/rust/hello-vulkan/Cargo.toml")
(split-window-below)
(shell)

[nisse@localhost nissesettings]$ git commit -a -m "added settings for Rust and current Rust Project"
[master 818a276] added settings for Rust and current Rust Project
 1 file changed, 27 insertions(+)
[nisse@localhost nissesettings]$ git push
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
Enumerating objects: 5, done.
Counting objects: 100% (5/5), done.
Delta compression using up to 16 threads
Compressing objects: 100% (3/3), done.
Writing objects: 100% (3/3), 827 bytes | 827.00 KiB/s, done.
Total 3 (delta 1), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (1/1), completed with 1 local object.        
To https://github.com/nilssab/nissesettings
   0b2cb73..818a276  master -> master
[nisse@localhost nissesettings]$ cd ..
[nisse@localhost Code]$ cd rust/hello-vulkan/
[nisse@localhost hello-vulkan]$ cargo run
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
the color will be changed to ChangeColor(
    50,
    50,
    30,
)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.09s
     Running `target/debug/hello-vulkan`
Box1 Volume is 2
the waiting Debug Action is ChangeColor(50, 50, 30)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.03s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo build
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected type, found keyword `enum`
  --> src/main.rs:38:18
   |
38 |     vertex_list: enum Option<Vertex1> {
   |                  ^^^^ expected type

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.18s
[nisse@localhost hello-vulkan]$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'
info: checking for self-updates

     stable-x86_64-pc-windows-gnu unchanged - (rustc does not exist)
  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.46.0 (04488afe3 2020-08-24)

info: cleaning up downloads & tmp directories
[nisse@localhost hello-vulkan]$ rustup add component clippy
error: error: Found argument 'clippy' which wasn't expected, or isn't valid in this context

USAGE:
    rustup component <SUBCOMMAND>

For more information try --help

[nisse@localhost hello-vulkan]$ rustup component add clippy
info: component 'clippy' for target 'x86_64-unknown-linux-gnu' is up to date
[nisse@localhost hello-vulkan]$ cargo clippy
    Checking hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: missing `in` in `for` loop
  --> src/main.rs:93:6
   |
93 |     );
   |      ^ help: try adding `in` here

error: expected expression, found `;`
  --> src/main.rs:93:6
   |
93 |     );
   |      ^ expected expression

warning: unused imports: `BufferUsage`, `CpuAccessibleBuffer`
 --> src/main.rs:1:23
  |
1 | use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
  |                       ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused import: `AutoCommandBufferBuilder`
 --> src/main.rs:2:31
  |
2 | use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
  |                               ^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `DeviceExtensions`, `Device`
 --> src/main.rs:3:23
  |
3 | use vulkano::device::{Device, DeviceExtensions};
  |                       ^^^^^^  ^^^^^^^^^^^^^^^^

warning: unused import: `Subpass`
 --> src/main.rs:4:82
  |
4 | use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
  |                                                                                  ^^^^^^^

warning: unused import: `ImageUsage`
 --> src/main.rs:5:22
  |
5 | use vulkano::image::{ImageUsage, SwapchainImage};
  |                      ^^^^^^^^^^

warning: unused import: `vulkano::pipeline::GraphicsPipeline`
 --> src/main.rs:8:5
  |
8 | use vulkano::pipeline::GraphicsPipeline;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `vulkano::swapchain`
 --> src/main.rs:9:5
  |
9 | use vulkano::swapchain;
  |     ^^^^^^^^^^^^^^^^^^

warning: unused imports: `AcquireError`, `ColorSpace`, `FullscreenExclusive`, `PresentMode`, `SurfaceTransform`, `SwapchainCreationError`, `Swapchain`
  --> src/main.rs:11:5
   |
11 |     AcquireError, ColorSpace, FullscreenExclusive, PresentMode, SurfaceTransform, Swapchain,
   |     ^^^^^^^^^^^^  ^^^^^^^^^^  ^^^^^^^^^^^^^^^^^^^  ^^^^^^^^^^^  ^^^^^^^^^^^^^^^^  ^^^^^^^^^
12 |     SwapchainCreationError,
   |     ^^^^^^^^^^^^^^^^^^^^^^

warning: unused import: `vulkano::sync`
  --> src/main.rs:14:5
   |
14 | use vulkano::sync;
   |     ^^^^^^^^^^^^^

warning: unused imports: `FlushError`, `GpuFuture`
  --> src/main.rs:15:21
   |
15 | use vulkano::sync::{FlushError, GpuFuture};
   |                     ^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `vulkano_win::VkSurfaceBuild`
  --> src/main.rs:17:5
   |
17 | use vulkano_win::VkSurfaceBuild;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: unused imports: `Event`, `WindowEvent`
  --> src/main.rs:18:20
   |
18 | use winit::event::{Event, WindowEvent};
   |                    ^^^^^  ^^^^^^^^^^^

warning: unused imports: `ControlFlow`, `EventLoop`
  --> src/main.rs:19:25
   |
19 | use winit::event_loop::{ControlFlow, EventLoop};
   |                         ^^^^^^^^^^^  ^^^^^^^^^

warning: unused import: `WindowBuilder`
  --> src/main.rs:20:29
   |
20 | use winit::window::{Window, WindowBuilder};
   |                             ^^^^^^^^^^^^^

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^

error: aborting due to 2 previous errors; 15 warnings emitted

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo clippy
    Checking hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.53s
[nisse@localhost hello-vulkan]$ ccccccccccccargo build
bash: ccccccccccccargo: command not found
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected `;`, found keyword `for`
  --> src/main.rs:86:22
   |
86 |     let device_nr = 1
   |                      ^ help: add `;` here
87 |     for device in PhysicalDevice::enumerate(&instance){
   |     --- unexpected token

error: expected identifier, found keyword `type`
  --> src/main.rs:92:13
   |
92 |         device.type()
   |                ^^^^ expected identifier, found keyword
   |
help: you can escape reserved keywords to use them as identifiers
   |
92 |         device.r#type()
   |                ^^^^^^

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0599]: no method named `r#type` found for struct `vulkano::instance::PhysicalDevice<'_>` in the current scope
  --> src/main.rs:92:13
   |
92 |         device.type()
   |                ^^^^ method not found in `vulkano::instance::PhysicalDevice<'_>`

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected `;`, found keyword `for`
  --> src/main.rs:86:22
   |
86 |     let device_nr = 1
   |                      ^ help: add `;` here
87 |     for device in PhysicalDevice::enumerate(&instance){
   |     --- unexpected token

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo build
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `device_nr`
  --> src/main.rs:94:6
   |
94 |     let device_nr = device_nr + 1;
   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_device_nr`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 4 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 5.00s
[nisse@localhost hello-vulkan]$ cargo run
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `device_nr`
  --> src/main.rs:94:6
   |
94 |     let device_nr = device_nr + 1;
   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_device_nr`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 4 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cat Cargo.lock | grep wrap
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `device_nr`
  --> src/main.rs:93:6
   |
93 |     let device_nr = device_nr + 1;
   |         ^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_device_nr`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 4 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 5.05s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 2.51s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.79s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: IncompatibleVertexDefinition(FormatMismatch { attribute: "position", shader: (R32G32Sfloat, 1), definition: (F32, 3) })', src/main.rs:299:14
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: proc macro panicked
   --> src/main.rs:210:9
    |
210 | /         vulkano_shaders::shader! {
211 | |             ty: "vertex",
212 | |             src: "
213 | |                 #version 450
...   |
218 | |             "
219 | |         }
    | |_________^
    |
    = help: message: 2 compilation errors:
            shader.glsl:5: error: 'constructor' : too many arguments
            shader.glsl:5: error: 'assign' :  cannot convert from ' const float' to ' gl_Position 4-component vector of float Position'
            

error[E0433]: failed to resolve: could not find `Shader` in `vs`
   --> src/main.rs:235:18
    |
235 |     let vs = vs::Shader::load(device.clone()).unwrap();
    |                  ^^^^^^ could not find `Shader` in `vs`

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to 2 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0433`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.83s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0599]: no variant or associated item named `Discard` found for enum `vulkano::framebuffer::StoreOp` in the current scope
   --> src/main.rs:256:28
    |
256 |                     store: Discard,
    |                            ^^^^^^^ variant or associated item not found in `vulkano::framebuffer::StoreOp`

error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0599`.
error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.84s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.90s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
test
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.92s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
Killed
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `;`
  --> src/main.rs:28:19
   |
28 | struct Vertex1(f32; 3);
   |                   ^ expected one of 7 possible tokens

error: expected one of `!`, `#`, `&&`, `&`, `(`, `)`, `*`, `...`, `;`, `<`, `?`, `[`, `_`, `async`, `const`, `crate`, `dyn`, `extern`, `fn`, `for`, `impl`, `pub`, `unsafe`, `where`, lifetime, or path, found `3`
  --> src/main.rs:28:21
   |
28 | struct Vertex1(f32; 3);
   |                     ^ expected one of 26 possible tokens

error: aborting due to 2 previous errors

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `;`
  --> src/main.rs:28:19
   |
28 | struct Vertex1(f32; f32, f32);
   |                   ^
   |                   |
   |                   expected one of 7 possible tokens
   |                   help: missing `,`

warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to previous error; 1 warning emitted

error: could not compile `hello-vulkan`.

To learn more, run the command again with --verbose.
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.83s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 4.86s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ cargo run
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 7.02s
     Running `target/debug/hello-vulkan`
Box1 Volume is 4.5
the waiting Debug Action is ChangeColor(50, 50, 30)
Device 1: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu
Using device: AMD RADV RENOIR (LLVM 10.0.1) (type: IntegratedGpu)
[nisse@localhost hello-vulkan]$ git commit -a -m "trying out enums etc."
[master a34711f] trying out enums etc.
 1 file changed, 57 insertions(+), 125 deletions(-)
[nisse@localhost hello-vulkan]$ git push
Username for 'https://github.com': Pajgithub1
Password for 'https://Pajgithub1@github.com': 
remote: Invalid username or password.
fatal: Authentication failed for 'https://github.com/nilssab/hello-vulkan.git/'
[nisse@localhost hello-vulkan]$ git push
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
Enumerating objects: 7, done.
Counting objects: 100% (7/7), done.
Delta compression using up to 16 threads
Compressing objects: 100% (3/3), done.
Writing objects: 100% (4/4), 1.23 KiB | 1.23 MiB/s, done.
Total 4 (delta 2), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (2/2), completed with 2 local objects.        
To https://github.com/nilssab/hello-vulkan.git
   b19e576..a34711f  master -> master
[nisse@localhost hello-vulkan]$ cargo update
    Updating crates.io index
    Updating getrandom v0.1.14 -> v0.1.15
    Updating libc v0.2.76 -> v0.2.77
    Updating proc-macro2 v1.0.20 -> v1.0.21
[nisse@localhost hello-vulkan]$ cargo build
 Downloading crates ...
  Downloaded proc-macro2 v1.0.21
  Downloaded libc v0.2.77
  Downloaded getrandom v0.1.15
   Compiling libc v0.2.77
   Compiling proc-macro2 v1.0.21
   Compiling getrandom v0.1.15
   Compiling quote v1.0.7
   Compiling net2 v0.2.35
   Compiling iovec v0.1.4
   Compiling nix v0.14.1
   Compiling parking_lot_core v0.7.2
   Compiling memmap v0.7.0
   Compiling shaderc-sys v0.6.2
   Compiling rand_os v0.1.3
   Compiling shared_library v0.1.9
   Compiling raw-window-handle v0.3.3
   Compiling x11-dl v2.18.5
   Compiling shaderc v0.6.2
   Compiling vulkano v0.19.0
   Compiling rand v0.6.5
   Compiling rand_core v0.5.1
   Compiling syn v1.0.40
   Compiling parking_lot v0.10.2
   Compiling mio v0.6.22
   Compiling rand_chacha v0.2.2
   Compiling rand v0.7.3
   Compiling mio-extras v2.0.6
   Compiling cgmath v0.17.0
   Compiling calloop v0.4.4
   Compiling wayland-commons v0.23.6
   Compiling wayland-client v0.23.6
   Compiling vulkano-shaders v0.19.0
   Compiling wayland-protocols v0.23.6
   Compiling smithay-client-toolkit v0.6.6
   Compiling winit v0.22.2
   Compiling vulkano-win v0.19.0
   Compiling hello-vulkan v0.1.1 (/home/nisse/Code/rust/hello-vulkan)
warning: unused import: `rand::Rng`
  --> src/main.rs:23:5
   |
23 | use rand::Rng;
   |     ^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: variant is never constructed: `Quit`
  --> src/main.rs:49:5
   |
49 |     Quit,
   |     ^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: variant is never constructed: `CreateBox`
  --> src/main.rs:50:5
   |
50 |     CreateBox {w: f32, d: f32, l: f32, cord: Cord, thetaxy: f32, thetaxz: f32},
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 26.79s
[nisse@localhost hello-vulkan]$ git pull
remote: Enumerating objects: 5, done.        
remote: Counting objects: 100% (5/5), done.        
remote: Compressing objects: 100% (3/3), done.        
remote: Total 3 (delta 1), reused 0 (delta 0), pack-reused 0        
Unpacking objects: 100% (3/3), 767 bytes | 767.00 KiB/s, done.
From https://github.com/nilssab/hello-vulkan
   a34711f..e5075b7  master     -> origin/master
Updating a34711f..e5075b7
Fast-forward
 README.md | 2 ++
 1 file changed, 2 insertions(+)
[nisse@localhost hello-vulkan]$ git commit -a -m "updated crates and README"
[master fb37516] updated crates and README
 2 files changed, 17 insertions(+), 13 deletions(-)
[nisse@localhost hello-vulkan]$ git push
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
Enumerating objects: 7, done.
Counting objects: 100% (7/7), done.
Delta compression using up to 16 threads
Compressing objects: 100% (4/4), done.
Writing objects: 100% (4/4), 895 bytes | 895.00 KiB/s, done.
Total 4 (delta 1), reused 0 (delta 0), pack-reused 0
remote: Resolving deltas: 100% (1/1), completed with 1 local object.        
To https://github.com/nilssab/hello-vulkan.git
   e5075b7..fb37516  master -> master
[nisse@localhost hello-vulkan]$ cd ..
[nisse@localhost rust]$ cargo new fft-denoise
     Created binary (application) `fft-denoise` package
[nisse@localhost rust]$ cd fft-denoise/
[nisse@localhost fft-denoise]$ cargo build
    Updating crates.io index
 Downloading crates ...
  Downloaded strength_reduce v0.2.3
  Downloaded rustfft v3.0.1
  Downloaded num-complex v0.2.4
  Downloaded num-integer v0.1.43
  Downloaded transpose v0.1.0
   Compiling autocfg v1.0.1
   Compiling transpose v0.1.0
   Compiling strength_reduce v0.2.3
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling num-complex v0.2.4
   Compiling rustfft v3.0.1
   Compiling fft-denoise v0.1.0 (/home/nisse/Code/rust/fft-denoise)
    Finished dev [unoptimized + debuginfo] target(s) in 6.15s
[nisse@localhost fft-denoise]$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/fft-denoise`
Hello, world!
[nisse@localhost fft-denoise]$ ls -lah target/
total 20K
drwxrwxr-x. 3 nisse nisse 4.0K Sep 11 00:55 .
drwxrwxr-x. 5 nisse nisse 4.0K Sep 11 00:55 ..
-rw-rw-r--. 1 nisse nisse  176 Sep 11 00:55 CACHEDIR.TAG
drwxrwxr-x. 7 nisse nisse 4.0K Sep 11 00:55 debug
-rw-rw-r--. 1 nisse nisse  771 Sep 11 00:55 .rustc_info.json
[nisse@localhost fft-denoise]$ ls -lah target/debug/
total 2.8M
drwxrwxr-x.  7 nisse nisse 4.0K Sep 11 00:55 .
drwxrwxr-x.  3 nisse nisse 4.0K Sep 11 00:55 ..
drwxrwxr-x.  8 nisse nisse 4.0K Sep 11 00:55 build
-rw-rw-r--.  1 nisse nisse    0 Sep 11 00:55 .cargo-lock
drwxrwxr-x.  2 nisse nisse 4.0K Sep 11 00:55 deps
drwxrwxr-x.  2 nisse nisse 4.0K Sep 11 00:55 examples
-rwxrwxr-x.  2 nisse nisse 2.8M Sep 11 00:55 fft-denoise
-rw-rw-r--.  1 nisse nisse  106 Sep 11 00:55 fft-denoise.d
drwxrwxr-x. 16 nisse nisse 4.0K Sep 11 00:55 .fingerprint
drwxrwxr-x.  3 nisse nisse 4.0K Sep 11 00:55 incremental
[nisse@localhost fft-denoise]$ git --help
usage: git [--version] [--help] [-C <path>] [-c <name>=<value>]
           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]
           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]
           <command> [<args>]

These are common Git commands used in various situations:

start a working area (see also: git help tutorial)
   clone             Clone a repository into a new directory
   init              Create an empty Git repository or reinitialize an existing one

work on the current change (see also: git help everyday)
   add               Add file contents to the index
   mv                Move or rename a file, a directory, or a symlink
   restore           Restore working tree files
   rm                Remove files from the working tree and from the index
   sparse-checkout   Initialize and modify the sparse-checkout

examine the history and state (see also: git help revisions)
   bisect            Use binary search to find the commit that introduced a bug
   diff              Show changes between commits, commit and working tree, etc
   grep              Print lines matching a pattern
   log               Show commit logs
   show              Show various types of objects
   status            Show the working tree status

grow, mark and tweak your common history
   branch            List, create, or delete branches
   commit            Record changes to the repository
   merge             Join two or more development histories together
   rebase            Reapply commits on top of another base tip
   reset             Reset current HEAD to the specified state
   switch            Switch branches
   tag               Create, list, delete or verify a tag object signed with GPG

collaborate (see also: git help workflows)
   fetch             Download objects and refs from another repository
   pull              Fetch from and integrate with another repository or a local branch
   push              Update remote refs along with associated objects

'git help -a' and 'git help -g' list available subcommands and some
concept guides. See 'git help <command>' or 'git help <concept>'
to read about a specific subcommand or concept.
See 'git help git' for an overview of the system.
[nisse@localhost fft-denoise]$ git init --help
WARNING: terminal is not fully functional
-  (press RETURN)
GIT-INIT(1)                                           Git Manual                                          GIT-INIT(1)

NAME
       git-init - Create an empty Git repository or reinitialize an existing one

SYNOPSIS
       git init [-q | --quiet] [--bare] [--template=<template_directory>]
                 [--separate-git-dir <git dir>]
                 [--shared[=<permissions>]] [directory]

DESCRIPTION
       This command creates an empty Git repository - basically a .git directory with subdirectories for objects,
       refs/heads, refs/tags, and template files. An initial HEAD file that references the HEAD of the master branch
       is also created.

       If the $GIT_DIR environment variable is set then it specifies a path to use instead of ./.git for the base of
       the repository.

       If the object storage directory is specified via the $GIT_OBJECT_DIRECTORY environment variable then the sha1
       directories are created underneath - otherwise the default $GIT_DIR/objects directory is used.

       Running git init in an existing repository is safe. It will not overwrite things that are already there. The
       primary reason for rerunning git init is to pick up newly added templates (or to move the repository to
       another place if --separate-git-dir is given).

OPTIONS
       -q, --quiet
           Only print error and warning messages; all other output will be suppressed.

       --bare
           Create a bare repository. If GIT_DIR environment is not set, it is set to the current working directory.

       --template=<template_directory>
 Manual page git-init(1) line 1 (press h for help or q to quit)q
[nisse@localhost fft-denoise]$ [nisse@localhost fft-denoise]$ git init
Reinitialized existing Git repository in /home/nisse/Code/rust/fft-denoise/.git/
[nisse@localhost fft-denoise]$ cat .git/config 
[core]
	bare = false
	repositoryformatversion = 0
	filemode = true
	logallrefupdates = true
[nisse@localhost fft-denoise]$ cat .git/HEAD 
ref: refs/heads/master
[nisse@localhost fft-denoise]$ git push
fatal: No configured push destination.
Either specify the URL from the command-line or configure a remote repository using

    git remote add <name> <url>

and then push using the remote name

    git push <name>

[nisse@localhost fft-denoise]$ git push --help
WARNING: terminal is not fully functional
-  (press RETURN)
GIT-PUSH(1)                                           Git Manual                                          GIT-PUSH(1)

NAME
       git-push - Update remote refs along with associated objects

SYNOPSIS
       git push [--all | --mirror | --tags] [--follow-tags] [--atomic] [-n | --dry-run] [--receive-pack=<git-receive-pack>]
                  [--repo=<repository>] [-f | --force] [-d | --delete] [--prune] [-v | --verbose]
                  [-u | --set-upstream] [-o <string> | --push-option=<string>]
                  [--[no-]signed|--signed=(true|false|if-asked)]
                  [--force-with-lease[=<refname>[:<expect>]]]
                  [--no-verify] [<repository> [<refspec>...]]

DESCRIPTION
       Updates remote refs using local refs, while sending objects necessary to complete the given refs.

       You can make interesting things happen to a repository every time you push into it, by setting up hooks there.
       See documentation for git-receive-pack(1).

       When the command line does not specify where to push with the <repository> argument, branch.*.remote
       configuration for the current branch is consulted to determine where to push. If the configuration is missing,
       it defaults to origin.

       When the command line does not specify what to push with <refspec>... arguments or --all, --mirror, --tags
       options, the command finds the default <refspec> by consulting remote.*.push configuration, and if it is not
       found, honors push.default configuration to decide what to push (See git-config(1) for the meaning of
       push.default).
q
       When neither the command-line nor the configuration specify what to push, the default behavior is used, which
       corresponds to the simple value for push.default: the current branch is pushed to the corresponding upstream
       branch, but as a safety measure, the push is aborted if the upstream branch does not have the same name as the
       local one.
 Manual page git-push(1) line 1 (press h for help or q to quit)q
[nisse@localhost fft-denoise]$ [nisse@localhost fft-denoise]$ git add src/main.rs
[nisse@localhost fft-denoise]$ git add Cargo.toml
[nisse@localhost fft-denoise]$ commit -a -m "initial commit"
bash: commit: command not found
[nisse@localhost fft-denoise]$ git commit -a -m "initial commit"
[master (root-commit) 90ba664] initial commit
 2 files changed, 18 insertions(+)
 create mode 100644 Cargo.toml
 create mode 100644 src/main.rs
[nisse@localhost fft-denoise]$ git push https://github.com/nilssab/fft-denoise
fatal: The current branch master has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream https://github.com/nilssab/fft-denoise master

[nisse@localhost fft-denoise]$ git push --set-upstream https://github.com/nilssab/fft-denoise master
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
To https://github.com/nilssab/fft-denoise
 ! [rejected]        master -> master (fetch first)
error: failed to push some refs to 'https://github.com/nilssab/fft-denoise'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing
hint: to the same ref. You may want to first integrate the remote changes
hint: (e.g., 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
[nisse@localhost fft-denoise]$ git pull
There is no tracking information for the current branch.
Please specify which branch you want to merge with.
See git-pull(1) for details.

    git pull <remote> <branch>

If you wish to set tracking information for this branch you can do so with:

    git branch --set-upstream-to=<remote>/<branch> master

[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise master
remote: Enumerating objects: 5, done.        
remote: Counting objects: 100% (5/5), done.        
remote: Compressing objects: 100% (5/5), done.        
remote: Total 5 (delta 0), reused 0 (delta 0), pack-reused 0        
Unpacking objects: 100% (5/5), 1.51 KiB | 1.51 MiB/s, done.
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
fatal: refusing to merge unrelated histories
[nisse@localhost fft-denoise]$ git merge https://github.com/nilssab/fft-denoise
merge: https://github.com/nilssab/fft-denoise - not something we can merge
[nisse@localhost fft-denoise]$ git push --set-origin https://github.com/nilssab/fft-denoise master
error: unknown option `set-origin'
usage: git push [<options>] [<repository> [<refspec>...]]

    -v, --verbose         be more verbose
    -q, --quiet           be more quiet
    --repo <repository>   repository
    --all                 push all refs
    --mirror              mirror all refs
    -d, --delete          delete refs
    --tags                push tags (can't be used with --all or --mirror)
    -n, --dry-run         dry run
    --porcelain           machine-readable output
    -f, --force           force updates
    --force-with-lease[=<refname>:<expect>]
                          require old value of ref to be at this value
    --recurse-submodules[=(check|on-demand|no)]
                          control recursive pushing of submodules
    --thin                use thin pack
    --receive-pack <receive-pack>
                          receive pack program
    --exec <receive-pack>
                          receive pack program
    -u, --set-upstream    set upstream for git pull/status
    --progress            force progress reporting
    --prune               prune locally removed refs
    --no-verify           bypass pre-push hook
    --follow-tags         push missing but relevant tags
    --signed[=(yes|no|if-asked)]
                          GPG sign the push
    --atomic              request atomic transaction on remote side
    -o, --push-option <server-specific>
                          option to transmit
    -4, --ipv4            use IPv4 addresses only
    -6, --ipv6            use IPv6 addresses only

[nisse@localhost fft-denoise]$ git push --repo https://github.com/nilssab/fft-denoise
fatal: The current branch master has no upstream branch.
To push the current branch and set the remote as upstream, use

    git push --set-upstream https://github.com/nilssab/fft-denoise master

[nisse@localhost fft-denoise]$ git branch
WARNING: terminal is not fully functional
-  (press RETURN)
* master
[nisse@localhost fft-denoise]$ git branch test
[nisse@localhost fft-denoise]$ git push --set-upstream https://github.com/nilssab/fft-denoise master
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
To https://github.com/nilssab/fft-denoise
 ! [rejected]        master -> master (non-fast-forward)
error: failed to push some refs to 'https://github.com/nilssab/fft-denoise'
hint: Updates were rejected because the tip of your current branch is behind
hint: its remote counterpart. Integrate the remote changes (e.g.
hint: 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
[nisse@localhost fft-denoise]$ git push --set-upstream https://github.com/nilssab/fft-denoise test
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
Enumerating objects: 5, done.
Counting objects: 100% (5/5), done.
Delta compression using up to 16 threads
Compressing objects: 100% (4/4), done.
Writing objects: 100% (5/5), 585 bytes | 585.00 KiB/s, done.
Total 5 (delta 0), reused 0 (delta 0), pack-reused 0
remote: 
remote: Create a pull request for 'test' on GitHub by visiting:        
remote:      https://github.com/nilssab/fft-denoise/pull/new/test        
remote: 
To https://github.com/nilssab/fft-denoise
 * [new branch]      test -> test
Branch 'test' set up to track remote branch 'test' from 'https://github.com/nilssab/fft-denoise'.
[nisse@localhost fft-denoise]$ git pull 
There is no tracking information for the current branch.
Please specify which branch you want to merge with.
See git-pull(1) for details.

    git pull <remote> <branch>

If you wish to set tracking information for this branch you can do so with:

    git branch --set-upstream-to=<remote>/<branch> master

[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise master
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
fatal: refusing to merge unrelated histories
[nisse@localhost fft-denoise]$ git fetch https://github.com/nilssab/fft-denoise
From https://github.com/nilssab/fft-denoise
 * branch            HEAD       -> FETCH_HEAD
[nisse@localhost fft-denoise]$ ls -la
total 36
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 00:55 .
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 00:46 ..
-rw-rw-r--. 1 nisse nisse 1756 Sep 11 00:55 Cargo.lock
-rw-rw-r--. 1 nisse nisse  311 Sep 11 00:55 Cargo.toml
-rw-rw-r--. 1 nisse nisse  230 Sep 11 00:46 Cargo.toml~
drwxrwxr-x. 8 nisse nisse 4096 Sep 11 01:17 .git
-rw-rw-r--. 1 nisse nisse    8 Sep 11 00:46 .gitignore
drwxrwxr-x. 2 nisse nisse 4096 Sep 11 01:14 src
drwxrwxr-x. 3 nisse nisse 4096 Sep 11 00:55 target
[nisse@localhost fft-denoise]$ git fetch https://github.com/nilssab/fft-denoise master
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
[nisse@localhost fft-denoise]$ ls -la
total 36
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 00:55 .
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 00:46 ..
-rw-rw-r--. 1 nisse nisse 1756 Sep 11 00:55 Cargo.lock
-rw-rw-r--. 1 nisse nisse  311 Sep 11 00:55 Cargo.toml
-rw-rw-r--. 1 nisse nisse  230 Sep 11 00:46 Cargo.toml~
drwxrwxr-x. 8 nisse nisse 4096 Sep 11 01:17 .git
-rw-rw-r--. 1 nisse nisse    8 Sep 11 00:46 .gitignore
drwxrwxr-x. 2 nisse nisse 4096 Sep 11 01:14 src
drwxrwxr-x. 3 nisse nisse 4096 Sep 11 00:55 target
[nisse@localhost fft-denoise]$ git merge https://github.com/nilssab/fft-denoise
merge: https://github.com/nilssab/fft-denoise - not something we can merge
[nisse@localhost fft-denoise]$ git merge https://github.com/nilssab/fft-denoise master
merge: https://github.com/nilssab/fft-denoise - not something we can merge
[nisse@localhost fft-denoise]$ git pause
git: 'pause' is not a git command. See 'git --help'.

The most similar command is
	prune
[nisse@localhost fft-denoise]$ git --help
usage: git [--version] [--help] [-C <path>] [-c <name>=<value>]
           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]
           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]
           <command> [<args>]

These are common Git commands used in various situations:

start a working area (see also: git help tutorial)
   clone             Clone a repository into a new directory
   init              Create an empty Git repository or reinitialize an existing one

work on the current change (see also: git help everyday)
   add               Add file contents to the index
   mv                Move or rename a file, a directory, or a symlink
   restore           Restore working tree files
   rm                Remove files from the working tree and from the index
   sparse-checkout   Initialize and modify the sparse-checkout

examine the history and state (see also: git help revisions)
   bisect            Use binary search to find the commit that introduced a bug
   diff              Show changes between commits, commit and working tree, etc
   grep              Print lines matching a pattern
   log               Show commit logs
   show              Show various types of objects
   status            Show the working tree status

grow, mark and tweak your common history
   branch            List, create, or delete branches
   commit            Record changes to the repository
   merge             Join two or more development histories together
   rebase            Reapply commits on top of another base tip
   reset             Reset current HEAD to the specified state
   switch            Switch branches
   tag               Create, list, delete or verify a tag object signed with GPG

collaborate (see also: git help workflows)
   fetch             Download objects and refs from another repository
   pull              Fetch from and integrate with another repository or a local branch
   push              Update remote refs along with associated objects

'git help -a' and 'git help -g' list available subcommands and some
concept guides. See 'git help <command>' or 'git help <concept>'
to read about a specific subcommand or concept.
See 'git help git' for an overview of the system.
[nisse@localhost fft-denoise]$ git rebase https://github.com/nilssab/fft-denoise master
fatal: invalid upstream 'https://github.com/nilssab/fft-denoise'
[nisse@localhost fft-denoise]$ git switch master
Already on 'master'
[nisse@localhost fft-denoise]$ git --help
usage: git [--version] [--help] [-C <path>] [-c <name>=<value>]
           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]
           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]
           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]
           <command> [<args>]

These are common Git commands used in various situations:

start a working area (see also: git help tutorial)
   clone             Clone a repository into a new directory
   init              Create an empty Git repository or reinitialize an existing one

work on the current change (see also: git help everyday)
   add               Add file contents to the index
   mv                Move or rename a file, a directory, or a symlink
   restore           Restore working tree files
   rm                Remove files from the working tree and from the index
   sparse-checkout   Initialize and modify the sparse-checkout

examine the history and state (see also: git help revisions)
   bisect            Use binary search to find the commit that introduced a bug
   diff              Show changes between commits, commit and working tree, etc
   grep              Print lines matching a pattern
   log               Show commit logs
   show              Show various types of objects
   status            Show the working tree status

grow, mark and tweak your common history
   branch            List, create, or delete branches
   commit            Record changes to the repository
   merge             Join two or more development histories together
   rebase            Reapply commits on top of another base tip
   reset             Reset current HEAD to the specified state
   switch            Switch branches
   tag               Create, list, delete or verify a tag object signed with GPG

collaborate (see also: git help workflows)
   fetch             Download objects and refs from another repository
   pull              Fetch from and integrate with another repository or a local branch
   push              Update remote refs along with associated objects

'git help -a' and 'git help -g' list available subcommands and some
concept guides. See 'git help <command>' or 'git help <concept>'
to read about a specific subcommand or concept.
See 'git help git' for an overview of the system.
[nisse@localhost fft-denoise]$ git rm src/main.rs
git logrm 'src/main.rs'
[nisse@localhost fft-denoise]$ git rm Cargo.toml
rm 'Cargo.toml'
[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise master
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
fatal: refusing to merge unrelated histories
[nisse@localhost fft-denoise]$ git reset master
Unstaged changes after reset:
D	Cargo.toml
D	src/main.rs
[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise master
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
fatal: refusing to merge unrelated histories
[nisse@localhost fft-denoise]$ git push
fatal: No configured push destination.
Either specify the URL from the command-line or configure a remote repository using

    git remote add <name> <url>

and then push using the remote name

    git push <name>

[nisse@localhost fft-denoise]$ git logrm 'src/main.rs'
git: 'logrm' is not a git command. See 'git --help'.
[nisse@localhost fft-denoise]$ git log
WARNING: terminal is not fully functional
-  (press RETURN)
commit 90ba664228636d5b7576a5a3fd6f29b63626b5af (HEAD -> master, test)
Author: nils sabelstrom <nilssab@gmail.com>
Date:   Fri Sep 11 01:10:20 2020 +0200

    initial commit
[nisse@localhost fft-denoise]$ git branch rm test
[nisse@localhost fft-denoise]$ git log
WARNING: terminal is not fully functional
-  (press RETURN)
commit 90ba664228636d5b7576a5a3fd6f29b63626b5af (HEAD -> master, test, rm)
Author: nils sabelstrom <nilssab@gmail.com>
Date:   Fri Sep 11 01:10:20 2020 +0200

    initial commit
[nisse@localhost fft-denoise]$ git switch test
D	Cargo.toml
D	src/main.rs
Switched to branch 'test'
[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise master
From https://github.com/nilssab/fft-denoise
 * branch            master     -> FETCH_HEAD
fatal: refusing to merge unrelated histories
[nisse@localhost fft-denoise]$ git remote add origin https://github.com/nilssab/fft-denoise
[nisse@localhost fft-denoise]$ git push origin master
Username for 'https://github.com': nilssab
Password for 'https://nilssab@github.com': 
To https://github.com/nilssab/fft-denoise
 ! [rejected]        master -> master (non-fast-forward)
error: failed to push some refs to 'https://github.com/nilssab/fft-denoise'
hint: Updates were rejected because a pushed branch tip is behind its remote
hint: counterpart. Check out this branch and integrate the remote changes
hint: (e.g. 'git pull ...') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
[nisse@localhost fft-denoise]$ git pull
Your configuration specifies to merge with the ref 'refs/heads/test'
from the remote, but no such ref was fetched.
[nisse@localhost fft-denoise]$ git fetch
[nisse@localhost fft-denoise]$ git pull master
fatal: 'master' does not appear to be a git repository
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
[nisse@localhost fft-denoise]$ git pull git push origin master
fatal: 'git' does not appear to be a git repository
fatal: Could not read from remote repository.

Please make sure you have the correct access rights
and the repository exists.
[nisse@localhost fft-denoise]$ git pull https://github.com/nilssab/fft-denoise
Your configuration specifies to merge with the ref 'refs/heads/test'
from the remote, but no such ref was fetched.
[nisse@localhost fft-denoise]$ git merge https://github.com/nilssab/fft-denoise master
merge: https://github.com/nilssab/fft-denoise - not something we can merge
[nisse@localhost fft-denoise]$ git merge https://github.com/nilssab/fft-denoise
merge: https://github.com/nilssab/fft-denoise - not something we can merge
[nisse@localhost fft-denoise]$ cd ..
[nisse@localhost rustmv fft-denoise/ fft-denoise-bkp
[nisse@localhost rust]$ ls
fft-denoise-bkp  hello-vulkan
[nisse@localhost rust]$ git clone https://github.com/nilssab/fft-denoise
Cloning into 'fft-denoise'...
remote: Enumerating objects: 5, done.        
remote: Counting objects: 100% (5/5), done.        
remote: Compressing objects: 100% (5/5), done.        
remote: Total 5 (delta 0), reused 0 (delta 0), pack-reused 0        
Receiving objects: 100% (5/5), done.
[nisse@localhost rust]$ mv fft-denoise-bkp/ fft-denoise/
[nisse@localhost rust]$ cd fft-denoise/
[nisse@localhost fft-denoise]$ ls -la
total 28
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 01:34 .
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 01:34 ..
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 01:22 fft-denoise-bkp
drwxrwxr-x. 8 nisse nisse 4096 Sep 11 01:34 .git
-rw-rw-r--. 1 nisse nisse  320 Sep 11 01:34 .gitignore
-rw-rw-r--. 1 nisse nisse 1064 Sep 11 01:34 LICENSE
-rw-rw-r--. 1 nisse nisse   63 Sep 11 01:34 README.md
[nisse@localhost fft-denoise]$ cd ..
[nisse@localhost rust]$ ls -la
total 16
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 01:34 .
drwxrwxr-x. 4 nisse nisse 4096 Sep 10 00:17 ..
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 01:34 fft-denoise
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 00:43 hello-vulkan
[nisse@localhost rust]$ cd fft-denoise/
[nisse@localhost fft-denoise]$ cp fft-denoise-bkp/src/ ./
cp: -r not specified; omitting directory 'fft-denoise-bkp/src/'
[nisse@localhost fft-denoise]$ cp -r fft-denoise-bkp/src/ ./
[nisse@localhost fft-denoise]$ ls -la
total 32
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 01:36 .
drwxrwxr-x. 4 nisse nisse 4096 Sep 11 01:34 ..
drwxrwxr-x. 5 nisse nisse 4096 Sep 11 01:22 fft-denoise-bkp
drwxrwxr-x. 8 nisse nisse 4096 Sep 11 01:34 .git
-rw-rw-r--. 1 nisse nisse  320 Sep 11 01:34 .gitignore
-rw-rw-r--. 1 nisse nisse 1064 Sep 11 01:34 LICENSE
-rw-rw-r--. 1 nisse nisse   63 Sep 11 01:34 README.md
drwxrwxr-x. 2 nisse nisse 4096 Sep 11 01:36 src
[nisse@localhost fft-denoise]$ git pull
Already up to date.
[nisse@localhost fft-denoise]$ git add src/main.rs
fatal: pathspec 'src/main.rs' did not match any files
[nisse@localhost fft-denoise]$ 