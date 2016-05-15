require "bundler/gem_tasks"
require "rake/testtask"
require 'rake/extensiontask'

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList['test/**/*_test.rb', "test/**/*_benchmark.rb"]
end

Rake::ExtensionTask.new("dispersion")

task :default => [:compile, :test]
