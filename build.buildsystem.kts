projects {
  project("buildsystem", ProjectType.JVM)
  project("discord-frontend", ProjectType.RUST)
  project("rust-utilities", ProjectType.RUST)
  project("web-frontend", ProjectType.TYPESCRIPT)
}

buildConfigurationForProject("buildsystem") {
  buildTool(ProjectBuildTool.GRADLE)
}

buildConfigurationForProject("discord-frontend") {
  buildTool(ProjectBuildTool.CARGO)
  cargoBuildProfile(CargoBuildProfile.RELEASE)
  includeDebugInformation(true)
  linker("rust-lld")
}

buildConfigurationForProject("rust-utilities") {
  buildTool(ProjectBuildTool.CARGO)
  cargoBuildProfile(CargoBuildProfile.RELEASE)
  includeDebugInformation(true)
  linker("rust-lld")
}

buildConfigurationForProject("web-frontend") {
  buildTool(ProjectBuildTool.YARN)
  outputDirectory("dist")
}
