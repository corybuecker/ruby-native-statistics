# Ruby Native Statistics

[![Build Status](https://travis-ci.org/corybuecker/ruby-native-statistics.svg)](https://travis-ci.org/corybuecker/ruby-native-statistics)

This is a native extension to Ruby that adds native (C) statistical functions to the Array class. At present the following functions are provided:

* [Sample Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Corrected_sample_standard_deviation) (stdev, stdevs)
* [Population Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Uncorrected_sample_standard_deviation) (stdevp)
* [Sample Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (var)
* [Population Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (varp)


Check the TravisCI build to see the currently supported versions of Ruby. This list will match whatever versions are specified at https://www.ruby-lang.org/en/downloads/.

It is much more performant than calculating the standard deviation with pure Ruby. For a comparison, run the benchmarks with rake.

    bench_native_dispersion  0.000686  0.000628  0.000595  0.000475  0.000590
    bench_ruby_dispersion    0.002630  0.004559  0.002589  0.002909  0.002812

## Usage

    require 'ruby_native_statistics'
    r = [1,3,21,32,42]

    # calculate sample standard deviation, you can also use "stdevs"
    p r.stdev

    # calculate population standard deviation
    p r.stdevp

## Links

This is the third version of this gem, and it is a total rewrite of a SWIG-based design. Lots of thanks to the following resources:

* https://blog.jcoglan.com/2012/07/29/your-first-ruby-native-extension-c/
* https://github.com/andremedeiros/ruby-c-cheat-sheet
* http://silverhammermba.github.io/emberb/c/
* http://docs.ruby-lang.org/en/2.3.0/extension_rdoc.html
