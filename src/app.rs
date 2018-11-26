use crate::config::ArgEnum;
use crate::config::{DarkSkyUnit, GenericWeatherUnit, OwmUnit};
use clap::{App, Arg, SubCommand};

pub fn build_cli() -> App<'static, 'static> {
    app_from_crate!()
        .arg(
            Arg::with_name("units")
                .long("units")
                .short("u")
                .takes_value(true)
                .possible_values(GenericWeatherUnit::VARIANTS),
        )
        .arg(
            Arg::with_name("coordinates")
                .long("coordinates")
                .short("c")
                .value_names(&["lat", "lon"])
                .required(false)
                .require_equals(true)
                .require_delimiter(true),
        )
        .arg(
            Arg::with_name("time")
                .long("time")
                .short("t")
                .takes_value(true),
        )
        .arg(Arg::with_name("live").long("live"))
        .subcommand(
            SubCommand::with_name("owm")
                .about("OpenWeatherMap")
                .arg(Arg::with_name("current").long("current").short("c"))
                .arg(
                    Arg::with_name("units")
                        .long("units")
                        .short("u")
                        .takes_value(true)
                        .possible_values(OwmUnit::VARIANTS),
                ),
        )
        .subcommand(
            SubCommand::with_name("darksky").about("DarkSky").arg(
                Arg::with_name("units")
                    .long("units")
                    .short("u")
                    .takes_value(true)
                    .possible_values(DarkSkyUnit::VARIANTS),
            ),
        )
        .arg(
            Arg::with_name("verbose")
                .long("verbose")
                .short("v")
                .global(true)
                .multiple(true),
        )
}
