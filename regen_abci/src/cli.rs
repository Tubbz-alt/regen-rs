use clap::{App, ArgMatches};
use crate::result::Res;
use regen_context::SimpleContext;

pub trait CliHandler<R> {
    fn build_cli_app(&self, ctx: &SimpleContext, app: App) -> App;
    fn run_cli_app(&self, ctx: &SimpleContext, matches: ArgMatches) -> Res<&R>;
}
pub trait CliMiddleware<R> {
    fn on_build_cli_app(&self, ctx: &SimpleContext, app: App, next: &dyn CliHandler<R>) -> App;
    fn on_run_cli_app(&self, ctx: &SimpleContext, matches: ArgMatches, next: &dyn CliHandler<R>) -> Res<&R>;
}
