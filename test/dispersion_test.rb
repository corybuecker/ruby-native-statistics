# frozen_string_literal: true

require 'test_helper'

class DispersionTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::RubyNativeStatistics::VERSION
  end

  def test_sample_standard_deviation_with_empty_array
    assert_raises RangeError do
      [].stdev
    end
  end

  def test_population_standard_deviation_with_empty_array
    assert_raises RangeError do
      [].stdevp
    end
  end

  def test_empty_array_error_message
    begin
      [].stdevs
    rescue RangeError => e
      assert_equal e.to_s, 'Array must have at least one element'
    end
  end

  def test_sample_standard_deviation
    assert_in_delta 13.65039682, ((1..10).to_a + [-41, 0]).stdev, 0.00001
  end

  def test_sample_standard_deviation_with_single_element
    assert [1].stdev.nan?
  end

  def test_sample_variance
    assert_in_delta 186.3333333, ((1..10).to_a + [-41, 0]).var, 0.00001
  end

  def test_sample_standard_deviation_with_non_number
    assert_raises TypeError do
      ((1..10).to_a + [-41, 'a', 0]).stdev
    end
  end

  def test_population_standard_deviation
    assert_in_delta 12.55660528, ((1..10).to_a + [-41, 1.32, 0]).stdevp, 0.00001
  end

  def test_population_variance
    assert_in_delta 157.6683361, ((1..10).to_a + [-41, 1.32, 0]).varp, 0.00001
  end

  def test_population_standard_deviation_with_non_number
    assert_raises TypeError do
      ((1..10).to_a + [-41, 'a', 0]).stdevp
    end
  end

  def test_percentile_errors
    assert_raises RangeError do
      (1..10).to_a.percentile(1.1)
    end

    assert_raises RangeError do
      (1..10).to_a.percentile(-0.1)
    end

    assert_raises TypeError do
      (1..10).to_a.percentile('a')
    end

    assert_raises ArgumentError do
      (1..10).to_a.percentile()
    end

    assert_raises RangeError do
      [].percentile(0.5)
    end

    assert_raises TypeError do
      ((1..10).to_a + [-41, 'a', 0]).percentile(1)
    end
  end

  def test_percentile_odd_elements
    array = [1, 5, 15].to_a.shuffle

    assert_equal 1.0, array.percentile(0)
    assert_equal 4.2, array.percentile(0.4)
    assert_equal 5.0, array.percentile(0.5)
    assert_in_delta 7.0, array.percentile(0.6), 0.000001
    assert_in_delta 7.5695, array.percentile(0.628475), 0.000001
    assert_equal 10.0, array.percentile(0.75)
    assert_in_delta 14.99998, array.percentile(0.999999), 0.000001
    assert_equal 15.0, array.percentile(1)
  end

  def test_percentile_even_elements
    array = (1..22).to_a.shuffle

    assert_equal 1.0, array.percentile(0)
    assert_equal 7.2681997, array.percentile(0.2984857)
    assert_equal 9.4, array.percentile(0.4)
    assert_equal 11.5, array.percentile(0.5)
    assert_equal 13.6, array.percentile(0.6)
    assert_equal 16.75, array.percentile(0.75)
    assert_equal 22.0, array.percentile(1)
  end

  def test_percentile_quartiles
    assert_equal 1.0, (1..9).to_a.percentile(0)
    assert_equal 5.0, (1..9).to_a.percentile(0.5)
    assert_equal 7.0, (1..9).to_a.percentile(0.75)
    assert_equal 9.0, (1..9).to_a.percentile(1)
  end

  def test_percentile_single_element
    assert_equal 5.0, [5].percentile(0)
    assert_equal 5.0, [5].percentile(0.4827475)
    assert_equal 5.0, [5].percentile(1)
  end

  def test_percentile_two_elements
    array = [45, -937].to_a.shuffle

    assert_equal(-937.0, array.percentile(0))
    assert_in_delta(-462.941955, array.percentile(0.4827475), 0.000001)
    assert_equal 45.0, array.percentile(1)
  end

  def test_percentile_floats
    array = [0.83272, 12, 9.3745765, -92.928].to_a.shuffle

    assert_in_delta 0.73895928, array.percentile(0.333), 0.000001
    assert_in_delta 11.43290852, array.percentile(0.928), 0.000001
  end

  def test_percentile_repeating
    array = [5.4, 5.3, 5.2, 5.4, 5.2].to_a.shuffle

    assert_in_delta 5.4, array.percentile(0.9), 0.000001
    assert_in_delta 5.2, array.percentile(0.1), 0.000001
    assert_in_delta 5.3, array.percentile(0.5), 0.000001
    assert_in_delta 5.26, array.percentile(0.4), 0.000001
  end

  def test_percentile_repeating_many
    array = [5.4, 5.3, 5.2, 5.4, 5.2, 4.6, 4.7, 5.0, 5.1, 5.0, 5.4, 4.6, 5.2, 4.6, 4.7, 5.2, 5.4, 5.0,
             5.3, 5.3, 6.0, 5.1, 5.5, 5.5, 5.5, 5.6, 5.8, 5.2, 4.9, 5.5, 5.7, 5.9].to_a.shuffle

    assert_in_delta 4.6, array.percentile(0.0), 0.00000001
    assert_in_delta 4.7, array.percentile(0.1), 0.00000001
    assert_in_delta 5.0, array.percentile(0.2), 0.00000001
    assert_in_delta 5.1, array.percentile(0.3), 0.00000001
    assert_in_delta 5.2, array.percentile(0.4), 0.00000001
    assert_in_delta 5.25, array.percentile(0.5), 0.00000001
    assert_in_delta 5.36, array.percentile(0.6), 0.00000001
    assert_in_delta 5.4, array.percentile(0.7), 0.00000001
    assert_in_delta 5.5, array.percentile(0.8), 0.00000001
    assert_in_delta 5.69, array.percentile(0.9), 0.00000001
    assert_in_delta 6.0, array.percentile(1.0), 0.00000001
  end

  def test_percentile_duplicates
    array = [5.2, 5.2, 5.2, 5.2, 5.2].to_a.shuffle

    assert_in_delta 5.2, array.percentile(0.9), 0.000001
  end
end
