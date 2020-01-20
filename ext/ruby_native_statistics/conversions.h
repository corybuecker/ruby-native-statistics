#include <stdbool.h>
#include <ruby.h>

int compare_doubles(const void *a, const void *b);
double *sorted_ruby_array(VALUE array, long array_length);