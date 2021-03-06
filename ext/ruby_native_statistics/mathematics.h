#include <stdbool.h>
#include <ruby.h>
#include "conversions.h"

VALUE rb_mean(VALUE self);
VALUE rb_median(VALUE self);

double calculate_mean(VALUE array, unsigned long array_length);
double calculate_total_distance_from_mean(VALUE array, unsigned long array_length);