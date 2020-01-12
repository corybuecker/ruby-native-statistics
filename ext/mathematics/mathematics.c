#include "stdbool.h"
#include "ruby.h"
#include "mathematics.h"

void Init_mathematics()
{
  MathematicsModule = rb_define_module("Mathematics");
  rb_define_method(MathematicsModule, "mean", rb_mean, 0);
  rb_define_method(MathematicsModule, "median", rb_median, 0);
}

double calculate_mean(VALUE array, unsigned long array_length)
{
  unsigned long i;
  double total = 0;
  double mean = 0;

  for (i = 0; i < array_length; i++)
  {
    total += rb_num2dbl(rb_ary_entry(array, i));
  }

  mean = total / array_length;

  return mean;
}

double calculate_total_distance_from_mean(VALUE array, unsigned long array_length)
{
  unsigned long i;
  double mean = 0;
  double total_distance_from_mean = 0;

  mean = calculate_mean(array, array_length);

  for (i = 0; i < array_length; i++)
  {
    total_distance_from_mean += pow((rb_num2dbl(rb_ary_entry(array, i)) - mean), 2);
  }

  return total_distance_from_mean;
}

VALUE rb_mean(VALUE self)
{
  unsigned int array_length;

  Check_Type(self, T_ARRAY);

  array_length = rb_long2int(RARRAY_LEN(self));

  if (array_length <= 0)
  {
    rb_raise(rb_eRangeError, "array must have at least one element");
  }

  return DBL2NUM(calculate_mean(self, array_length));
}

int compare_doubles(const void *a, const void *b)
{
  double *dbl_a = (double *)a;
  double *dbl_b = (double *)b;

  double cmp_a = *dbl_a;
  double cmp_b = *dbl_b;

  return (cmp_a - cmp_b);
}

VALUE rb_median(VALUE self)
{
  unsigned long array_length;
  unsigned long i;
  double *working_array;
  VALUE result;

  Check_Type(self, T_ARRAY);

  array_length = RARRAY_LEN(self);

  if (array_length <= 0)
  {
    rb_raise(rb_eRangeError, "array must have at least one element");
  }

  bool array_even_size = (array_length % 2) == 0;
  unsigned long middle = (long)floor(array_length / 2.0);

  working_array = malloc(array_length * sizeof(double));

  if (working_array == NULL)
  {
    rb_raise(rb_eStandardError, "unknown problem calculating median (possibly array is too large)");
  }

  for (i = 0; i < array_length; i++)
  {
    VALUE item = rb_ary_entry(self, i);

    if (!RB_INTEGER_TYPE_P(item) && !RB_FLOAT_TYPE_P(item))
    {
      free(working_array);
      rb_raise(rb_eTypeError, "element is not a number");
    }

    working_array[i] = NUM2DBL(item);
  }

  // Reminder to myself as I'm learning C. Using an array as a function parameter decays that reference
  // to a pointer to the first element in the array.
  // https://www.gnu.org/software/gnu-c-manual/gnu-c-manual.html#Function-Parameters
  qsort(working_array, array_length, sizeof(double), compare_doubles);

  if (!array_even_size)
  {
    result = DBL2NUM(working_array[middle]);
  }
  else
  {
    result = DBL2NUM((working_array[middle - 1] + working_array[middle]) / 2);
  }

  free(working_array);

  return result;
}
