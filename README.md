[![crates.io](https://img.shields.io/crates/v/inotify-info-rs?style=for-the-badge)](https://crates.io/crates/inotify-info-rs)
[![release](https://img.shields.io/github/release/retr0h/inotify-info-rs.svg?style=for-the-badge)](https://github.com/retr0h/inotify-info-rs/releases/latest)
[![codecov](https://img.shields.io/codecov/c/github/retr0h/inotify-info-rs?token=HPZ323ZKBG&style=for-the-badge)](https://codecov.io/gh/retr0h/inotify-info-rs)
[![rust report card](https://rust-reportcard.xuri.me/badge/github.com/retr0h/inotify-info-rs?style=for-the-badge)](https://rust-reportcard.xuri.me/report/github.com/retr0h/inotify-info-rs)
[![license](https://img.shields.io/badge/license-MIT-brightgreen.svg?style=for-the-badge)](LICENSE)
[![build](https://img.shields.io/github/actions/workflow/status/retr0h/inotify-info-rs/rust.yml?style=for-the-badge)](https://github.com/retr0h/inotify-info-rs/actions/workflows/rust.yml)
[![conventional commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg?style=for-the-badge)](https://conventionalcommits.org)
![gitHub commit activity](https://img.shields.io/github/commit-activity/m/retr0h/inotify-info-rs?style=for-the-badge)

# inotify-info-rs

A port of [inotify-info][] to Rust.

> Easily track down the number of inotify watches, instances, and which
files are being watched.

The Linux inotify system has a few issues [problem1][]/[problem2][], and
it can be difficult to debug when you for instance run out of watches.
Using this app should hopefully aid you in tracking down how many inotify
watches, instances, and what files are being watched.

[inotify-info]: https://github.com/mikesart/inotify-info
[problem1]: https://code.visualstudio.com/docs/setup/linux#_visual-studio-code-is-unable-to-watch-for-file-changes-in-this-large-workspace-error-enospc
[problem2]: https://unix.stackexchange.com/questions/15509/whos-consuming-my-inotify-resources

# Testing

Install the `cargo-make` task runner:

```bash
cargo install --no-default-features --force cargo-make
```

To execute tests:

```bash
cargo make test
```

Auto format code:

```bash
cargo make fmt
```

List helpful targets:

```bash
cargo make --list-category-steps project
```

## License

The [MIT][] License.

[MIT]: LICENSE
