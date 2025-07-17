<p align="center">
  <h1 align="center">dfft</h1>
  <p align="center">
    <a href="https://github.com/dhth/dfft/actions/workflows/main.yml"><img alt="Build status" src="https://img.shields.io/github/actions/workflow/status/dhth/dfft/main.yml?style=flat-square"></a>
  </p>
</p>

`dfft` (short for "diff-trail") watches for changes to files in a directory and
displays them in a TUI as they happen.

![dfft](https://github.com/user-attachments/assets/4f2b4182-87cb-47a4-bfb1-0aa45a8a24d0)

🤔 Motivation
---

I recently started to make use of agentic AI tools in my day-to-day workflows.
While I like setting them off on their own to solve problems, every now and then
I find myself needing to keep an eye on the changes introduced by them, either
to see if they're on the right track, or just because I'm curious about their
approach. Most agentic tools will print the changes they make as they make them,
but more often than not, the output gets scrolled out of view pretty fast. I
wanted a fast and simple tool that would let me peruse the changes made by these
agents at my own pace via a keyboard driven TUI.

`dfft` came out of my attempts at finding a good balance between letting AI
agents operate freely and having control over every little change in my
codebases. The hope is that the added transparency `dfft` provides helps me
understand the process AI agents follow when implementing a change, and also
lets me catch issues early.

💾 Installation
---

**cargo**:

```sh
cargo install --git https://github.com/dhth/dfft.git
```

⚡️ Usage
---

```bash
dfft run --help
```

```text
Usage: dfft run [OPTIONS]

Options:
  -p, --path <PATH>     Path of the directory to watch (defaults to current directory)
      --debug           Output debug information without doing anything
  -f, --follow-changes  Start with the setting "follow changes" enabled
      --no-prepop       Skip prepopulating cache with file snapshots
      --no-watch        Start with file watching disabled
      --no-sound        Start with sound notifications disabled
  -h, --help            Print help
```

🔔 Notifications
---

By default `dfft` will play distinct sound notifications for each kind of change
(`CREATION` / `MODIFICATION` / `REMOVAL`). You can toggle them on/off whenever
needed. If you prefer to not use sound notifications at all, you can build a
version of `dfft` that has no audio dependencies using the following.

```bash
cargo install --no-default-features --git https://github.com/dhth/dfft.git
```

📟 TUI
---

`dfft`'s TUI has 2 panes:

- `diff`: shows the diff for a change, or file contents for newly created files.
  Supports scrolling.
- `changes`: Holds the list of changes, with a label for each change

![start](https://tools.dhruvs.space/images/dfft/v0-1-0/start.png)

![tui](https://tools.dhruvs.space/images/dfft/v0-1-0/tui.png)

### General

| Key         | Action              |
|-------------|---------------------|
| `?`         | show/hide help view |
| `Esc` / `q` | go back/exit        |
| `<Ctrl+C>`  | exit immediately    |

### Diff Pane

| Key                 | Action                        |
|---------------------|-------------------------------|
| `j` / `↓`           | select next change            |
| `k` / `↑`           | select previous change        |
| `J`                 | scroll diff down by a line    |
| `K`                 | scroll diff up by a line      |
| `<c-d>`             | scroll diff down by half page |
| `<c-u>`             | scroll diff up by half page   |
| `g`                 | select first change           |
| `G`                 | select last change            |
| `<space>`           | toggle watching               |
| `<c-r>`             | reset list                    |
| `f`                 | toggle following changes      |
| `s`                 | toggle sound notifications    |
| `<tab>` / `<s-tab>` | switch to changes pane        |

### Changes Pane

| Key                 | Action                        |
|---------------------|-------------------------------|
| `j` / `↓`           | select next change            |
| `k` / `↑`           | select previous change        |
| `g`                 | select first change           |
| `G`                 | select last change            |
| `J`                 | scroll diff down by a line    |
| `K`                 | scroll diff up by a line      |
| `<c-d>`             | scroll diff down by half page |
| `<c-u>`             | scroll diff up by half page   |
| `f`                 | toggle following changes      |
| `s`                 | toggle sound notifications    |
| `<c-r>`             | reset list                    |
| `<space>`           | toggle watching               |
| `<tab>` / `<s-tab>` | switch to diff pane           |

### Help Pane

| Key       | Action      |
|-----------|-------------|
| `j` / `↓` | scroll down |
| `k` / `↑` | scroll up   |

Ignoring files
---

By default, `dfft` will consider `.gitignore` and `.git/info/exclude` files when
deciding which files to ignore. Additionally, you can create a `.dfftignore`
file to exclude paths that are not covered by the previous two.
