# frozen_string_literal: true

require 'minitest/test_task'
Minitest::TestTask.create

require 'rb_sys/extensiontask'
GEMSPEC = Gem::Specification.load('ruby_native_statistics.gemspec')
RbSys::ExtensionTask.new('ruby_native_statistics', GEMSPEC) do |ext|
  ext.lib_dir = 'lib/ruby_native_statistics'
end

require 'rake/testtask'
Rake::TestTask.new benchmark: [:clean, :clobber, 'compile:release'] do |t|
  t.libs << 'test'
  t.test_files = ['test/**/*_benchmark.rb']
end

task :default => ['compile:dev', :test]
