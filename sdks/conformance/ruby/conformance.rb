# frozen_string_literal: true

# Live conformance: drive the GENERATED Ruby control-plane client against a
# running beaterd and verify typed request/response shapes match the API.
#
# Proves API-shape == SDK-shape for Ruby. Run via run.sh.

require "uri"
require "beater_client"

base = ENV.fetch("BEATER_BASE_URL").sub(%r{/+\z}, "")
tenant = ENV.fetch("BEATER_TENANT", "demo")
project = ENV.fetch("BEATER_PROJECT", "demo")
uri = URI.parse(base)

BeaterClient.configure do |config|
  config.scheme = uri.scheme
  config.host = "#{uri.host}:#{uri.port}"
  config.server_index = nil
end

begin
  # 1. health -> typed response
  health = BeaterClient::HealthApi.new.health
  raise "health.ok != true: #{health.inspect}" unless health.ok == true

  puts "  health: ok=#{health.ok}"

  # 2. create dataset -> typed request body + typed response (shape parity)
  req = BeaterClient::CreateDatasetRequest.new(name: "conformance-ruby")
  created = BeaterClient::DatasetsApi.new.create_dataset(tenant, project, req)
  puts "  createDataset -> #{created.class.name.split('::').last}"

  # 3. list traces -> typed page response
  page = BeaterClient::TracesApi.new.list_traces(tenant)
  raise "traces.list page missing 'items': #{page.inspect}" if page.items.nil?

  puts "  traces.list -> #{page.class.name.split('::').last} items=#{page.items.length}"

  puts "PASS: ruby generated client round-trips against live API"
rescue StandardError => e
  warn "FAIL: #{e.class}: #{e.message}"
  exit 1
end
