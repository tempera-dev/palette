// Live conformance: drive the GENERATED Kotlin control-plane client against a
// running beaterd and verify typed request/response shapes match the API.
//
// Proves API-shape == SDK-shape for Kotlin. Run via run.sh.

package ai.beater.conformance

import ai.beater.client.kotlin.apis.DatasetsApi
import ai.beater.client.kotlin.apis.HealthApi
import ai.beater.client.kotlin.apis.TracesApi
import ai.beater.client.kotlin.models.CreateDatasetRequest
import kotlin.system.exitProcess

fun main() {
    val base = (System.getenv("BEATER_BASE_URL")
        ?: error("BEATER_BASE_URL must be set")).trimEnd('/')
    val tenant = System.getenv("BEATER_TENANT") ?: "demo"
    val project = System.getenv("BEATER_PROJECT") ?: "demo"

    try {
        // 1. health -> typed response
        val health = HealthApi(base).health()
        check(health.ok) { "health.ok != true: $health" }
        println("  health: ok=${health.ok}")

        // 2. create dataset -> typed request body + typed response (shape parity)
        val created = DatasetsApi(base)
            .createDataset(tenant, project, CreateDatasetRequest(name = "conformance-kotlin"))
        println("  createDataset -> ${created::class.simpleName}")

        // 3. list traces -> typed page response
        val page = TracesApi(base).listTraces(tenant)
        println("  traces.list -> ${page::class.simpleName} items=${page.items.size}")

        println("PASS: kotlin generated client round-trips against live API")
    } catch (e: Throwable) {
        System.err.println("FAIL: ${e::class.simpleName}: ${e.message}")
        exitProcess(1)
    }
}
