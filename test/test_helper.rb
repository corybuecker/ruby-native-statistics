require "simplecov"
require "simplecov-lcov"

SimpleCov::Formatter::LcovFormatter.config do |c|
  c.single_report_path = "coverage/lcov.info"
  c.report_with_single_file = true
end

SimpleCov.formatter = SimpleCov::Formatter::LcovFormatter
SimpleCov.start

require "ruby_native_statistics"

require "minitest/reporters"
Minitest::Reporters.use! [Minitest::Reporters::SpecReporter.new]

require "minitest/autorun"
