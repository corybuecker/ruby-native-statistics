VALUE MathematicsModule = Qnil;
VALUE rb_mean(VALUE self);
double calculate_mean(VALUE array, unsigned long array_length);
double calculate_total_distance_from_mean(VALUE array, unsigned long array_length);
