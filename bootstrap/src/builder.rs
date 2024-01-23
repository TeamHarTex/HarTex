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

use std::any::type_name;
use std::ops::Deref;

use crate::build::Build;
use crate::config::flags::BootstrapSubcommand;
use crate::steps::build;
use crate::steps::clean;
use crate::steps::setup;

#[derive(Clone, Copy)]
pub enum BuildKind {
    Build,
    Clean,
    Setup,
}

impl BuildKind {
    pub fn steps(&self) -> Vec<StepDescriptor> {
        match self {
            Self::Build => vec![
                StepDescriptor::from::<build::Api>(*self),
                StepDescriptor::from::<build::Database>(*self),
                StepDescriptor::from::<build::Discord>(*self),
                StepDescriptor::from::<build::Localization>(*self),
                StepDescriptor::from::<build::Utilities>(*self),
            ],
            Self::Clean => vec![
                StepDescriptor::from::<clean::CleanApi>(*self),
                StepDescriptor::from::<clean::CleanDatabase>(*self),
                StepDescriptor::from::<clean::CleanDiscord>(*self),
                StepDescriptor::from::<clean::CleanLocalization>(*self),
                StepDescriptor::from::<clean::CleanUtilities>(*self),
            ],
            Self::Setup => vec![StepDescriptor::from::<setup::SetupProfile>(*self)],
        }
    }
}

pub struct Builder<'build> {
    pub build: &'build Build,
    pub kind: BuildKind,
}

impl<'build> Builder<'build> {
    pub fn new(build: &'build Build) -> Self {
        let kind = match build.config.subcommand {
            BootstrapSubcommand::Build => BuildKind::Build,
            BootstrapSubcommand::Clean => BuildKind::Clean,
            BootstrapSubcommand::Setup => BuildKind::Setup,
        };

        Self { build, kind }
    }

    pub fn run_cli(&self) {
        self.run_steps(self.kind.steps());
    }

    pub fn run_step<S: Step>(&'build self, step: S) -> S::Output {
        step.run(self)
    }

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

pub struct RunConfig<'run> {
    pub builder: &'run Builder<'run>,
}

pub struct StepDescriptor {
    pub name: &'static str,
    pub kind: BuildKind,
    pub run_config: fn(RunConfig<'_>),
}

impl StepDescriptor {
    pub fn from<S: Step>(kind: BuildKind) -> Self {
        Self {
            name: type_name::<S>(),
            kind,
            run_config: S::run_config,
        }
    }

    pub fn run(&self, builder: &Builder<'_>) {
        (self.run_config)(RunConfig { builder })
    }
}

pub trait Step {
    type Output;

    fn run(self, builder: &Builder<'_>) -> Self::Output;

    fn run_config(_: RunConfig<'_>);
}
