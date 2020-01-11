require "rake/extensiontask"
require "rake/testtask"

Rake::ExtensionTask.new "dispersion" do |ext|
  ext.lib_dir = "lib/dispersion"
end

Rake::ExtensionTask.new "mathematics" do |ext|
  ext.lib_dir = "lib/mathematics"
end

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_test.rb"]
end

Rake::TestTask.new(:benchmark) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/*_benchmark.rb"]
end

task :default => [:compile, :test]
