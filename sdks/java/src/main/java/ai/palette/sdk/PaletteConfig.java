package ai.palette.sdk;

/**
 * Connection + scope settings for the Palette SDK.
 *
 * <p>{@link #fromEnv()} reads {@code PALETTE_*} environment variables with local
 * defaults so a zero-config {@code Palette.init(PaletteConfig.fromEnv())} works
 * when the environment is configured.
 */
public final class PaletteConfig {

    public String baseUrl;
    public String tenantId;
    public String projectId;
    public String environmentId;
    public String apiKey;
    public String serviceName;
    public String releaseId;

    public PaletteConfig() {
        this.baseUrl = "http://127.0.0.1:8080";
        this.tenantId = "demo";
        this.projectId = "demo";
        this.environmentId = "local";
        this.serviceName = "palette-java";
    }

    private static String env(String name, String fallback) {
        String value = System.getenv(name);
        return (value != null && !value.isEmpty()) ? value : fallback;
    }

    /** Resolve config from {@code PALETTE_*} env vars with sensible local defaults. */
    public static PaletteConfig fromEnv() {
        PaletteConfig c = new PaletteConfig();
        c.baseUrl = env("PALETTE_BASE_URL", "http://127.0.0.1:8080");
        c.tenantId = env("PALETTE_TENANT_ID", "demo");
        c.projectId = env("PALETTE_PROJECT_ID", "demo");
        c.environmentId = env("PALETTE_ENVIRONMENT_ID", "local");
        c.apiKey = env("PALETTE_API_KEY", null);
        c.serviceName = env("PALETTE_SERVICE_NAME", "palette-java");
        c.releaseId = env("PALETTE_RELEASE_ID", null);
        return c;
    }

    /** The OTLP/HTTP traces endpoint, with tenant/project/environment in the path. */
    public String otlpHttpTracesUrl() {
        String base = baseUrl;
        while (base.endsWith("/")) {
            base = base.substring(0, base.length() - 1);
        }
        return base + "/v1/otlp/" + tenantId + "/" + projectId + "/" + environmentId + "/v1/traces";
    }
}
