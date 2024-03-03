#include "conversions.h"
#include "float.h"

int compare_doubles(const void *a, const void *b)
{
  double *dbl_a = (double *)a;
  double *dbl_b = (double *)b;

  double cmp_a = *dbl_a;
  double cmp_b = *dbl_b;

  if (fabs(cmp_a - cmp_b) <= (DBL_EPSILON * fabs(cmp_a + cmp_b))) 
  {
    return 0;
  }

  if (cmp_a > cmp_b)
  {
    return 1;
  }

  return -1;
}

double *sorted_ruby_array(VALUE array, long array_length)
{
  long i;
  double *working_array;

  working_array = malloc(array_length * sizeof(double));

  if (working_array == NULL)
  {
    rb_raise(rb_eStandardError, "unknown problem sorting array (possibly array is too large)");
  }

  for (i = 0; i < array_length; i++)
  {
    VALUE item = rb_ary_entry(array, i);

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

  return working_array;
}