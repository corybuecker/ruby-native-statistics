require "ruby_native_statistics/version"
require "mathematics"
require "dispersion"

class Array
  include Mathematics
  include Dispersion
end
