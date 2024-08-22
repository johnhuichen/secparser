use indicatif::{self, ProgressBar, ProgressStyle};

pub struct CustomProgressBar {
    bar: ProgressBar,
}

impl CustomProgressBar {
    pub fn new(count: usize) -> Self {
        let bar = ProgressBar::new(count as u64);
        bar.set_style(
            ProgressStyle::with_template(
                "[{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        CustomProgressBar { bar }
    }

    pub fn inc_with_msg(&self, delta: u64, msg: &str) {
        self.bar.set_message(msg.to_string());
        self.bar.inc(delta);
    }

    pub fn finish(&self) {
        self.bar.finish();
    }
}
