require_relative "lib/ruby_native_statistics/version"

Gem::Specification.new do |spec|
  spec.name = "ruby_native_statistics"
  spec.version = RubyNativeStatistics::VERSION
  spec.authors = ["Cory Buecker"]
  spec.email = ["cory.buecker@gmail.com"]

  spec.license = "Unlicense"
  spec.summary = "This is a native extension to Ruby that adds various statistical functions to the Array class."
  spec.homepage = "https://github.com/corybuecker/ruby-native-statistics"

  spec.required_ruby_version = Gem::Requirement.new(">= 2.4.9")

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = "#{spec.homepage}/changelog.md"

  spec.files = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.require_paths = ["lib"]

  spec.extensions = %w[ext/mathematics/extconf.rb ext/dispersion/extconf.rb]
end
