projects {
  project("discord-frontend", ProjectType.RUST)
  project("web-frontend", ProjectType.TYPESCRIPT)
}

buildConfigurationForProject("discord-frontend") {
  buildTool(ProjectBuildTool.CARGO)
  cargoBuildProfile(CargoBuildProfile.RELEASE)
  includeDebugInformation(true)
  linker("rust-lld")
}

buildConfigurationForProject("web-frontend") {
  buildTool(ProjectBuildTool.YARN)
  outputDirectory("dist")
}
