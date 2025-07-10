use std::fmt::Display;

use similar::ChangeTag;
use similar::TextDiff;

#[derive(Clone, Debug)]
pub struct Diff {
    pub hunks: Vec<DiffHunk>,
}

#[derive(Clone, Debug)]
pub struct DiffHunk {
    pub lines: Vec<DiffLine>,
}

#[derive(Clone, Debug)]
pub struct DiffLine {
    pub kind: DiffOperation,
    pub old_line_num: Option<usize>,
    pub new_line_num: Option<usize>,
    pub inline_changes: Vec<InlineChange>,
}

#[derive(Clone, Debug)]
pub struct InlineChange {
    pub value: String,
    pub emphasized: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DiffOperation {
    Insert,
    Delete,
    Equal,
}

impl DiffOperation {
    pub fn sign(&self) -> String {
        match self {
            DiffOperation::Delete => "-",
            DiffOperation::Insert => "+",
            DiffOperation::Equal => " ",
        }
        .to_string()
    }
}

impl From<ChangeTag> for DiffOperation {
    fn from(value: ChangeTag) -> Self {
        match value {
            ChangeTag::Delete => DiffOperation::Delete,
            ChangeTag::Insert => DiffOperation::Insert,
            ChangeTag::Equal => DiffOperation::Equal,
        }
    }
}

impl Diff {
    pub fn new(old: &str, new: &str) -> Option<Self> {
        let diff = TextDiff::from_lines(old, new);

        if diff.ops().is_empty() {
            return None;
        }

        let mut hunks = Vec::new();
        for group in diff.grouped_ops(3) {
            let mut lines = Vec::new();

            for op in group {
                for change in diff.iter_inline_changes(&op) {
                    let operation = DiffOperation::from(change.tag());
                    let mut inline_changes = Vec::new();

                    for (emphasized, value) in change.iter_strings_lossy() {
                        inline_changes.push(InlineChange {
                            value: value.to_string(),
                            emphasized,
                        });
                    }

                    lines.push(DiffLine {
                        kind: operation,
                        old_line_num: change.old_index(),
                        new_line_num: change.new_index(),
                        inline_changes,
                    });
                }
            }

            hunks.push(DiffHunk { lines });
        }

        Some(Diff { hunks })
    }
}

impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.hunks.is_empty() {
            return Ok(());
        }

        let mut lines = Vec::new();

        for (idx, hunk) in self.hunks.iter().enumerate() {
            if idx > 0 {
                lines.push(format!("{:-^80}\n", "-"));
            }

            for diff_line in &hunk.lines {
                let sign = diff_line.kind.sign();
                let old_line = diff_line
                    .old_line_num
                    .map(|n| format!("{:<4}", n + 1))
                    .unwrap_or_else(|| "    ".to_string());

                let new_line = diff_line
                    .new_line_num
                    .map(|n| format!("{:<4}", n + 1))
                    .unwrap_or_else(|| "    ".to_string());

                let mut line_spans = vec![old_line, new_line, format!(" |{sign}")];

                for inline_change in &diff_line.inline_changes {
                    if inline_change.emphasized {
                        line_spans.push(format!("⸢{}⸣", inline_change.value));
                    } else {
                        line_spans.push(inline_change.value.clone());
                    }
                }

                lines.push(line_spans.join(""));
            }
        }

        f.write_str(&lines.join(""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn creating_simple_diff_works() {
        // GIVEN
        let diff = Diff::new(
            "
line 1
line 2
line 3
",
            "
line 1 (changed)
new line
line 2
(prefix) line 3 ( changed)
",
        )
        .expect("diff should've been created");

        // WHEN
        // THEN
        assert_snapshot!(diff, @r"
        1   1    | 
        2        |-line 1
            2    |+line 1⸢ (changed)⸣
            3    |+⸢new line⸣
        3   4    | line 2
        4        |-line 3
            5    |+⸢(prefix) ⸣line 3⸢ ( changed)⸣
        ");
    }

    #[test]
    fn creating_diff_with_multiple_hunks_works() {
        // GIVEN
        let diff = Diff::new(
            "
line 1
line 2
line 3
line 4
line 5
line 6
line 7
line 8
line 9
",
            "
line 1 (changed)
line 2
line 3
line 4
line 5
line 6
line 7
line 8
(prefix) line 9 (changed)
",
        )
        .expect("diff should've been created");

        // WHEN
        // THEN
        assert_snapshot!(diff, @r"
        1   1    | 
        2        |-line 1
            2    |+line 1⸢ (changed)⸣
        3   3    | line 2
        4   4    | line 3
        5   5    | line 4
        --------------------------------------------------------------------------------
        7   7    | line 6
        8   8    | line 7
        9   9    | line 8
        10       |-line 9
            10   |+⸢(prefix) ⸣line 9⸢ (changed)⸣
        ");
    }

    #[test]
    fn creating_a_diff_with_no_changes_works() {
        // GIVEN
        let diff = Diff::new("text", "text").expect("diff should've been created");

        // WHEN
        // THEN
        assert_snapshot!(diff, @r"");
    }
}
