# Attempt at a Bazel-ified build with complex dependencies

Build using Bazel:

```
$ bazel build //scratch-bin:scratch-bin
```

Run using Bazel:

```
$ bazel build //scratch-bin:scratch-bin
```

(Yes, I know the `:scratch-bin` part is redundant in this case)

This fails currently, because we can't build `openssl-sys` with the "vendored"
feature.

You can see a working build in action by commenting out the `openssl` dep in
`scratch/Cargo.toml` and then re-running the Bazel commands with `REPIN=1`:

```
$ REPIN=1 bazel run //scratch-bin:scratch-bin
```

You need the `REPIN` because the dependencies changed so the Bazel lock
file/crate repository needs to be updated. It's only required once, when
changing dependencies.
