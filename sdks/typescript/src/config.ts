/** Configuration resolved from explicit options then `PALETTE_*` env vars. */

export interface PaletteOptions {
  baseUrl?: string;
  tenantId?: string;
  projectId?: string;
  environmentId?: string;
  apiKey?: string;
  serviceName?: string;
  releaseId?: string;
}

export interface PaletteConfig {
  baseUrl: string;
  tenantId: string;
  projectId: string;
  environmentId: string;
  apiKey?: string;
  serviceName: string;
  releaseId?: string;
}

function env(name: string): string | undefined {
  return typeof process !== "undefined" && process.env ? process.env[name] : undefined;
}

export function resolveConfig(options: PaletteOptions = {}): PaletteConfig {
  return {
    baseUrl: options.baseUrl ?? env("PALETTE_BASE_URL") ?? "http://127.0.0.1:8080",
    tenantId: options.tenantId ?? env("PALETTE_TENANT_ID") ?? "demo",
    projectId: options.projectId ?? env("PALETTE_PROJECT_ID") ?? "demo",
    environmentId: options.environmentId ?? env("PALETTE_ENVIRONMENT_ID") ?? "local",
    apiKey: options.apiKey ?? env("PALETTE_API_KEY"),
    serviceName: options.serviceName ?? env("PALETTE_SERVICE_NAME") ?? "palette-node",
    releaseId: options.releaseId ?? env("PALETTE_RELEASE_ID"),
  };
}

export function otlpHttpTracesUrl(config: PaletteConfig): string {
  const base = config.baseUrl.replace(/\/+$/, "");
  return `${base}/v1/otlp/${config.tenantId}/${config.projectId}/${config.environmentId}/v1/traces`;
}
