projects {
  project("common", ProjectType.TYPESCRIPT)
  project("discord-frontend", ProjectType.RUST)
  project("web-frontend", ProjectType.TYPESCRIPT)
}

buildConfigurationForProject("common") {
  buildTool(ProjectBuildTool.CARGO)
  cargoBuildProfile(CargoBuildProfile.RELEASE)
  includeDebugInformation(true)
  linker("rust-lld")
}

buildConfigurationForProject("discord-frontend") {
  buildTool(ProjectBuildTool.CARGO)
  cargoBuildProfile(CargoBuildProfile.RELEASE)
  includeDebugInformation(true)
  linker("rust-lld")
}

