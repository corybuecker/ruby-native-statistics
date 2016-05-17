VALUE DispersionModule = Qnil;
VALUE rb_sample_standard_deviation(VALUE self);
VALUE rb_population_standard_deviation(VALUE self);
VALUE rb_sample_variance(VALUE self);
VALUE rb_population_variance(VALUE self);
double calculate_total_distance_from_mean(VALUE array, unsigned long array_length);
