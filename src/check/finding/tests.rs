//! Every finding names its path, what is wrong and a fix.

use crate::check::{Finding, Problem};

const ALL: [Problem; 4] = [
    Problem::Undeclared,
    Problem::Missing,
    Problem::AlsoRendered,
    Problem::NoReason,
];

fn finding(line: Option<usize>, problem: Problem) -> Finding {
    Finding {
        path: ".gitlab/ci/x.yml".to_owned(),
        line,
        problem,
    }
}

#[test]
fn the_text_names_the_path_the_problem_and_the_fix() {
    for problem in ALL {
        assert!(!problem.what().is_empty() && !problem.fix().is_empty());
        let want = format!(
            ".gitlab/ci/x.yml: {}; fix: {}",
            problem.what(),
            problem.fix()
        );
        assert_eq!(finding(None, problem).to_string(), want);
    }
}

#[test]
fn a_row_finding_names_its_line() {
    let shown = finding(Some(7), Problem::Missing).to_string();
    assert!(
        shown.starts_with(".gitlab/ci/x.yml (.lablith line 7): "),
        "{shown}"
    );
}

#[test]
fn the_texts_name_the_tables_to_edit() {
    assert!(Problem::Undeclared.fix().contains("## hand-written"));
    assert!(Problem::Undeclared.fix().contains("## workflows"));
    assert!(Problem::Missing.fix().contains("## hand-written"));
    assert!(Problem::AlsoRendered.fix().contains("## hand-written"));
    assert!(Problem::NoReason.fix().contains("reason"));
}
