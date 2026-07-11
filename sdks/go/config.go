package palette

import (
	"os"
	"strings"
)

// Config holds the connection and scope settings for the Palette SDK.
type Config struct {
	BaseURL       string
	TenantID      string
	ProjectID     string
	EnvironmentID string
	APIKey        string
	ServiceName   string
	ReleaseID     string
}

func env(name, def string) string {
	if v := os.Getenv(name); v != "" {
		return v
	}
	return def
}

// ConfigFromEnv resolves a Config from PALETTE_* environment variables, with
// sensible local defaults (http://127.0.0.1:8080, demo/demo/local).
func ConfigFromEnv() Config {
	return Config{
		BaseURL:       env("PALETTE_BASE_URL", "http://127.0.0.1:8080"),
		TenantID:      env("PALETTE_TENANT_ID", "demo"),
		ProjectID:     env("PALETTE_PROJECT_ID", "demo"),
		EnvironmentID: env("PALETTE_ENVIRONMENT_ID", "local"),
		APIKey:        env("PALETTE_API_KEY", ""),
		ServiceName:   env("PALETTE_SERVICE_NAME", "palette-go"),
		ReleaseID:     env("PALETTE_RELEASE_ID", ""),
	}
}

// otlpTracesURL builds the OTLP/HTTP traces endpoint for this scope:
// {base}/v1/otlp/{tenant}/{project}/{environment}/v1/traces
func (c Config) otlpTracesURL() string {
	base := strings.TrimRight(c.BaseURL, "/")
	return base + "/v1/otlp/" + c.TenantID + "/" + c.ProjectID + "/" + c.EnvironmentID + "/v1/traces"
}
