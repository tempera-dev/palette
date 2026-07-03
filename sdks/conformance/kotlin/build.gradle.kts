plugins {
    kotlin("jvm") version "1.9.24"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    // Substituted by the included build (../../clients/kotlin) via settings.gradle.kts.
    implementation("ai.beater:beater-client-kotlin:0.1.0")
    // The generated API constructors expose okhttp3.Call.Factory in a default
    // parameter, so it must be on the consumer's compile classpath (the client
    // pulls okhttp as a non-transitive `implementation` dependency).
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
}

application {
    mainClass.set("ai.beater.conformance.MainKt")
}
