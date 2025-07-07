<p align="center">
  <h1 align="center">dfft</h1>
  <p align="center">
    <a href="https://github.com/dhth/dfft/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/dfft/main.yml?style=flat-square"></a>
  </p>
</p>

`dfft` (short for "diff-trail") watches for changes to files in a directory, and
outputs them as they happen.

> `dfft` is very early in the development process, and is not ready for use.
> Its behaviour and interface is likely to change for a while.

Todo
---

- [ ] Add TUI
    - [x] Add basic TUI
    - [ ] Scrollable diff section
    - [ ] Help page
    - [ ] More detailed diffs
    - [ ] Errors are shown to the user wherever possible
        - [ ] Errors listening to fs events
    - [ ] Consistent colors
    - [ ] Fix issue with list cursor going beyond the limit
    - [x] Stop/resume listening for changes
- [ ] Add CLI argument parsing
    - [ ] Add flag to pre-populate cache at startup
- [ ] Consider `.git/index`
- [x] Consider `dfft` specific ignore file
