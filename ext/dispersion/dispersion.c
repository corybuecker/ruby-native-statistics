#include "ruby.h"
#include "dispersion.h"

void Init_dispersion() {
  DispersionModule = rb_define_module("Dispersion");
  rb_define_method(DispersionModule, "stdev", rb_sample_standard_deviation, 0);
  rb_define_method(DispersionModule, "stdevs", rb_sample_standard_deviation, 0);
  rb_define_method(DispersionModule, "stdevp", rb_population_standard_deviation, 0);
  rb_define_method(DispersionModule, "var", rb_sample_variance, 0);
  rb_define_method(DispersionModule, "varp", rb_population_variance, 0);
}

double calculate_total_distance_from_mean(VALUE array, unsigned long array_length){
  unsigned long i;
  double total = 0;
  double mean = 0;
  double total_distance_from_mean = 0;

  for(i = 0; i < array_length; i++){
    total += rb_num2dbl(rb_ary_entry(array, i));
  }

  mean = total / array_length;

  for(i = 0; i < array_length; i++){
    total_distance_from_mean += pow((rb_num2dbl(rb_ary_entry(array, i)) - mean), 2);
  }

  return total_distance_from_mean;
}

VALUE rb_sample_standard_deviation(VALUE self) {
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1) {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(sqrt((calculate_total_distance_from_mean(self, array_length)/(array_length - 1))));
}

VALUE rb_sample_variance(VALUE self) {
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1) {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM((calculate_total_distance_from_mean(self, array_length)/(array_length - 1)));
}

VALUE rb_population_standard_deviation(VALUE self) {
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1) {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(sqrt(calculate_total_distance_from_mean(self, array_length) / array_length));
}

VALUE rb_population_variance(VALUE self) {
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 1) {
    rb_raise(rb_eRangeError, "array must have more than one element");
  }

  return DBL2NUM(calculate_total_distance_from_mean(self, array_length) / array_length);
}
