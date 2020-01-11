#include "ruby.h"
#include "mathematics.h"

void Init_mathematics() {
  MathematicsModule = rb_define_module("Mathematics");
  rb_define_method(MathematicsModule, "mean", rb_mean, 0);
}

double calculate_mean(VALUE array, unsigned long array_length){
  unsigned long i;
  double total = 0;
  double mean = 0;

  for(i = 0; i < array_length; i++){
    total += rb_num2dbl(rb_ary_entry(array, i));
  }

  mean = total / array_length;

  return mean;
}

double calculate_total_distance_from_mean(VALUE array, unsigned long array_length){
  unsigned long i;
  double mean = 0;
  double total_distance_from_mean = 0;

  mean = calculate_mean(array, array_length);

  for(i = 0; i < array_length; i++){
    total_distance_from_mean += pow((rb_num2dbl(rb_ary_entry(array, i)) - mean), 2);
  }

  return total_distance_from_mean;
}

VALUE rb_mean(VALUE self) {
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 0) {
    rb_raise(rb_eRangeError, "array must have at least one element");
  }

  return DBL2NUM(calculate_mean(self, array_length));
}

