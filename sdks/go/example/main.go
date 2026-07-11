// Quickstart: emit an agent.run -> agent.plan -> llm.call trace to Palette.
//
//	PALETTE_TENANT_ID=demo PALETTE_PROJECT_ID=demo PALETTE_ENVIRONMENT_ID=local \
//	    go run ./example
package main

import (
	"context"
	"log"

	palette "github.com/jadenfix/palette/sdks/go"
	"go.opentelemetry.io/otel/trace"
)

func main() {
	ctx := context.Background()

	cfg := palette.ConfigFromEnv()
	cfg.ServiceName = "palette-go-quickstart"
	cfg.ReleaseID = "quickstart"

	shutdown, err := palette.Init(ctx, cfg)
	if err != nil {
		log.Fatalf("palette init: %v", err)
	}
	defer func() { _ = shutdown(ctx) }()

	err = palette.Observe(ctx, "handle_refund", palette.KindAgentRun, func(ctx context.Context) error {
		log.Printf("trace_id=%s", trace.SpanContextFromContext(ctx).TraceID())
		palette.SetInput(ctx, "late delivery refund after 31 days")

		var plan string
		if err := palette.Observe(ctx, "make_plan", palette.KindAgentPlan, func(ctx context.Context) error {
			plan = "look up refund policy"
			palette.SetOutput(ctx, plan)
			return nil
		}); err != nil {
			return err
		}

		return palette.Observe(ctx, "call_model", palette.KindLLMCall, func(ctx context.Context) error {
			palette.SetInput(ctx, plan)
			palette.SetOutput(ctx, "Escalate: order is outside the standard refund window.")
			return nil
		})
	})
	if err != nil {
		log.Fatalf("agent run: %v", err)
	}

	if err := palette.Flush(ctx); err != nil {
		log.Printf("flush: %v", err)
	}
	log.Println("trace flushed -- open the dashboard to inspect it")
}
