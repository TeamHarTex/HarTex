plugins {
    java
    kotlin("jvm") version "1.7.21"
}

group = "com.github.teamhartex"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    runtimeClasspath(project(":script-def"))
}

tasks.withType<Jar> {
    from(configurations.runtimeClasspath)

    duplicatesStrategy = DuplicatesStrategy.EXCLUDE
}
