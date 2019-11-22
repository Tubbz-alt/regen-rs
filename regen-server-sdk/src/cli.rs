use clap::{App, ArgMatches};
use crate::result::Res;
use crate::context::Context;

pub trait CliHandler<R> {
    fn build_cli_app(&self, ctx: &Context, app: App) -> App;
    fn run_cli_app(&self, ctx: &Context, matches: ArgMatches) -> Res<R>;
}
pub trait CliMiddleware<R> {
    fn on_build_cli_app(&self, ctx: &Context, app: App, next: &dyn CliHandler<R>) -> App;
    fn on_run_cli_app(&self, ctx: &Context, matches: ArgMatches, next: &dyn CliHandler<R>) -> Res<R>;
}
