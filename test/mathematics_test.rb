# frozen_string_literal: true

require 'test_helper'

class MathematicsTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::RubyNativeStatistics::VERSION
  end

  def test_mean
    assert_in_delta 1.1784615, ((1..10).to_a + [-41, 1.32, 0]).mean, 0.00001
  end

  def test_mean_with_non_number
    assert_raises TypeError do
      ((1..10).to_a + [-41, 'a', 0]).mean
    end
  end

  def test_mean_with_empty_array
    assert_raises RangeError do
      [].mean
    end
  end

  def test_median
    assert_equal 2.0, [1, 3, 2].median
  end

  def test_median_infinity
    assert_equal 2, [-Float::INFINITY, 1, 3, 2, Float::INFINITY].median
  end

  def test_median_with_even_number_of_elements
    assert_equal 2.5, [4, 2, 1, 3].median
  end

  def test_median_with_one_element
    assert_equal 3.57, [3.57].median
  end

  def test_median_with_empty_array
    assert_raises RangeError do
      [].median
    end
  end

  def test_median_with_non_number
    assert_raises TypeError do
      ((1..10).to_a + [-41, 'a', 0]).median
    end
  end

  def test_median_with_floats
    assert_equal 1.0, [3.5, 1, -6.2583, 9.2756, -10.35757].median
  end

  def test_median_with_even_number_of_float_elements
    assert_equal 1.69235, [3.5, 2.3847, 1, -6.2583, 9.2756, -10.35757].median
  end

  def test_median_does_not_sort_array
    array = [1, 3, 2]
    array.median
    assert_equal [1, 3, 2], array
  end

  def test_median_with_large_array
    assert_equal 7500000.0, (0..15_000_000).to_a.median
  end
end
