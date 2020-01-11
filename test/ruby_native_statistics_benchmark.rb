require "minitest/benchmark"
require "test_helper"

class RubyNativeStatisticsBenchmark < Minitest::Benchmark
  def bench_native_dispersion
    assert_performance_constant 0.99 do |input|
      (1..10000).to_a.stdev
    end
  end

  def bench_ruby_dispersion
    assert_performance_constant 0.99 do |input|
      array = (1..10000).to_a
      total = array.inject(0) { |inner_total, element| inner_total + element }
      mean = total.to_f / array.size
      total_distance_from_mean = array.inject(0) { |inner_total, element| inner_total + (element - mean) ** 2 }
      Math.sqrt(total_distance_from_mean / (array.size - 1))
    end
  end
end
