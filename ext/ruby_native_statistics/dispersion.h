#include <ruby.h>
#include <math.h>
#include "conversions.h"
#include "mathematics.h"

VALUE rb_sample_standard_deviation(VALUE self);
VALUE rb_population_standard_deviation(VALUE self);
VALUE rb_sample_variance(VALUE self);
VALUE rb_population_variance(VALUE self);
VALUE rb_percentile(VALUE self, VALUE percentile);
