# Ruby Native Statistics ![](https://github.com/corybuecker/ruby-native-statistics/workflows/Test%20suite/badge.svg) [![Coverage Status](https://coveralls.io/repos/github/corybuecker/ruby-native-statistics/badge.svg?branch=master)](https://coveralls.io/github/corybuecker/ruby-native-statistics?branch=master)

This is a native extension to Ruby that adds native (C) statistical functions to the Array class. At present the following functions are provided:

- [Sample Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Corrected_sample_standard_deviation) (stdev, stdevs)
- [Population Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Uncorrected_sample_standard_deviation) (stdevp)
- [Sample Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (var)
- [Population Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (varp)
- [Median](https://en.wikipedia.org/wiki/Median) (median)
- [Mean](https://en.wikipedia.org/wiki/Arithmetic_mean) (mean)

Check the Github Actions build to see the currently supported versions of Ruby. This list will match whatever stable versions are specified at https://www.ruby-lang.org/en/downloads/.

It is much more performant than calculating the standard deviation with pure Ruby. For a comparison, run the benchmarks with `rake benchmark`.

    bench_native_dispersion	 0.000425	 0.000341	 0.000420	 0.000324	 0.000319
    bench_ruby_dispersion	 0.002168	 0.002156	 0.002148	 0.002149	 0.002151

## Usage

    require 'ruby_native_statistics'
    r = [1,3,21,32,42]

    # calculate sample standard deviation, you can also use "stdevs"
    p r.stdev

    # calculate population standard deviation
    p r.stdevp

    # calculate mean
    p r.mean

    # calculate median
    p r.median

## Links

This is the third version of this gem, and it is a total rewrite of a SWIG-based design. Lots of thanks to the following resources:

- https://blog.jcoglan.com/2012/07/29/your-first-ruby-native-extension-c/
- https://github.com/andremedeiros/ruby-c-cheat-sheet
- http://silverhammermba.github.io/emberb/c/
- http://docs.ruby-lang.org/en/2.3.0/extension_rdoc.html
