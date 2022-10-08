plugins {
    kotlin("jvm") version "1.7.20"
}

group = "com.github.teamhartex"
version = "0.1.0"

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-scripting-jvm:1.7.20")
    implementation("org.jetbrains.kotlin:kotlin-scripting-jvm-host:1.7.20")
    implementation(project(":script-def"))
}
