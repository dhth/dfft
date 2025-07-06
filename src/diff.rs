use console::{Style, style};
use similar::ChangeTag;
use similar::TextDiff;

use ratatui::{
    style::{Color, Stylize},
    text::{Line, Masked, Span},
};

struct ChangeLineNum(Option<usize>);
impl std::fmt::Display for ChangeLineNum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(i) => write!(f, "{:<4}", i + 1),
        }
    }
}

// inspired by https://github.com/mitsuhiko/similar/blob/main/examples/terminal-inline.rs
pub fn get_diff(old: &str, new: &str) -> Option<String> {
    let diff = TextDiff::from_lines(old, new);

    let mut diff_lines = vec![];
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            diff_lines.push(format!("{:-^1$}\n", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                diff_lines.push(format!(
                    "{}{} |{}",
                    style(ChangeLineNum(change.old_index())).dim(),
                    style(ChangeLineNum(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));

                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        diff_lines.push(format!("{}", s.apply_to(value).underlined().on_black()));
                    } else {
                        diff_lines.push(format!("{}", s.apply_to(value)));
                    }
                }
                if change.missing_newline() {
                    diff_lines.push("\n".to_string());
                }
            }
        }
    }

    if diff_lines.is_empty() {
        None
    } else {
        Some(diff_lines.join(""))
    }
}

enum DiffLine {
    Separator,
    ChangeRemoved {
        old_line_num: Option<usize>,
        new_line_num: Option<usize>,
    },
}

pub fn get_diff2(old: &str, new: &str) -> Option<String> {
    let diff = TextDiff::from_lines(old, new).unified_diff().to_string();
    if diff.is_empty() { None } else { Some(diff) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    //-------------//
    //  SUCCESSES  //
    //-------------//

    #[test]
    fn diffing_works_when_contents_differ() {
        // GIVEN
        let old = r#"
struct Line(Option<usize>);
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(index) => write!(f, "{:<4}", index + 1),
        }
    }
}
"#;

        let new = r#"
struct Line(Option<usize>);
impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.0 {
            None => write!(f, "    "),
            Some(i) => write!(f, "{:<4}", i + 1),
        }
    }
}
"#;

        // WHEN
        let diff = get_diff(old, new).expect("diff should've been calculated");
        let plain_diff = strip_ansi_escapes::strip_str(&diff);

        // THEN
        assert_snapshot!(plain_diff, @r#"
        1   1    | 
        2   2    | struct Line(Option<usize>);
        3        |-impl fmt::Display for Line {
        4        |-    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            3    |+impl std::fmt::Display for Line {
            4    |+    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        5   5    |         match self.0 {
        6   6    |             None => write!(f, "    "),
        7        |-            Some(index) => write!(f, "{:<4}", index + 1),
            7    |+            Some(i) => write!(f, "{:<4}", i + 1),
        8   8    |         }
        9   9    |     }
        10  10   | }
        "#);
    }

    #[test]
    fn diffing_shows_various_sections_with_changes() {
        // GIVEN
        let old = r#"
pub fn get_diff(old: &str, new: &str) -> Option<String> {
    let diff = TextDiff::from_lines(old, new);

    let mut diff_lines = vec![];
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            diff_lines.push(format!("{:-^1$}\n", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().red()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                diff_lines.push(format!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));

                for (emphasized, value) in change.iter_strings_lossy() {
                    if emphasized {
                        diff_lines.push(format!("{}", s.apply_to(value).underlined().on_black()));
                    } else {
                        diff_lines.push(format!("{}", s.apply_to(value)));
                    }
                }
                if change.missing_newline() {
                    diff_lines.push("\n".to_string());
                }
            }
        }
    }

    if diff_lines.is_empty() {
        None
    } else {
        Some(diff_lines.join(""))
    }
}
"#;

        let new = r#"
pub fn compute_diff(old: &str, new: &str) -> Option<String> {
    let diff = TextDiff::from_lines(old, new);

    let mut diff_lines = vec![];
    for (idx, group) in diff.grouped_ops(3).iter().enumerate() {
        if idx > 0 {
            diff_lines.push(format!("{:-^1$}\n", "-", 80));
        }
        for op in group {
            for change in diff.iter_inline_changes(op) {
                let (sign, s) = match change.tag() {
                    ChangeTag::Delete => ("-", Style::new().magenta()),
                    ChangeTag::Insert => ("+", Style::new().green()),
                    ChangeTag::Equal => (" ", Style::new().dim()),
                };

                diff_lines.push(format!(
                    "{}{} |{}",
                    style(Line(change.old_index())).dim(),
                    style(Line(change.new_index())).dim(),
                    s.apply_to(sign).bold(),
                ));

                for (emph, value) in change.iter_strings_lossy() {
                    if emph {
                        diff_lines.push(format!("{}", s.apply_to(value).underlined().on_black()));
                    } else {
                        diff_lines.push(format!("{}", s.apply_to(value)));
                    }
                }
                if change.missing_newline() {
                    diff_lines.push("\n".to_string());
                }
            }
        }
    }

    if diff_lines.is_empty() {
        None
    } else {
        Some(diff_lines.join(""))
    }
}
"#;

        // WHEN
        let diff = get_diff(old, new).expect("diff should've been calculated");
        let plain_diff = strip_ansi_escapes::strip_str(&diff);

        // THEN
        assert_snapshot!(plain_diff, @r#"
        1   1    | 
        2        |-pub fn get_diff(old: &str, new: &str) -> Option<String> {
            2    |+pub fn compute_diff(old: &str, new: &str) -> Option<String> {
        3   3    |     let diff = TextDiff::from_lines(old, new);
        4   4    | 
        5   5    |     let mut diff_lines = vec![];
        --------------------------------------------------------------------------------
        10  10   |         for op in group {
        11  11   |             for change in diff.iter_inline_changes(op) {
        12  12   |                 let (sign, s) = match change.tag() {
        13       |-                    ChangeTag::Delete => ("-", Style::new().red()),
            13   |+                    ChangeTag::Delete => ("-", Style::new().magenta()),
        14  14   |                     ChangeTag::Insert => ("+", Style::new().green()),
        15  15   |                     ChangeTag::Equal => (" ", Style::new().dim()),
        16  16   |                 };
        --------------------------------------------------------------------------------
        22  22   |                     s.apply_to(sign).bold(),
        23  23   |                 ));
        24  24   | 
        25       |-                for (emphasized, value) in change.iter_strings_lossy() {
        26       |-                    if emphasized {
            25   |+                for (emph, value) in change.iter_strings_lossy() {
            26   |+                    if emph {
        27  27   |                         diff_lines.push(format!("{}", s.apply_to(value).underlined().on_black()));
        28  28   |                     } else {
        29  29   |                         diff_lines.push(format!("{}", s.apply_to(value)));
        "#);
    }

    #[test]
    fn diffing_returns_nothing_when_contents_dont_change() {
        // GIVEN
        let contents = "something";

        // WHEN
        let diff = get_diff(contents, contents);

        // THEN
        assert!(diff.is_none());
    }
}
