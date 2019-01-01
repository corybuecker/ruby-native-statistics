$LOAD_PATH.unshift File.expand_path('../../lib', __FILE__)

require 'coveralls'
Coveralls.wear!

require "ruby_native_statistics"
require "minitest/autorun"
