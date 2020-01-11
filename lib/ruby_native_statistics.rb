require "ruby_native_statistics/version"
require "dispersion/dispersion"

class Array
  include Dispersion
end
