#include "ruby_native_statistics.h"

void Init_ruby_native_statistics()
{
  DispersionModule = rb_define_module("Dispersion");
  rb_define_method(DispersionModule, "stdev", rb_sample_standard_deviation, 0);
  rb_define_method(DispersionModule, "stdevs", rb_sample_standard_deviation, 0);
  rb_define_method(DispersionModule, "stdevp", rb_population_standard_deviation, 0);
  rb_define_method(DispersionModule, "var", rb_sample_variance, 0);
  rb_define_method(DispersionModule, "varp", rb_population_variance, 0);
  rb_define_method(DispersionModule, "percentile", rb_percentile, 1);

  MathematicsModule = rb_define_module("Mathematics");
  rb_define_method(MathematicsModule, "mean", rb_mean, 0);
  rb_define_method(MathematicsModule, "median", rb_median, 0);
}