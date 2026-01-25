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
    // cli deps for environment variables
    (
        "cli",
        "command.wit",
        include_str!("../../../../capsule-wit/deps/cli/command.wit"),
    ),
    (
        "cli",
        "environment.wit",
        include_str!("../../../../capsule-wit/deps/cli/environment.wit"),
    ),
    (
        "cli",
        "exit.wit",
        include_str!("../../../../capsule-wit/deps/cli/exit.wit"),
    ),
    (
        "cli",
        "imports.wit",
        include_str!("../../../../capsule-wit/deps/cli/imports.wit"),
    ),
    (
        "cli",
        "run.wit",
        include_str!("../../../../capsule-wit/deps/cli/run.wit"),
    ),
    (
        "cli",
        "stdio.wit",
        include_str!("../../../../capsule-wit/deps/cli/stdio.wit"),
    ),
    (
        "cli",
        "terminal.wit",
        include_str!("../../../../capsule-wit/deps/cli/terminal.wit"),
    ),
    // sockets deps (required by cli)
    (
        "sockets",
        "instance-network.wit",
        include_str!("../../../../capsule-wit/deps/sockets/instance-network.wit"),
    ),
    (
        "sockets",
        "ip-name-lookup.wit",
        include_str!("../../../../capsule-wit/deps/sockets/ip-name-lookup.wit"),
    ),
    (
        "sockets",
        "network.wit",
        include_str!("../../../../capsule-wit/deps/sockets/network.wit"),
    ),
    (
        "sockets",
        "tcp-create-socket.wit",
        include_str!("../../../../capsule-wit/deps/sockets/tcp-create-socket.wit"),
    ),
    (
        "sockets",
        "tcp.wit",
        include_str!("../../../../capsule-wit/deps/sockets/tcp.wit"),
    ),
    (
        "sockets",
        "udp-create-socket.wit",
        include_str!("../../../../capsule-wit/deps/sockets/udp-create-socket.wit"),
    ),
    (
        "sockets",
        "udp.wit",
        include_str!("../../../../capsule-wit/deps/sockets/udp.wit"),
    ),
    (
        "sockets",
        "world.wit",
        include_str!("../../../../capsule-wit/deps/sockets/world.wit"),
    ),
    // random deps (required by cli)
    (
        "random",
        "insecure-seed.wit",
        include_str!("../../../../capsule-wit/deps/random/insecure-seed.wit"),
    ),
    (
        "random",
        "insecure.wit",
        include_str!("../../../../capsule-wit/deps/random/insecure.wit"),
    ),
    (
        "random",
        "random.wit",
        include_str!("../../../../capsule-wit/deps/random/random.wit"),
    ),
    (
        "random",
        "world.wit",
        include_str!("../../../../capsule-wit/deps/random/world.wit"),
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
