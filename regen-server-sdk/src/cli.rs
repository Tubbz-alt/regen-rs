use clap::{App, ArgMatches};
use crate::result::Res;
use crate::context::Context;

pub trait CliHandler<R> {
    fn build_cli_app(&self, ctx: &Context, app: App) -> App;
    fn run_cli_app(&self, ctx: &Context, matches: ArgMatches) -> Res<R>;
}