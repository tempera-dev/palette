// Live conformance: drive the GENERATED C# control-plane client against a
// running beaterd and verify typed request/response shapes match the API.
//
// Proves API-shape == SDK-shape for C#. Run via run.sh.

using Beater.Client.Api;
using Beater.Client.Model;

var baseUrl = (Environment.GetEnvironmentVariable("BEATER_BASE_URL")
    ?? throw new InvalidOperationException("BEATER_BASE_URL must be set")).TrimEnd('/');
var tenant = Environment.GetEnvironmentVariable("BEATER_TENANT") ?? "demo";
var project = Environment.GetEnvironmentVariable("BEATER_PROJECT") ?? "demo";

try
{
    // 1. health -> typed response
    var health = new HealthApi(baseUrl).Health();
    if (health.Ok != true)
    {
        throw new Exception($"health.ok != true: {health}");
    }
    Console.WriteLine($"  health: ok={health.Ok}");

    // 2. create dataset -> typed request body + typed response (shape parity)
    var created = new DatasetsApi(baseUrl)
        .CreateDataset(tenant, project, new CreateDatasetRequest(name: "conformance-csharp"));
    Console.WriteLine($"  createDataset -> {created.GetType().Name}");

    // 3. list traces -> typed page response
    var page = new TracesApi(baseUrl).ListTraces(tenant);
    if (page.Items == null)
    {
        throw new Exception($"traces.list page missing Items: {page}");
    }
    Console.WriteLine($"  traces.list -> {page.GetType().Name} items={page.Items.Count}");

    Console.WriteLine("PASS: csharp generated client round-trips against live API");
    return 0;
}
catch (Exception e)
{
    Console.Error.WriteLine($"FAIL: {e.GetType().Name}: {e.Message}");
    return 1;
}
