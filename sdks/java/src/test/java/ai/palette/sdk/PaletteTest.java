package ai.palette.sdk;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;

import io.opentelemetry.sdk.testing.exporter.InMemorySpanExporter;
import io.opentelemetry.sdk.trace.data.SpanData;
import io.opentelemetry.sdk.trace.export.SimpleSpanProcessor;
import java.util.List;
import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

class PaletteTest {

    private InMemorySpanExporter exporter;

    @BeforeEach
    void setUp() {
        exporter = InMemorySpanExporter.create();
        PaletteConfig cfg = new PaletteConfig();
        cfg.releaseId = "rel-42";
        Palette.init(cfg, SimpleSpanProcessor.create(exporter));
    }

    @AfterEach
    void tearDown() {
        Palette.shutdown();
    }

    @Test
    void observeSetsKindReleaseAndOutput() {
        String result = Palette.observe("answer", SemConv.LLM_CALL, () -> {
            Palette.setInput("hello");
            Palette.setOutput("world");
            return "world";
        });
        assertEquals("world", result);

        List<SpanData> spans = exporter.getFinishedSpanItems();
        assertEquals(1, spans.size());
        SpanData span = spans.get(0);

        assertEquals("answer", span.getName());
        assertEquals(
                SemConv.LLM_CALL,
                span.getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.stringKey(SemConv.SPAN_KIND)));
        assertEquals(
                "rel-42",
                span.getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.stringKey(SemConv.RELEASE_ID)));
        assertEquals(
                "hello",
                span.getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.stringKey(SemConv.INPUT_VALUE)));
        assertEquals(
                "world",
                span.getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.stringKey(SemConv.OUTPUT_VALUE)));
        assertNotNull(
                span.getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.longKey(SemConv.SEQ)));
    }

    @Test
    void runnableOverloadProducesSpan() {
        Palette.observe("plan", SemConv.AGENT_PLAN, () -> Palette.setOutput("ok"));
        List<SpanData> spans = exporter.getFinishedSpanItems();
        assertEquals(1, spans.size());
        assertEquals(
                SemConv.AGENT_PLAN,
                spans.get(0).getAttributes().get(
                        io.opentelemetry.api.common.AttributeKey.stringKey(SemConv.SPAN_KIND)));
    }

    @Test
    void semconvHasElevenSpanKinds() {
        assertEquals(11, SemConv.SPAN_KINDS.size());
    }
}
