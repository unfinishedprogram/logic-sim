use std::fmt::Write;

use crate::stopwatch::Stopwatch;

// Running-average timings for named, nestable spans within a frame.
//
// A span is identified by its name *and* its parent, so the same name can be
// reused under different parents. Each span keeps its own running average, so
// the numbers are smoothed rather than per-frame noise.
#[derive(Default)]
pub struct Profiler {
    spans: Vec<Span>,
    open: Vec<usize>,
}

struct Span {
    name: &'static str,
    parent: Option<usize>,
    depth: usize,
    watch: Stopwatch,
}

impl Profiler {
    // Opens a span, nested under whichever span is currently open.
    //
    // Reusing a name under a different parent creates a separate span. Reusing
    // it under the *same* parent reopens that span, so opening it twice in one
    // frame feeds two samples into one average rather than totalling them.
    pub fn begin(&mut self, name: &'static str) {
        let parent = self.open.last().copied();

        let index = match self
            .spans
            .iter()
            .position(|span| span.name == name && span.parent == parent)
        {
            Some(index) => index,
            None => {
                self.spans.push(Span {
                    name,
                    parent,
                    depth: self.open.len(),
                    watch: Stopwatch::default(),
                });
                self.spans.len() - 1
            }
        };

        self.spans[index].watch.start();
        self.open.push(index);
    }

    // Closes the most recently opened span, which must be `name`.
    pub fn end(&mut self, name: &'static str) {
        let Some(index) = self.open.pop() else {
            debug_assert!(false, "`{name}` was ended without a matching begin");
            return;
        };

        debug_assert_eq!(
            self.spans[index].name, name,
            "profiler spans must be closed in the order they were opened"
        );

        self.spans[index].watch.end();
    }

    // One line per span, indented by nesting depth. Children will not sum to
    // their parent exactly: the difference is work inside the parent that no
    // child covers.
    pub fn report(&self) -> String {
        let label_width = self
            .spans
            .iter()
            .map(|span| span.name.len() + span.depth * 2)
            .max()
            .unwrap_or_default();

        let mut report = String::new();
        self.write_subtree(None, label_width, &mut report);

        report
    }

    // Walks the tree rather than the insertion order, so a span opened for the
    // first time on a later frame still prints under its parent instead of
    // being appended after everything else.
    fn write_subtree(&self, parent: Option<usize>, label_width: usize, report: &mut String) {
        for (index, span) in self.spans.iter().enumerate() {
            if span.parent != parent {
                continue;
            }

            let label = format!("{}{}", "  ".repeat(span.depth), span.name);
            let millis = span.watch.running_average().as_secs_f64() * 1000.0;
            let _ = writeln!(report, "{label:<label_width$} {millis:>7.3}ms");

            self.write_subtree(Some(index), label_width, report);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Profiler;

    // The indented span names of a report, with the timings stripped off.
    fn rows(profiler: &Profiler) -> Vec<String> {
        profiler
            .report()
            .lines()
            .map(|line| {
                let (label, _millis) = line.rsplit_once(' ').expect("every row has a timing");
                label.trim_end().to_string()
            })
            .collect()
    }

    #[test]
    fn same_name_under_different_parents_stays_separate() {
        let mut profiler = Profiler::default();

        profiler.begin("frame");

        profiler.begin("world");
        profiler.begin("upload");
        profiler.end("upload");
        profiler.end("world");

        profiler.begin("ui");
        profiler.begin("upload");
        profiler.end("upload");
        profiler.end("ui");

        profiler.end("frame");

        assert_eq!(
            rows(&profiler),
            ["frame", "  world", "    upload", "  ui", "    upload"]
        );
    }

    #[test]
    fn same_name_under_the_same_parent_is_one_span() {
        let mut profiler = Profiler::default();

        profiler.begin("frame");
        profiler.begin("pass");
        profiler.end("pass");
        profiler.begin("pass");
        profiler.end("pass");
        profiler.end("frame");

        assert_eq!(rows(&profiler), ["frame", "  pass"]);
    }

    #[test]
    fn a_span_first_seen_late_reports_under_its_parent() {
        let mut profiler = Profiler::default();

        profiler.begin("frame");
        profiler.begin("update");
        profiler.begin("solve");
        profiler.end("solve");
        profiler.end("update");
        profiler.begin("render");
        profiler.end("render");
        profiler.end("frame");

        // A later frame reaches a child of `update` for the first time
        profiler.begin("frame");
        profiler.begin("update");
        profiler.begin("draw");
        profiler.end("draw");
        profiler.end("update");
        profiler.end("frame");

        assert_eq!(
            rows(&profiler),
            ["frame", "  update", "    solve", "    draw", "  render"]
        );
    }
}
