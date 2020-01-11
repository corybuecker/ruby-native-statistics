require "test_helper"

class TestClass
  include Mathematics
end

class MathematicsTest < Minitest::Test
  def test_that_it_has_a_version_number
    refute_nil ::RubyNativeStatistics::VERSION
  end

  def test_exception_for_non_array
    assert_raises TypeError do
      TestClass.new.mean
    end
  end

  def test_mean
    assert_in_delta 1.1784615, ((1..10).to_a + [-41, 1.32, 0]).mean, 0.00001
  end

  def test_mean_with_non_number
    assert_raises TypeError do
      ((1..10).to_a + [-41, "a", 0]).mean
    end
  end

  def test_mean_with_empty_array
    assert_raises RangeError do
      [].mean
    end
  end
end
