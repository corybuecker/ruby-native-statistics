# Ruby Native Statistics

[![Test status](https://github.com/corybuecker/ruby-native-statistics/workflows/Test%20suite/badge.svg)](https://github.com/corybuecker/ruby-native-statistics/actions)

This is a native extension to Ruby that adds native (C) statistical functions to the Array class. At present the following functions are provided:

- [Sample Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Corrected_sample_standard_deviation) (stdev, stdevs)
- [Population Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Uncorrected_sample_standard_deviation) (stdevp)
- [Sample Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (var)
- [Population Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (varp)
- [Median](https://en.wikipedia.org/wiki/Median) (median)
- [Mean](https://en.wikipedia.org/wiki/Arithmetic_mean) (mean)
- [Percentile](https://en.wikipedia.org/wiki/Quantile) (percentile)

Check the Github Actions build to see the currently supported versions of Ruby. This list will match whatever stable versions are specified at https://www.ruby-lang.org/en/downloads/.

It is generally more performant than calculating these values with pure Ruby. For a comparison, run the benchmarks with `rake benchmark`.

| Test (Ruby 2.7.0)  | Run 1    | Run 2    | Run 3    | Run 4    | Run 5    |
| ------------------ | -------- | -------- | -------- | -------- | -------- |
| bench_native_stdev | 0.000105 | 0.000102 | 0.000101 | 0.000099 | 0.000132 |
| bench_ruby_stdev   | 0.001661 | 0.001695 | 0.001733 | 0.001664 | 0.002006 |

| Test (Ruby 2.7.0)   | Run 1    | Run 2    | Run 3    | Run 4    | Run 5    |
| ------------------- | -------- | -------- | -------- | -------- | -------- |
| bench_native_median | 0.001019 | 0.000955 | 0.001023 | 0.000933 | 0.000986 |
| bench_ruby_median   | 0.001206 | 0.001166 | 0.001160 | 0.001160 | 0.001162 |

| Test (Ruby 2.7.0) | Run 1    | Run 2    | Run 3    | Run 4    | Run 5    |
| ----------------- | -------- | -------- | -------- | -------- | -------- |
| bench_native_mean | 0.000051 | 0.000045 | 0.000050 | 0.000050 | 0.000049 |
| bench_ruby_mean   | 0.000457 | 0.000462 | 0.000465 | 0.000486 | 0.000475 |

## Found a bug? Need a function?

If you found a bug or need a particular function, please let me know! I work on this gem in my spare time, mainly for learning purposes. Feel free to open a PR or a Github issue and I'll take a look as soon as possible.

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

    # calculate percentile
    p r.percentile(0.3333)

## Implementation notes

### Percentile

Percentile uses the same rounding method as Excel, sometimes called R7.

## Links

This is the third version of this gem, and it is a total rewrite of a SWIG-based design. Lots of thanks to the following resources:

- https://blog.jcoglan.com/2012/07/29/your-first-ruby-native-extension-c/
- https://github.com/andremedeiros/ruby-c-cheat-sheet
- http://silverhammermba.github.io/emberb/c/
- http://docs.ruby-lang.org/en/2.3.0/extension_rdoc.html
