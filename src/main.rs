use clap::builder::TypedValueParser as _;
use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(
        value_parser = clap::builder::PossibleValuesParser::new(["no-preference", "light", "dark"])
            .map(|s| s.parse::<value::PreferenceValue>().unwrap()),
    )]
    preference: Option<value::PreferenceValue>,
}

mod value {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum PreferenceValue {
        NoPreference,
        Light,
        Dark,
    }

    impl std::fmt::Display for PreferenceValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::NoPreference => "no-preference",
                Self::Light => "light",
                Self::Dark => "dark",
            };
            s.fmt(f)
        }
    }
    impl std::str::FromStr for PreferenceValue {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "no-preference" => Ok(Self::NoPreference),
                "light" => Ok(Self::Light),
                "dark" => Ok(Self::Dark),
                _ => Err(format!("Unknown log level: {s}")),
            }
        }
    }
}

fn main() {
    let preference = Cli::parse().preference;
    println!("{:?}", preference);
}
