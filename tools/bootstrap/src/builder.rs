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

//! # Builder of the Buildsystem
//!
//! This governs how the project is built for individual commands and tasks.

use std::any::type_name;
use std::ops::Deref;

use crate::build::Build;
use crate::config::flags::BootstrapSubcommand;
use crate::steps::build;
use crate::steps::check;
use crate::steps::clean;
use crate::steps::clippy;
use crate::steps::setup;
use crate::steps::test;

/// The type of build to perform.
#[derive(Clone, Copy)]
pub enum BuildKind {
    /// Run `cargo build`.
    Build,
    /// Run `cargo check`.
    Check,
    /// Delete build artifacts from the `build` directory.
    Clean,
    /// Run `cargo clippy`.
    Clippy,
    /// Setup editor tooling for contributors.
    Setup,
    /// Run the test suite.
    Test,
}

impl BuildKind {
    /// Returns the steps required for each build workflow.
    #[must_use]
    pub fn steps(&self) -> Vec<StepDescriptor> {
        match self {
            Self::Build => vec![
                StepDescriptor::from::<build::Api>(*self),
                StepDescriptor::from::<build::Database>(*self),
                StepDescriptor::from::<build::Discord>(*self),
                StepDescriptor::from::<build::Localization>(*self),
                StepDescriptor::from::<build::Utilities>(*self),
            ],
            Self::Check => vec![
                StepDescriptor::from::<check::CheckApi>(*self),
                StepDescriptor::from::<check::CheckDatabase>(*self),
                StepDescriptor::from::<check::CheckDiscord>(*self),
                StepDescriptor::from::<check::CheckLocalization>(*self),
                StepDescriptor::from::<check::CheckUtilities>(*self),
            ],
            Self::Clean => vec![
                StepDescriptor::from::<clean::CleanApi>(*self),
                StepDescriptor::from::<clean::CleanDatabase>(*self),
                StepDescriptor::from::<clean::CleanDiscord>(*self),
                StepDescriptor::from::<clean::CleanLocalization>(*self),
                StepDescriptor::from::<clean::CleanUtilities>(*self),
            ],
            Self::Clippy => vec![
                StepDescriptor::from::<clippy::ClippyApi>(*self),
                StepDescriptor::from::<clippy::ClippyDatabase>(*self),
                StepDescriptor::from::<clippy::ClippyDiscord>(*self),
                StepDescriptor::from::<clippy::ClippyLocalization>(*self),
                StepDescriptor::from::<clippy::ClippyUtilities>(*self),
            ],
            Self::Setup => vec![
                StepDescriptor::from::<setup::SetupProfile>(*self),
                StepDescriptor::from::<setup::ConfigureVscode>(*self),
                StepDescriptor::from::<setup::ConfigureFleet>(*self),
            ],
            Self::Test => vec![
                StepDescriptor::from::<test::BuildTestsuiteTool>(*self),
                StepDescriptor::from::<test::RunUiTests>(*self),
            ],
        }
    }
}

/// A builder with a reference to the current build session.
pub struct Builder<'build> {
    /// The reference to the current build session.
    pub build: &'build Build,
    /// The type of build workflow to perform.
    pub kind: BuildKind,
}

impl<'build> Builder<'build> {
    /// Construct a new builder from a build session.
    #[must_use]
    pub fn new(build: &'build Build) -> Self {
        let kind = match build.config.subcommand {
            BootstrapSubcommand::Build => BuildKind::Build,
            BootstrapSubcommand::Check => BuildKind::Check,
            BootstrapSubcommand::Clean => BuildKind::Clean,
            BootstrapSubcommand::Clippy => BuildKind::Clippy,
            BootstrapSubcommand::Setup => BuildKind::Setup,
            BootstrapSubcommand::Test => BuildKind::Test,
        };

        Self { build, kind }
    }

    /// Run the build.
    pub fn run_cli(&self) {
        self.run_steps(self.kind.steps());
    }

    /// Run the specified step.
    pub fn run_step<S: Step>(&'build self, step: S) -> S::Output {
        step.run(self)
    }

    /// Run all steps.
    fn run_steps(&self, steps: Vec<StepDescriptor>) {
        for step in steps {
            step.run(self);
        }
    }
}

impl<'build> Deref for Builder<'build> {
    type Target = Build;

    fn deref(&self) -> &Self::Target {
        self.build
    }
}

/// A run configuration.
pub struct RunConfig<'run> {
    /// A builder.
    pub builder: &'run Builder<'run>,
}

/// A descriptor that describes how a step should be performed.
pub struct StepDescriptor {
    /// Name of the step descriptor.
    pub name: &'static str,
    /// The build kind the step is associated with.
    pub kind: BuildKind,
    /// A closure that takes the run configuration.
    pub run_config: fn(RunConfig<'_>),
}

impl StepDescriptor {
    /// Construct a step descriptor from a build workflow type and a generic step.
    #[must_use]
    pub fn from<S: Step>(kind: BuildKind) -> Self {
        Self {
            name: type_name::<S>(),
            kind,
            run_config: S::run_config,
        }
    }

    /// Configure the step associated with this descriptor.
    pub fn run(&self, builder: &Builder<'_>) {
        (self.run_config)(RunConfig { builder });
    }
}

/// A trait for all build steps to implement.
pub trait Step {
    /// The output type of this step.
    type Output;

    /// The run callback of this step.
    fn run(self, builder: &Builder<'_>) -> Self::Output;

    /// The run configuration callback.
    fn run_config(_: RunConfig<'_>);
}
