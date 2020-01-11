require "mkmf"

abort "missing pow()" unless have_func "pow"

create_makefile "dispersion"
