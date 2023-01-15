use clap::builder::TypedValueParser as _;
use clap::Parser;
use std::fmt::Debug;
use std::process::Command;
use zbus::blocking::Connection;
use zbus::zvariant::Value;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(
        value_parser = clap::builder::PossibleValuesParser::new(["default", "light", "dark"])
            .map(|s| s.parse::<preferences::PreferenceValue>().unwrap()),
    )]
    preference: Option<preferences::PreferenceValue>,
}

mod preferences {
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    pub enum PreferenceValue {
        Default,
        Light,
        Dark,
    }

    impl std::fmt::Display for PreferenceValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Self::Default => "default",
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
                "default" => Ok(Self::Default),
                "light" => Ok(Self::Light),
                "dark" => Ok(Self::Dark),
                _ => Err(format!("Unknown log level: {s}")),
            }
        }
    }

    // Get index of preference
    impl PreferenceValue {
        pub fn gsettings_value(&self) -> &str {
            match self {
                Self::Default => "default",
                Self::Light => "prefer-light",
                Self::Dark => "prefer-dark",
            }
        }
    }
}

fn get_preference() -> Option<preferences::PreferenceValue> {
    // Get preference from dbus
    let conn = Connection::session();

    if conn.is_err() {
        return None;
    }

    let reply = conn.unwrap().call_method(
        Some("org.freedesktop.portal.Desktop"),
        "/org/freedesktop/portal/desktop",
        Some("org.freedesktop.portal.Settings"),
        "Read",
        &("org.freedesktop.appearance", "color-scheme"),
    );

    if let Ok(reply) = &reply {
        let theme = reply.body::<Value>();

        if theme.is_err() {
            return None;
        }

        let theme = theme.unwrap().downcast::<u32>();

        match theme.unwrap() {
            1 => Some(preferences::PreferenceValue::Dark),
            2 => Some(preferences::PreferenceValue::Light),
            _ => Some(preferences::PreferenceValue::Default),
        }
    } else {
        None
    }
}

fn set_preference(preference: preferences::PreferenceValue) {
    Command::new("gsettings")
        .args([
            "set",
            "org.gnome.desktop.interface",
            "color-scheme",
            preference.gsettings_value(),
        ])
        .spawn()
        .expect("failed to execute process");
}

fn main() {
    let preference = Cli::parse().preference;

    if preference.is_some() {
        set_preference(preference.unwrap());
    }

    // Wait for gsettings to finish
    std::thread::sleep(std::time::Duration::from_millis(100));

    let preference = get_preference();

    if preference.is_some() {
        println!("{:?}", preference.unwrap().to_string());
    } else {
        std::process::exit(1);
    }
}
