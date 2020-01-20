require "test_helper"
require "minitest/benchmark"

class RubyNativeStatisticsBenchmark < Minitest::Benchmark
  def bench_native_stdev
    array = (1..10_000).to_a

    assert_performance_constant 0.99 do |input|
      array.to_a.stdev
    end
  end

  def bench_ruby_stdev
    array = (1..10_000).to_a

    assert_performance_constant 0.99 do |input|
      total = array.inject(0) { |inner_total, element| inner_total + element }
      mean = total.to_f / array.size
      total_distance_from_mean = array.inject(0) { |inner_total, element| inner_total + (element - mean) ** 2 }
      Math.sqrt(total_distance_from_mean / (array.size - 1))
    end
  end

  def bench_native_mean
    array = (1..10_000).to_a

    assert_performance_constant 0.99 do |input|
      array.mean
    end
  end

  def bench_ruby_mean
    array = (1..10_000).to_a

    assert_performance_constant 0.99 do |input|
      array.inject { |sum, el| sum + el }.to_f / array.size
    end
  end

  def bench_native_median
    array = (1..10_000).to_a.shuffle

    assert_performance_constant 0.99 do |input|
      array.median
    end
  end

  def bench_ruby_median
    array = (1..10_000).to_a.shuffle

    assert_performance_constant 0.99 do |input|
      new_array = array.sort
      elements = new_array.length
      middle = elements / 2
      elements.even? ? (new_array[middle] + new_array[middle + 1]) / 2.0 : new_array[middle]
    end
  end
end
