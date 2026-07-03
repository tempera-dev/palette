rootProject.name = "beater-kotlin-conformance"

// Build the generated client from source and substitute it for the
// ai.beater:beater-client-kotlin dependency declared in build.gradle.kts.
includeBuild("../../clients/kotlin")
