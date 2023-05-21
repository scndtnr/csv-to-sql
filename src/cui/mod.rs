mod options;

use self::options::Opts;
pub(super) use self::options::Tasks;
use crate::tasks;
use clap::Parser;

#[derive(Debug, Clone)]
pub(super) struct Cui {
    opts: Opts,
}

impl Cui {
    pub(super) async fn new() -> Self {
        Self {
            opts: Opts::parse(),
        }
    }

    pub(super) async fn process(&self) {
        println!("args: {:#?}", &self.opts);
        let paths =
            tasks::make_start_and_end_paths(self.opts.src(), self.opts.dst(), self.opts.tasks())
                .expect("Fail to make paths.");
        tasks::convert_files(&self.opts.tasks, paths)
            .expect("Fail to convert input files to output files.");
    }
}
