require "ruby_native_statistics/version"
require "mathematics/mathematics"
require "dispersion/dispersion"

class Array
  include Mathematics
  include Dispersion
end
