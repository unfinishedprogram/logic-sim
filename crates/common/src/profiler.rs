use std::fmt::Write;

use crate::stopwatch::Stopwatch;

// Running-average timings for named, nestable spans within a frame.
//
// Spans are identified by name and reported in the order they were first
// opened, so the readout stays stable from frame to frame. Each span keeps its
// own running average, so the numbers are smoothed rather than per-frame noise.
#[derive(Default)]
pub struct Profiler {
    spans: Vec<Span>,
    open: Vec<usize>,
}

struct Span {
    name: &'static str,
    depth: usize,
    watch: Stopwatch,
}

impl Profiler {
    // Opens a span. A span opened while another is still open is reported
    // indented underneath it.
    pub fn begin(&mut self, name: &'static str) {
        let index = match self.spans.iter().position(|span| span.name == name) {
            Some(index) => index,
            None => {
                self.spans.push(Span {
                    name,
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
        for span in self.spans.iter() {
            let label = format!("{}{}", "  ".repeat(span.depth), span.name);
            let millis = span.watch.running_average().as_secs_f64() * 1000.0;
            let _ = writeln!(report, "{label:<label_width$} {millis:>7.3}ms");
        }

        report
    }
}
