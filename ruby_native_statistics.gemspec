# frozen_string_literal: true

require_relative 'lib/ruby_native_statistics/version'

Gem::Specification.new do |spec|
  spec.authors = ['Cory Buecker']
  spec.description = 'A Ruby gem providing high-performance statistical functions implemented in Rust for better performance than pure Ruby implementations.'
  spec.email = ['cory.buecker@gmail.com']
  spec.extensions = %w[ext/ruby_native_statistics/extconf.rb]
  spec.homepage = 'https://github.com/corybuecker/ruby-native-statistics'
  spec.license = 'Unlicense'

  spec.metadata['allowed_push_host'] = 'https://rubygems.org'
  spec.metadata['changelog_uri'] = "#{spec.homepage}/CHANGELOG.md"
  spec.metadata['source_code_uri'] = spec.homepage
  spec.metadata['bug_tracker_uri'] = "#{spec.homepage}/issues"
  spec.metadata['documentation_uri'] = "#{spec.homepage}/blob/main/README.md"

  spec.name = 'ruby_native_statistics'
  spec.summary = 'High performance, native (Rust) implementations of various statistical functions.'

  gemspec = File.basename(__FILE__)

  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[.ruby-version bin/ Gemfile .gitignore test/ spec/ features/ .github/])
    end
  end

  spec.require_paths = ['lib']

  spec.platform = Gem::Platform::RUBY
  spec.required_ruby_version = '>= 3.2.8'

  spec.add_dependency 'rb_sys', '~> 0.9.91'

  spec.add_development_dependency 'rake-compiler', '~> 1.3'
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency 'minitest', '~> 6.0'

  spec.version = RubyNativeStatistics::VERSION
end
