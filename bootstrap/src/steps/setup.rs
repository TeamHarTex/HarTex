/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2024 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

use std::fmt;
use std::fmt::Display;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;
use crate::config::Config;

#[derive(Clone, Copy)]
pub enum SetupProfile {
    ApiBackend,
    DiscordFrontend,
    Hosting,
    Localization,
    WebFrontend,
    None,
}

impl SetupProfile {
    pub fn purpose(&self) -> String {
        match self {
            Self::ApiBackend => "Contribute to the API backend",
            Self::DiscordFrontend => "Contribute to the Discord frontend",
            Self::Hosting => "Host an instance of HarTex",
            Self::Localization => "Contribute to the localization infastructure",
            Self::WebFrontend => "Contribute to the web frontend",
            Self::None => "Do not modify `hartex.conf`",
        }
        .to_string()
    }

    pub fn variants() -> impl Iterator<Item = Self> {
        [
            Self::ApiBackend,
            Self::DiscordFrontend,
            Self::Hosting,
            Self::Localization,
            Self::WebFrontend,
            Self::None,
        ]
        .iter()
        .copied()
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ApiBackend => "api",
            Self::DiscordFrontend => "discord",
            Self::Hosting => "hosting",
            Self::Localization => "localization",
            Self::WebFrontend => "web",
            Self::None => "none",
        }
    }
}

impl Display for SetupProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for SetupProfile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "api" => Ok(Self::ApiBackend),
            "discord" => Ok(Self::DiscordFrontend),
            "hosting" => Ok(Self::Hosting),
            "localization" => Ok(Self::Localization),
            "web" => Ok(Self::WebFrontend),
            "none" => Ok(Self::None),
            _ => Err(format!("unknown setup profile: {s}")),
        }
    }
}

impl Step for SetupProfile {
    type Output = ();

    fn run_config(run: RunConfig<'_>) {
        let path = &run
            .builder
            .config
            .config_path
            .clone()
            .unwrap_or(PathBuf::from("hartex.conf"));
        if path.exists() {
            eprintln!();
            eprintln!(
                "WARN: a configuration file already exists at {}",
                path.canonicalize()
                    .expect("failed to canonicalize path")
                    .display()
            );

            match question_bool("Do you wish to override the existing configuration, to allow the setup process to continue?", false) {
                Ok(true) => fs::remove_file(path).expect("failed to remove file"),
                _ => {
                    println!("Setup cancelled. Exiting.");
                    exit(1);
                }
            }
        }

        println!();

        let profile = interactive_profile().expect("failed to obtain profile");
        run.builder.run_step(profile);
    }

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        setup_profile(&builder.build.config, self);
    }
}

pub fn interactive_profile() -> io::Result<SetupProfile> {
    fn options() -> impl Iterator<Item = ((String, String), SetupProfile)> {
        ('a'..)
            .zip(1..)
            .map(|(letter, number)| (letter.to_string(), number.to_string()))
            .zip(SetupProfile::variants())
    }

    fn parse_profile(input: &str) -> Result<SetupProfile, String> {
        let input = input.trim().to_lowercase();
        for ((letter, number), profile) in options() {
            if input == letter || input == number {
                return Ok(profile);
            }
        }

        input.parse()
    }

    println!("Welcome to the HarTex project! What do you want to do with x.py?");

    for ((letter, _), profile) in options() {
        println!("{letter}) {profile}: {}", profile.purpose());
    }

    let profile = loop {
        let choice = question_str("Please choose one from above:")?;

        if choice.is_empty() {
            eprintln!("Unexpected choice. Please retry.");
            continue;
        }

        break match parse_profile(&choice) {
            Ok(profile) => profile,
            Err(err) => {
                eprintln!("ERROR: {err}. Please retry.");
                eprintln!("NOTE: press Ctrl+C to exit");
                continue;
            }
        };
    };

    Ok(profile)
}

pub fn setup_profile(config: &Config, profile: SetupProfile) {
    println!();
    println!("INFO: using profile {profile}");
    println!("INFO: copying `bootstrap/profiles/hartex.{profile}.conf` to `hartex.conf`");

    fs::copy(
        config
            .root
            .join(format!("bootstrap/profiles/hartex.{profile}.conf")),
        config.root.join("hartex.conf"),
    )
    .expect("failed to copy files");

    println!("INFO: Done. `x.py` will now use the specified configuration in `hartex.conf` for further invocations.");
}

fn question_bool(prompt: &str, default: bool) -> io::Result<bool> {
    let default_text = if default { "(Y/n)" } else { "(y/N)" };
    write!(io::stdout().lock(), "{prompt} {default_text} ")?;

    let _ = io::stdout().flush()?;
    let input = readln()?;

    if input.is_empty() {
        Ok(default)
    } else {
        match &*input.to_lowercase() {
            "y" => Ok(true),
            "n" => Ok(false),
            _ => Ok(default),
        }
    }
}

fn question_str(prompt: &str) -> io::Result<String> {
    write!(io::stdout().lock(), "{prompt} ")?;

    let _ = io::stdout().flush()?;
    readln()
}

fn readln() -> io::Result<String> {
    let stdin = io::stdin();
    let locked = stdin.lock();
    let mut lines = locked.lines();

    lines.next().transpose()?.ok_or(io::Error::new(
        io::ErrorKind::Uncategorized,
        "failed to read from stdin",
    ))
}
