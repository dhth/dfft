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

- [x] Add basic TUI
- [x] Scrollable diff section
- [x] Help page
- [x] More detailed diffs
- [x] Allow following changes
- [x] Keymaps for moving around the list from diff pane
- [ ] Show contents for new file
- [ ] [bug] error when "G" is pressed
- [ ] Errors are shown to the user wherever possible
    - [ ] Errors listening to fs events
- [ ] Consistent colors
- [x] Stop/resume listening for changes
- [x] Play sound when a change occurs
- [x] Allow pre-populating snapshots at startup
- [x] Show number of snapshots in memory
- [x] Consider `dfft` specific ignore file
- [ ] Consider `.git/info/exclude`
