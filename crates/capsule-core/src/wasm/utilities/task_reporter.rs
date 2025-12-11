use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use indicatif::{ProgressBar, ProgressStyle};

pub struct TaskReporter {
    verbose: bool,
    start_time: Instant,
    spinner: Option<ProgressBar>,
    is_active: Arc<AtomicBool>,
}

impl TaskReporter {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            start_time: Instant::now(),
            spinner: None,
            is_active: Arc::new(AtomicBool::new(false)),
        }
    }

    fn create_spinner(&self, message: &str) -> ProgressBar {
        let spinner = ProgressBar::new_spinner();
        spinner.set_style(
            ProgressStyle::default_spinner()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        spinner.enable_steady_tick(Duration::from_millis(80));
        spinner.set_message(message.to_string());
        spinner
    }

    pub fn task_running(&mut self, task_name: &str, task_id: &str) {
        self.finish_spinner();
        self.start_time = Instant::now();
        if self.verbose {
            println!("Capsule log: ▶ Starting task '{}' ({})", task_name, task_id);
        }
    }

    pub fn task_completed(&mut self, task_name: &str) {
        self.finish_spinner();
        if self.verbose {
            let elapsed = self.start_time.elapsed();
            let time_str = self.format_duration(elapsed);
            println!(
                "Capsule log: ✔ Task '{}' completed ({})",
                task_name, time_str
            );
        }
    }

    pub fn task_completed_with_time(&mut self, task_name: &str, elapsed: Duration) {
        self.finish_spinner();
        if self.verbose {
            let time_str = self.format_duration(elapsed);
            println!(
                "Capsule log: ✔ Task '{}' completed ({})",
                task_name, time_str
            );
        }
    }

    pub fn task_failed(&mut self, task_name: &str, error: &str) {
        self.finish_spinner();
        if self.verbose {
            println!("Capsule log: ✗ Task '{}' failed: {}", task_name, error);
        } else {
            eprintln!("Capsule log: ✗ {}", error);
        }
    }

    pub fn task_timeout(&mut self, task_name: &str) {
        self.finish_spinner();
        if self.verbose {
            println!("Capsule log: ✗ Task '{}' timed out", task_name);
        } else {
            eprintln!("Capsule log: ✗ Task timed out");
        }
    }

    pub fn start_progress(&mut self, message: &str) {
        if self.verbose {
            self.start_time = Instant::now();
            self.spinner = Some(self.create_spinner(message));
            self.is_active.store(true, Ordering::SeqCst);
        }
    }

    pub fn update_progress(&mut self, message: &str) {
        if let Some(spinner) = &self.spinner {
            spinner.set_message(message.to_string());
        }
    }

    pub fn finish_progress(&mut self, completion_message: Option<&str>) {
        self.finish_spinner();
        let elapsed = self.start_time.elapsed();
        let time_str = self.format_duration(elapsed);

        if let Some(msg) = completion_message
            && self.verbose
        {
            println!("✓ {} ({})", msg, time_str);
        }
    }

    pub fn info(&self, message: &str) {
        if self.verbose {
            println!("{}", message);
        }
    }

    pub fn success(&self, message: &str) {
        println!("{}", message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("{}", message);
    }

    pub fn format_duration(&self, duration: std::time::Duration) -> String {
        let total_secs = duration.as_secs_f64();

        if total_secs < 60.0 {
            format!("{:.2}s", total_secs)
        } else if total_secs < 3600.0 {
            let minutes = (total_secs / 60.0).floor() as u64;
            let seconds = total_secs % 60.0;
            format!("{}m {:.0}s", minutes, seconds)
        } else {
            let hours = (total_secs / 3600.0).floor() as u64;
            let remaining_secs = total_secs % 3600.0;
            let minutes = (remaining_secs / 60.0).floor() as u64;
            let seconds = remaining_secs % 60.0;
            format!("{}h {}m {:.0}s", hours, minutes, seconds)
        }
    }

    fn finish_spinner(&mut self) {
        if let Some(spinner) = self.spinner.take() {
            spinner.finish_and_clear();
            drop(spinner);
        }
        self.is_active.store(false, Ordering::SeqCst);
    }
}

impl Drop for TaskReporter {
    fn drop(&mut self) {
        self.finish_spinner();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let reporter = TaskReporter::new(true);
        assert!(reporter.verbose);
        assert!(reporter.spinner.is_none());
        assert!(!reporter.is_active.load(Ordering::SeqCst));
    }

    #[test]
    fn test_format_duration_seconds() {
        let reporter = TaskReporter::new(false);
        let duration = Duration::from_secs_f64(45.67);
        assert_eq!(reporter.format_duration(duration), "45.67s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let reporter = TaskReporter::new(false);
        let duration = Duration::from_secs(125);
        assert_eq!(reporter.format_duration(duration), "2m 5s");
    }

    #[test]
    fn test_format_duration_hours() {
        let reporter = TaskReporter::new(false);
        let duration = Duration::from_secs(3665);
        assert_eq!(reporter.format_duration(duration), "1h 1m 5s");
    }

    #[test]
    fn test_task_running_sets_active() {
        let mut reporter = TaskReporter::new(true);
        reporter.task_running("test_task", "task_123");
        assert!(!reporter.is_active.load(Ordering::SeqCst));
    }

    #[test]
    fn test_finish_spinner_clears_active() {
        let mut reporter = TaskReporter::new(true);
        reporter.start_progress("test");
        reporter.finish_spinner();
        assert!(!reporter.is_active.load(Ordering::SeqCst));
        assert!(reporter.spinner.is_none());
    }
}
