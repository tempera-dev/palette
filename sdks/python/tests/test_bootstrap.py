import importlib
import types

import pytest

import palette.bootstrap as bootstrap
from palette import PaletteConfig


@pytest.fixture(autouse=True)
def _clear_bootstrap_env(monkeypatch):
    for name in (
        "PALETTE_AUTO_INSTRUMENT",
        "PALETTE_AUTO_INSTRUMENT_PROVIDERS",
        "PALETTE_BASE_URL",
        "PALETTE_TENANT_ID",
        "PALETTE_PROJECT_ID",
        "PALETTE_ENVIRONMENT_ID",
        "PALETTE_API_KEY",
        "PALETTE_PROTOCOL",
        "PALETTE_GRPC_ENDPOINT",
        "PALETTE_SERVICE_NAME",
        "PALETTE_RELEASE_ID",
    ):
        monkeypatch.delenv(name, raising=False)


def test_importing_bootstrap_does_not_initialize_tracing(monkeypatch):
    calls = []
    monkeypatch.setattr(bootstrap.tracing, "init", lambda: calls.append("init"))

    importlib.reload(bootstrap)

    assert calls == []


def test_bootstrap_from_env_initializes_existing_tracing_api(monkeypatch):
    monkeypatch.setenv("PALETTE_BASE_URL", "http://palette.local:8080")
    monkeypatch.setenv("PALETTE_TENANT_ID", "tenant-a")
    monkeypatch.setenv("PALETTE_PROJECT_ID", "project-a")
    monkeypatch.setenv("PALETTE_ENVIRONMENT_ID", "prod")
    monkeypatch.setenv("PALETTE_SERVICE_NAME", "worker")
    monkeypatch.setattr(bootstrap.tracing, "get_config", lambda: None)
    monkeypatch.setattr(bootstrap.tracing, "init", PaletteConfig.resolve)

    result = bootstrap.bootstrap_from_env()

    assert result.tracing_initialized is True
    assert result.config.base_url == "http://palette.local:8080"
    assert result.config.tenant_id == "tenant-a"
    assert result.config.project_id == "project-a"
    assert result.config.environment_id == "prod"
    assert result.config.service_name == "worker"
    assert result.auto_instrumentation == ()


def test_bootstrap_reuses_existing_config_unless_forced(monkeypatch):
    existing = PaletteConfig.resolve(tenant_id="already-ready")
    monkeypatch.setattr(bootstrap.tracing, "get_config", lambda: existing)
    monkeypatch.setattr(
        bootstrap.tracing,
        "init",
        lambda: pytest.fail("init should not be called"),
    )

    result = bootstrap.bootstrap_from_env()

    assert result.tracing_initialized is False
    assert result.config is existing


def test_auto_instrumentation_is_explicit_and_delegates_to_palette_auto(monkeypatch):
    captured = []

    def instrument(*, providers):
        captured.append(providers)
        return (
            types.SimpleNamespace(
                provider="openai",
                instrumented=True,
                status="patched OpenAI",
            ),
            types.SimpleNamespace(
                provider="anthropic",
                instrumented=True,
                status="patched Anthropic",
            ),
        )

    monkeypatch.setenv("PALETTE_AUTO_INSTRUMENT", "openai,anthropic")
    monkeypatch.setattr(
        bootstrap.importlib,
        "import_module",
        lambda name: types.SimpleNamespace(instrument=instrument),
    )
    monkeypatch.setattr(bootstrap.tracing, "get_config", lambda: None)
    monkeypatch.setattr(bootstrap.tracing, "init", PaletteConfig.resolve)

    result = bootstrap.bootstrap_from_env()

    assert captured == [("openai", "anthropic")]
    assert [status.provider for status in result.auto_instrumentation] == ["openai", "anthropic"]
    assert all(status.instrumented for status in result.auto_instrumentation)


def test_auto_instrumentation_truthy_env_requests_all_providers(monkeypatch):
    captured = []

    def instrument(*, providers):
        captured.append(providers)
        return (types.SimpleNamespace(provider="all", instrumented=True, status="patched all"),)

    monkeypatch.setenv("PALETTE_AUTO_INSTRUMENT", "true")
    monkeypatch.setattr(
        bootstrap.importlib,
        "import_module",
        lambda name: types.SimpleNamespace(instrument=instrument),
    )
    monkeypatch.setattr(bootstrap.tracing, "get_config", lambda: None)
    monkeypatch.setattr(bootstrap.tracing, "init", PaletteConfig.resolve)

    result = bootstrap.bootstrap_from_env()

    assert captured == [None]
    assert result.auto_instrumentation[0].provider == "all"


def test_auto_instrumentation_reports_missing_support(monkeypatch):
    def missing_auto(name):
        raise ImportError(name)

    monkeypatch.setenv("PALETTE_AUTO_INSTRUMENT", "openai")
    monkeypatch.setattr(bootstrap.importlib, "import_module", missing_auto)
    monkeypatch.setattr(bootstrap.tracing, "get_config", lambda: None)
    monkeypatch.setattr(bootstrap.tracing, "init", PaletteConfig.resolve)

    result = bootstrap.bootstrap_from_env()

    assert result.auto_instrumentation == (
        bootstrap.AutoInstrumentationStatus(
            provider="openai",
            instrumented=False,
            status="palette.auto is not installed",
        ),
    )


def test_configurator_entry_point_runs_bootstrap(monkeypatch):
    calls = []
    monkeypatch.setattr(bootstrap, "bootstrap_from_env", lambda: calls.append("called"))

    bootstrap.PaletteConfigurator().configure()

    assert calls == ["called"]
