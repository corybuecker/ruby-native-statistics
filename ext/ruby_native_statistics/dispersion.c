#include "dispersion.h"

VALUE rb_sample_standard_deviation(VALUE self)
{
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1)
  {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(sqrt((calculate_total_distance_from_mean(self, array_length) / (array_length - 1))));
}

VALUE rb_sample_variance(VALUE self)
{
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1)
  {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM((calculate_total_distance_from_mean(self, array_length) / (array_length - 1)));
}

VALUE rb_population_standard_deviation(VALUE self)
{
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1)
  {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(sqrt(calculate_total_distance_from_mean(self, array_length) / array_length));
}

VALUE rb_population_variance(VALUE self)
{
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1)
  {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(calculate_total_distance_from_mean(self, array_length) / array_length);
}

VALUE rb_percentile(VALUE self, VALUE r_percentile)
{
  double result;
  Check_Type(self, T_ARRAY);

  long array_length = rb_array_len(self);
  double percentile = NUM2DBL(r_percentile);

  if (array_length == 0)
  {
    rb_raise(rb_eRangeError, "array must have at least one element");
  }

  if (percentile < 0 || percentile > 1)
  {
    rb_raise(rb_eRangeError, "percentile must be between 0 and 1 inclusive");
  }

  double *sorted_array = sorted_ruby_array(self, array_length);

  double h = (array_length - 1) * percentile + 1;

  if (trunc(h) == h)
  {
    result = sorted_array[(long)h - 1];
  }
  else
  {
    long h_floor = (long)trunc(h);
    result = (h - h_floor) * (sorted_array[h_floor] - sorted_array[h_floor - 1]) + sorted_array[h_floor - 1];
  }

  free(sorted_array);

  return DBL2NUM(result);
}
