# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'ruby_native_statistics/version'

Gem::Specification.new do |spec|
  spec.name          = "ruby_native_statistics"
  spec.version       = RubyNativeStatistics::VERSION
  spec.authors       = ["Cory Buecker"]
  spec.email         = ["email@corybuecker.com"]
  spec.license       = "Unlicense"
  spec.summary       = "This is a native extension to Ruby that adds various statistical functions to the Array class."
  spec.homepage      = "https://github.com/corybuecker/ruby-native-statistics"

  spec.metadata['allowed_push_host'] = "https://rubygems.org"

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.require_paths = ["lib"]

  spec.extensions << "ext/dispersion/extconf.rb"

  spec.add_development_dependency "bundler", "~> 1.11"
  spec.add_development_dependency "rake", "~> 11.0"
  spec.add_development_dependency "minitest", "~> 5.0"
  spec.add_development_dependency "rake-compiler", "~> 0.9"
end
