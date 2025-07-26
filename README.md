# Ruby Native Statistics

[![Test status](https://github.com/corybuecker/ruby-native-statistics/workflows/Test%20suite/badge.svg)](https://github.com/corybuecker/ruby-native-statistics/actions)

This is a native extension to Ruby that adds native (Rust) statistical functions to the Array class. At present the following functions are provided:

- [Sample Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Corrected_sample_standard_deviation) (stdev, stdevs)
- [Population Standard Deviation](https://en.wikipedia.org/wiki/Standard_deviation#Uncorrected_sample_standard_deviation) (stdevp)
- [Sample Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (var)
- [Population Variance](https://en.wikipedia.org/wiki/Variance#Population_variance_and_sample_variance) (varp)
- [Median](https://en.wikipedia.org/wiki/Median) (median)
- [Mean](https://en.wikipedia.org/wiki/Arithmetic_mean) (mean)
- [Percentile](https://en.wikipedia.org/wiki/Quantile) (percentile)

Check the Github Actions build to see the currently supported versions of Ruby. This list will match whatever stable versions are specified at https://www.ruby-lang.org/en/downloads/.

It is generally more performant than calculating these values with pure Ruby. For a comparison, run the benchmarks with `rake benchmark`.

Here's the data converted to a Markdown table:

| Benchmark (Ruby 3.4.4)  | Run 1    | Run 2    | Run 3    | Run 4    | Run 5    |
|--------------------------|----------|----------|----------|----------|----------|
| bench_ruby_median        | 0.000676 | 0.000611 | 0.000619 | 0.000620 | 0.000583 |
| bench_native_median      | 0.000179 | 0.000167 | 0.000127 | 0.000118 | 0.000188 |
| bench_ruby_stdev         | 0.000780 | 0.000767 | 0.000730 | 0.000718 | 0.000733 |
| bench_native_stdev       | 0.000046 | 0.000027 | 0.000034 | 0.000026 | 0.000043 |
| bench_ruby_mean          | 0.000261 | 0.000240 | 0.000242 | 0.000250 | 0.000241 |
| bench_native_mean        | 0.000033 | 0.000023 | 0.000022 | 0.000028 | 0.000022 |

## Found a bug? Need a function?

If you found a bug or need a particular function, please let me know! I work on this gem in my spare time, mainly for learning purposes. Feel free to open a PR or a Github issue and I'll take a look as soon as possible.

## Usage

    require 'ruby_native_statistics'
    r = [1,3,21,32,42]

    # calculate sample standard deviation, you can also use "stdevs"
    p r.stdev

    # calculate population standard deviation
    p r.stdevp

    # calculate sample variance
    p r.var

    # calculate population variance
    p r.varp

    # calculate mean
    p r.mean

    # calculate median
    p r.median

    # calculate percentile
    p r.percentile(0.3333)

## Implementation notes

### Percentile

Percentile uses the same rounding method as Excel, sometimes called R7.
