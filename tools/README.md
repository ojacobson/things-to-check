# Local Tools

The scripts in this directory assume they will be run from the root of the
project, as `tools/NAME`. They contain brief, branch-free, composable scripts
intended to be run to achieve frequent goals. They act as a shared shell
history, of a sorts, and as a place to put command-line-ish code that needs to
be shared by multiple components.

Each script begins with a brief comment demonstrating the intended invocation
and the effects.

## Authoring

Tools _should_ begin with a shebang or shell `set` expression that enables
exiting on failure and that enables command echoing, followed by a documentation
comment:

```bash
#!/bin/bash -ex

# tools/my-example-tool
#
# Runs all example tasks.

: …
```
