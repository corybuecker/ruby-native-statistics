require "mkmf"

abort "missing pow()" unless have_func "pow"
abort "missing sqrt()" unless have_func "sqrt"
abort "missing malloc()" unless have_func "malloc"
abort "missing free()" unless have_func "free"
abort "missing trunc()" unless have_func "trunc"

create_makefile "ruby_native_statistics/ruby_native_statistics"
