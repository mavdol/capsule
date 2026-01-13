use std::fs;
use std::io;
use std::path::Path;

pub const CAPSULE_WIT: &str = include_str!("../../../../capsule-wit/capsule.wit");

pub const WASI_DEPS: &[(&str, &str, &str)] = &[
    // filesystem deps
    (
        "filesystem",
        "types.wit",
        include_str!("../../../../capsule-wit/deps/filesystem/types.wit"),
    ),
    (
        "filesystem",
        "preopens.wit",
        include_str!("../../../../capsule-wit/deps/filesystem/preopens.wit"),
    ),
    (
        "filesystem",
        "world.wit",
        include_str!("../../../../capsule-wit/deps/filesystem/world.wit"),
    ),
    // io deps (required by filesystem)
    (
        "io",
        "error.wit",
        include_str!("../../../../capsule-wit/deps/io/error.wit"),
    ),
    (
        "io",
        "poll.wit",
        include_str!("../../../../capsule-wit/deps/io/poll.wit"),
    ),
    (
        "io",
        "streams.wit",
        include_str!("../../../../capsule-wit/deps/io/streams.wit"),
    ),
    (
        "io",
        "world.wit",
        include_str!("../../../../capsule-wit/deps/io/world.wit"),
    ),
    // clocks deps (required by filesystem)
    (
        "clocks",
        "monotonic-clock.wit",
        include_str!("../../../../capsule-wit/deps/clocks/monotonic-clock.wit"),
    ),
    (
        "clocks",
        "wall-clock.wit",
        include_str!("../../../../capsule-wit/deps/clocks/wall-clock.wit"),
    ),
    (
        "clocks",
        "world.wit",
        include_str!("../../../../capsule-wit/deps/clocks/world.wit"),
    ),
];

pub struct WitManager {}

impl WitManager {
    pub fn import_wit_deps(wit_dir: &Path) -> Result<(), io::Error> {
        fs::create_dir_all(wit_dir)?;
        fs::write(wit_dir.join("capsule.wit"), CAPSULE_WIT)?;

        for (pkg, filename, content) in WASI_DEPS {
            let pkg_dir = wit_dir.join("deps").join(pkg);
            fs::create_dir_all(&pkg_dir)?;
            fs::write(pkg_dir.join(filename), content)?;
        }

        Ok(())
    }
}
