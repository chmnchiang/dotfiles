use clap::{App, ArgMatches};
use crate::common::*;
use crate::prompt::Prompt;
use crate::ops::Op;

pub struct Context {
    //pub logger: slog::Logger,
    pub prompt: Prompt,
    pub is_dry_run: bool,
}

pub trait Runner {
    fn build_cli() -> App<'static, 'static>;
    fn run(argm: &ArgMatches, context: &Context) -> Result<()>;

    fn run2(argm: &ArgMatches) -> Result<()> {
        //let logger = slog_scope::logger();

        let context = Context {
            //logger: logger.clone(),
            prompt: Prompt::new(),
            is_dry_run: argm.is_present("dry"),
        };

        Self::run(argm, &context)
            .map_err(|error| {
                context.prompt.error(&error.to_string());
                error
            })
    }
}

pub trait Installable {
    fn generate_op() -> Box<dyn Op>;

    fn install(context: &Context) -> Result<()> {
        Self::generate_op().commit(context)
    }
}
