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
use std::str::FromStr;

use crate::builder::Builder;
use crate::builder::RunConfig;
use crate::builder::Step;
use crate::config::Config;

/// Path to a default Fleet settings file.
const FLEET_SETTINGS: &str = include_str!("../../config/fleet-settings.json");
/// Path to a default VSCode settings file.
const VSCODE_SETTINGS: &str = include_str!("../../config/vscode-settings.json");

/// Profile for setting up the editors for different needs.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Copy)]
pub enum SetupProfile {
    /// Contribute to the API backend.
    ApiBackend,
    /// Contribute to the Discord frontend.
    DiscordFrontend,
    /// Host an instance of HarTex.
    Hosting,
    /// Contribute to localization infrastructure.
    Localization,
    /// Contribute to the web frontend.
    WebFrontend,
    /// Nothing.
    None,
}

impl SetupProfile {
    /// Returns the purpose of a given profile.
    #[must_use]
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

    /// Returns a list of variants.
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

    /// Converts it into a string.
    #[must_use]
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

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        setup_profile(&builder.build.config, self);
    }

    #[allow(clippy::single_match_else)]
    fn run_config(run: RunConfig<'_>) {
        let path = &run
            .builder
            .config
            .config_path
            .clone()
            .unwrap_or(PathBuf::from("hartex.conf"));
        if path.exists() {
            eprintln!(
                "WARN: a configuration file already exists at {}",
                path.canonicalize()
                    .expect("failed to canonicalize path")
                    .display()
            );

            match question_bool(
                "Do you wish to delete the existing configuration and replace it?",
                false,
            ) {
                Ok(true) => fs::remove_file(path).expect("failed to remove file"),
                _ => {
                    println!("Operation cancelled. Skipping.");
                    return;
                }
            }
        }

        let profile = interactive_profile().expect("failed to obtain profile");
        run.builder.run_step(profile);
    }
}

/// An interactive question-answer session for setting up the development environment.
#[allow(clippy::missing_errors_doc)]
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

/// Set up the development environment with the corresponding profile.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
pub fn setup_profile(config: &Config, profile: SetupProfile) {
    println!("INFO: using profile {profile}");
    println!("INFO: copying `tools/bootstrap/profiles/hartex.{profile}.conf` to `hartex.conf`");

    fs::copy(
        config
            .root
            .join(format!("tools/bootstrap/profiles/hartex.{profile}.conf")),
        config.root.join("hartex.conf"),
    )
    .expect("failed to copy files");

    println!("INFO: Done. `x.py` will now use the specified configuration in `hartex.conf` for further invocations.");
}

/// Step for configuration for VSCode.
pub struct ConfigureVscode;

impl Step for ConfigureVscode {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        setup_vscode_config(builder);
    }

    #[allow(clippy::single_match_else)]
    fn run_config(run: RunConfig<'_>) {
        let vscode_config = run.builder.config.root.join(".vscode/settings.json");
        if vscode_config.exists() {
            eprintln!(
                "WARN: a vscode configuration file already exists at {}",
                vscode_config
                    .canonicalize()
                    .expect("failed to canonicalize path")
                    .display()
            );

            match question_bool("Do you wish to delete and replace it?", false) {
                Ok(true) => fs::remove_file(vscode_config).expect("failed to remove file"),
                _ => {
                    println!("Operation cancelled. Skipping");
                    return;
                }
            }
        }

        println!("INFO: Preview of the recommended vscode configuration file is as follows");
        println!("{VSCODE_SETTINGS}");

        match question_bool("Do you wish to continue?", true) {
            Ok(true) => run.builder.run_step(ConfigureVscode),
            _ => {
                println!("Operation cancelled. Skipping.");
            }
        }
    }
}

/// Utility function for setting up the configuration for VSCode.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_io_amount)]
pub fn setup_vscode_config(builder: &Builder<'_>) {
    println!("INFO: writing new `.vscode/settings.json`");

    let vscode_dir = builder.config.root.join(".vscode");
    if !vscode_dir.exists() {
        fs::create_dir(vscode_dir).expect("failed to create directory");
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(builder.build.config.root.join(".vscode/settings.json"))
        .expect("failed to open file");
    file.write(VSCODE_SETTINGS.as_bytes())
        .expect("failed to write to file");
}

/// Step for configuration for Fleet.
pub struct ConfigureFleet;

/// Utility function for setting up the configuration for Fleet.
impl Step for ConfigureFleet {
    type Output = ();

    fn run(self, builder: &Builder<'_>) -> Self::Output {
        setup_fleet_config(builder);
    }

    fn run_config(run: RunConfig<'_>) {
        let fleet_config = run.builder.config.root.join(".fleet/settings.json");

        if fleet_config.exists() {
            eprintln!(
                "WARN: a fleet configuration file already exists at {}",
                fleet_config
                    .canonicalize()
                    .expect("failed to canonicalize path")
                    .display()
            );

            match question_bool("Do you wish to delete and replace it?", false) {
                Ok(true) => fs::remove_file(fleet_config).expect("failed to remove file"),
                _ => {
                    println!("Operation cancelled. Skipping");
                    return;
                }
            }
        }

        println!("INFO: Preview of the recommended fleet configuration file is as follows");
        println!("{FLEET_SETTINGS}");

        match question_bool("Do you wish to continue?", true) {
            Ok(true) => run.builder.run_step(ConfigureFleet),
            _ => {
                println!("Operation cancelled. Skipping.");
            }
        }
    }
}

/// Utility function for setting up the configuration for Fleet.
#[allow(clippy::missing_panics_doc)]
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_io_amount)]
pub fn setup_fleet_config(builder: &Builder<'_>) {
    println!("INFO: writing new `.fleet/settings.json`");

    let fleet_dir = builder.config.root.join(".fleet");
    if !fleet_dir.exists() {
        fs::create_dir(fleet_dir).expect("failed to create directory");
    }

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(builder.build.config.root.join(".fleet/settings.json"))
        .expect("failed to open file");
    file.write(FLEET_SETTINGS.as_bytes())
        .expect("failed to write to file");
}

/// Utility function for asking a question returning a boolean result.
fn question_bool(prompt: &str, default: bool) -> io::Result<bool> {
    let default_text = if default { "(Y/n)" } else { "(y/N)" };
    write!(io::stdout().lock(), "{prompt} {default_text} ")?;

    io::stdout().flush()?;
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

/// Utility function for asking a question returning a string result.
fn question_str(prompt: &str) -> io::Result<String> {
    write!(io::stdout().lock(), "{prompt} ")?;

    io::stdout().flush()?;
    readln()
}

/// Utility function for awaiting user input.
fn readln() -> io::Result<String> {
    let stdin = io::stdin();
    let locked = stdin.lock();
    let mut lines = locked.lines();

    lines.next().transpose()?.ok_or(io::Error::new(
        io::ErrorKind::Uncategorized,
        "failed to read from stdin",
    ))
}
