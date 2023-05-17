mod options;

use self::options::Opts;
pub(super) use self::options::Tasks;
use crate::usecase;
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
        let paths =
            usecase::make_start_and_end_paths(self.opts.src(), self.opts.dst(), self.opts.tasks())
                .expect("Fail to make paths.");
        usecase::convert_files(&self.opts.tasks, paths)
            .expect("Fail to convert input files to output files.");
    }
}
