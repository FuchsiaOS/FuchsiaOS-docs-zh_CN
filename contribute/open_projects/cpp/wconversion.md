# C++ implicit conversions

## Goal & motivation

C/C++ allows converting values between types in ways that may modify the
underlying value, for instance due to overflows, underflows, sign changes,
truncation, and loss of precision.

These conversions may happen implicitly, in which case it's not indicated that
the author expects a conversion to take place, or explicitly, in which case it's
indicated that the conversion is intended and the author recognizes the
consequences.

By default, Fuchsia C/C++ code compiles with the flag
[`-Wconversion`][wconversion], which prohibits implicit type conversions that
may alter a value. When this new default behavior was rolled out in 2020, source
code that had pre-existing errors was left unchanged and associated build
definitions received a suppression. We would like to clean up this legacy, since
suppressing this warning can hide real and very subtle bugs in code.

## Technical background

Examples for implicit type conversions that may alter a value:

```cpp
float f = sqrt(2.0);
int   si = 0.5;
unsigned int ui = -1;
```

These will generate compiler errors, unless the warning is suppressed.

Examples for implicit type conversions that may not alter a value:

```cpp
double d = sqrtf(2.0f);
size_t size = 20;
int i = 4.0;
```

These are determined to be safe by the compiler, and will never emit a warning
for implicit type conversion.

A `BUILD.gn` definition for compiling code with implicit type conversion
warnings suppressed may look as follows:

```gn
source_set("foo") {
  ...
  # TODO(fxbug.dev/58162): delete the below and fix compiler warnings
  configs += [ "//build/config:Wno-conversion" ]
}
```

In [`//build/config/BUILD.gn`](/build/config/BUILD.gn) under the `visibility`
list of the `"Wno-conversion"` config target you will find an allowlist of all
instances where this suppression is present in the source tree.

## How to help

### Picking a task

Pick any instance where `Wno-conversion` or [bug 58162][fxb58162] are
referenced. Optionally browse
[`//build/config/BUILD.gn`](/build/config/BUILD.gn) for references to any code
that you're familiar with.

You can ignore third party code (`third_party` in the directory path).
The Fuchsia project doesn't own this code, so you can't fix it.
That said if you'd like you could go to the upstream code location and contribute
a fix there. That said, it's important for the upstream maintainers to also agree
to enable the `-Wconversion` flag, otherwise they'll regress.

### Doing a task

Remove the `-Wno-conversion` suppression, rebuild, and fix any errors.

Note that it's possible that no errors will be surfaced. This often happens when
someone else has made changes to source code that used to have implicit type
conversion issues, but did not remove the suppression. If there is nothing to
fix, proceed to sending the change for review. Otherwise, keep reading.

It's entirely possible for large swaths of code to have been needlessly
suppressed. If you removed a bunch of suppressions and your change passes CQ
then you're not wrong, just lucky.

Example:

*   [478402: [cleanup] Remove unused -Wno-conversions](https://fuchsia-review.googlesource.com/c/fuchsia/+/478402)
*   [474400: [bt][common] Remove Wno-conversion suppression](https://fuchsia-review.googlesource.com/c/fuchsia/+/474400)
*   [487186: [connectivity] Clean up Wno-conversion](https://fuchsia-review.googlesource.com/c/fuchsia/+/487186)
*   [484416: [storage] Drop Wno-conversion](https://fuchsia-review.googlesource.com/c/fuchsia/+/484416)

#### Simple downcasts

```
[5838/95510] CC
obj/src/graphics/lib/compute/spinel/ext/geometry/geometry.svg_arc.c.o
../../src/graphics/lib/compute/spinel/ext/geometry/svg_arc.c:96:57: error:
implicit conversion loses floating-point precision: 'double' to 'float'
[-Werror,-Wimplicit-float-conversion]
  arc_params->phi = fmodf(x_axis_rotation_radians, M_PI * 2.0);
                      ~~~~~                          ~~~~~^~~~~
```

In this case, you'll just need to rewrite the offending line of code to make the
conversion explicit.

```cpp
arc_params->phi = fmodf(x_axis_rotation_radians, (float)(M_PI * 2.0));
```

This is fine because the loss of precision from double to float isn't a concern
for this code. Other changes may be more complex. For instance you may want to
add a bounds check before doing a conversion.

#### Changing a variable type

There are plenty of instances where it will be more appropriate to change the
source variable type than to apply an explicit cast.

```cpp
../../garnet/bin/hwstress/memory_stress.cc:216:90: error: implicit conversion changes signedness: 'int' to 'uint64_t' (aka 'unsigned long') [-Werror,-Wsign-conversion]
                            MultiWordPattern(NegateWords((RotatePattern(every_sixth_bit, i))))));
                                                          ~~~~~~~~~~~~~                  ^
```

If we look at more of the code:

```cpp
std::vector<uint64_t> RotatePattern(std::vector<uint64_t> v, uint64_t n);
  ...
  for (int i = 0; i < 6; i++) {
    result.push_back(MakePatternWorkload(
        fxl::StringPrintf("Single bit clear 6-bit (%d/6)", i + 1),
        MultiWordPattern(NegateWords((RotatePattern(every_sixth_bit, i))))));
  }
```

We see that i is only used in this loop. It would be simpler to make it an
unsigned type:

```cpp
  for (uint32_t i = 0; i < 6; i++) {
    result.push_back(MakePatternWorkload(
        fxl::StringPrintf("Single bit clear 6-bit (%u/6)", i + 1),
        MultiWordPattern(NegateWords((RotatePattern(every_sixth_bit, i))))));
  }
```

#### Multiple casts in a ternary expression

If the compiler warns on operands in a ternary expression, you technically can
add an individual cast for each operand to the result type, but it would
probably be cleaner to wrap the entire expression in a cast. For example:

```
[5836/95510] CC obj/src/graphics/lib/compute/spinel/ext/geometry/geometry.arc.c.o
../../src/graphics/lib/compute/spinel/ext/geometry/arc.c:73:50: error: implicit conversion loses floating-point precision: 'double' to 'float' [-Werror,-Wimplicit-float-conversion]
  float const theta_sweep = theta_delta > 0.0f ? SPN_SWEEP_RADIANS : -SPN_SWEEP_RADIANS;
              ~~~~~~~~~~~                        ^~~~~~~~~~~~~~~~~
../../src/graphics/lib/compute/spinel/ext/geometry/arc.c:28:39: note: expanded from macro 'SPN_SWEEP_RADIANS'
#define SPN_SWEEP_RADIANS (2.0 * M_PI / 3.0)  // 120°
                           ~~~~~~~~~~~^~~~~
../../src/graphics/lib/compute/spinel/ext/geometry/arc.c:73:70: error: implicit conversion loses floating-point precision: 'double' to 'float' [-Werror,-Wimplicit-float-conversion]
  float const theta_sweep = theta_delta > 0.0f ? SPN_SWEEP_RADIANS : -SPN_SWEEP_RADIANS;
              ~~~~~~~~~~~                                            ^~~~~~~~~~~~~~~~~~
```

the error appears for both operands of the conditional expression. Casts could
be applied on the SPN_SWEEP_RADIANSs in the expression, or around the entire
conditional expression.

```cpp
// Explicit casts at the error sites.
float const theta_sweep = theta_delta > 0.0f ? (float)(SPN_SWEEP_RADIANS) : (float)(-SPN_SWEEP_RADIANS);

// Different AST, but effectively cleaner.
float const theta_sweep = (float)(theta_delta > 0.0f ? SPN_SWEEP_RADIANS : -SPN_SWEEP_RADIANS);
```

From an AST perspective, the expressions are different, but the logic and
optimized codegen are still the same.

#### Try not to apply casts in macros, but rather in their invocations

Macros (especially those in headers) tend to be used in multiple places and
accept different types of arguments. It’s usually better to apply casts to macro
arguments or around the macro itself rather than inside the macro. For example:

```cpp
../../src/virtualization/third_party/fdt/fdt.c:143:16: error: implicit conversion changes signedness: 'unsigned long' to 'int' [-Werror,-Wsign-conversion]
        *nextoffset = FDT_TAGALIGN(offset);
                    ~ ^~~~~~~~~~~~~~~~~~~~
../../src/virtualization/third_party/fdt/libfdt_internal.h:56:26: note: expanded from macro 'FDT_TAGALIGN'
#define FDT_TAGALIGN(x) (FDT_ALIGN((x), FDT_TAGSIZE))
                         ^~~~~~~~~~~~~~~~~~~~~~~~~~~
../../src/virtualization/third_party/fdt/libfdt_internal.h:55:40: note: expanded from macro 'FDT_ALIGN'
#define FDT_ALIGN(x, a) (((x) + (a)-1) & ~((a)-1))
                         ~~~~~~~~~~~~~~^~~~~~~~~~
../../src/virtualization/third_party/fdt/fdt.c:143:29: error: implicit conversion changes signedness: 'int' to 'unsigned long' [-Werror,-Wsign-conversion]
        *nextoffset = FDT_TAGALIGN(offset);
                      ~~~~~~~~~~~~~^~~~~~~
../../src/virtualization/third_party/fdt/libfdt_internal.h:56:37: note: expanded from macro 'FDT_TAGALIGN'
#define FDT_TAGALIGN(x) (FDT_ALIGN((x), FDT_TAGSIZE))
                         ~~~~~~~~~~~^~~~~~~~~~~~~~~~
../../src/virtualization/third_party/fdt/libfdt_internal.h:55:28: note: expanded from macro 'FDT_ALIGN'
#define FDT_ALIGN(x, a) (((x) + (a)-1) & ~((a)-1))
                           ^  ~
```

Both errors point to the innermost macro expansion. Rather than applying a cast
to the & in the first error or x in the second error, it would be better to
instead apply the casts to the macro argument offset and the result of
FDT_TAGALIGN in the original statement.

```cpp
        *nextoffset = (int)(FDT_TAGALIGN((unsigned)offset));
```

#### Use templated types

In a similar vein to macros, make sure that when you apply casts in a templated
function that you use templated types, not the type specified in the warning.
Other code may use this template with different types. For example, in:

```cpp
template <typename T>
constexpr typename std::make_signed<T>::type ConditionalNegate(
    T x,
    bool is_negative) {
  static_assert(std::is_integral<T>::value, "Type must be integral");
  using SignedT = typename std::make_signed<T>::type;
  using UnsignedT = typename std::make_unsigned<T>::type;
  return static_cast<SignedT>(
      (static_cast<UnsignedT>(x) ^ -SignedT(is_negative)) + is_negative);
}
```

We get:

```
../../zircon/third_party/ulib/safemath/include/safemath/safe_conversions_impl.h:75:36: error: implicit conversion changes signedness: 'SignedT' (aka 'long') to 'unsigned long' [-Werror,-Wsign-conversion]
      (static_cast<UnsignedT>(x) ^ -SignedT(is_negative)) + is_negative);
                                 ~ ^~~~~~~~~~~~~~~~~~~~~
```

We should static_cast to UnsignedT instead of unsigned long.

```cpp
  return static_cast<SignedT>(
      (static_cast<UnsignedT>(x) ^ static_cast<UnsignedT>(-SignedT(is_negative))) + is_negative);
```

#### Callback Functions

This can be a particularly deceitful error since the call stack may not
necessarily point to the exact problem. Assigning callback functions with
mismatching argument types are an example of this. For example, if we look at:

```
../../sdk/lib/fit/include/lib/fit/function_internal.h:70:19: error: implicit conversion changes signedness: 'long' to 'uint64_t' (aka 'unsigned long') [-Werror,-Wsign-conversion]
    return target(std::forward<Args>(args)...);
           ~~~~~~ ^~~~~~~~~~~~~~~~~~~~~~~~
../../sdk/lib/fit/include/lib/fit/function_internal.h:91:60: note: in instantiation of member function 'fit::internal::target<(lambda at ../../examples/fidl/test/launcher.cc:67:7), true, false, void, long, fuchsia::sys::TerminationReason>::invoke' requested here
    &unshared_target_type_id, &inline_target_get, &target::invoke, &target::move, &target::destroy};
                                                           ^
../../sdk/lib/fit/include/lib/fit/function_internal.h:370:45: note: in instantiation of static data member 'fit::internal::target<(lambda at ../../examples/fidl/test/launcher.cc:67:7), true, false, void, long, fuchsia::sys::TerminationReason>::ops' requested here
      ops_ = &target_type<DecayedCallable>::ops;
                                            ^
../../sdk/lib/fit/include/lib/fit/function_internal.h:297:5: note: in instantiation of function template specialization 'fit::internal::function_base<16, false, void (long, fuchsia::sys::TerminationReason)>::initialize_target<(lambda at ../../examples/fidl/test/launcher.cc:67:7)>' requested here
    initialize_target(std::forward<Callable>(target));
    ^
../../sdk/lib/fit/include/lib/fit/function.h:253:11: note: in instantiation of function template specialization 'fit::internal::function_base<16, false, void (long, fuchsia::sys::TerminationReason)>::assign<(lambda at ../../examples/fidl/test/launcher.cc:67:7), void>' requested here
    base::assign(std::forward<Callable>(target));
          ^
../../examples/fidl/test/launcher.cc:66:43: note: in instantiation of function template specialization 'fit::function_impl<16, false, void (long, fuchsia::sys::TerminationReason)>::operator=<(lambda at ../../examples/fidl/test/launcher.cc:67:7)>' requested here
  client_controller.events().OnTerminated =
                                          ^
```

One might think to look at the code in
sdk/lib/fit/include/lib/fit/function_internal.h:70, but the
underlying issue is in examples/fidl/test/launcher.cc:67

```cpp
  client_controller.events().OnTerminated =
      [&loop, &client_status](uint64_t code, fuchsia::sys::TerminationReason reason) {
...
```

If we look at sdk/fidl/fuchsia.sys/component_controller.fidl, we can see that
the OnTerminated accepts an int64 as the first argument, but we assign a lambda
that accepts a uint64_t as the first argument. The fix here is to accept an
int64_t as the lambda argument instead.

```cpp
  client_controller.events().OnTerminated =
      [&loop, &client_status](int64_t code, fuchsia::sys::TerminationReason reason) {
...
```

#### Lossless downcast

If you want to downcast a variable but you do not want to tolerate loss during
conversion, safemath library provides a set of utility functions, including
checked_cast, that assert when there is loss of data during downcast.

```cpp
void fun(uint64_t block_number) {
  // Downcast block_number because underlying layer expects uint32_t.
  uint32_t block = safemath::checked_cast<uint32_t>(block_number);
  ....
}
```

Above `checked_cast` asserts if block_number is greater than
`std::numberic_limits<uint32_t>::max()`.

To use safemath, add the `//zircon/third_party/ulib/safemath` as dependency in
`BUILD.gn` file. BUILD.gn example can be seen [here](https://fuchsia.googlesource.com/fuchsia/+/be0a0c3f97b29231c9207a934063b3ce9e562dd1/src/storage/minfs/BUILD.gn#97)
and example usage can be seen [here](https://fuchsia.googlesource.com/fuchsia/+/be0a0c3f97b29231c9207a934063b3ce9e562dd1/src/storage/minfs/minfs.cc#1428).

### Completing a task

Tag the cover bug in your change description as follows:

```
Bug: 58162
```

Remove any references to build targets that you cleaned up in
[`//build/config/BUILD.gn`](/build/config/BUILD.gn), under the `visibility` list
for the `"Wno-conversion"` config target.

Find reviewers via owners and merge your change.

Note: we recommend also attempting debug builds on Macs.
If you don't use a Mac, you can add these checks to your change with CQ.
In Gerrit click "choose tryjobs" and select `fuchsia-arm64-debug-mac-build_default`
and `fuchsia-x64-debug-mac-build_default`.
If the tryjobs turn red, you can click on them to find descriptive errors.

## Examples

*   [469454: [debugger] Fix -Wconversion issues](https://fuchsia-review.googlesource.com/c/fuchsia/+/469454)
*   [464514: [fit, fzl] Enable -Wconversion warnings](https://fuchsia-review.googlesource.com/c/fuchsia/+/464514)
*   [463856: [kcounter] Enable -Wconversion warnings](https://fuchsia-review.googlesource.com/c/fuchsia/+/463856)
*   [456573: [feedback] fix -Wconversion errors](https://fuchsia-review.googlesource.com/c/fuchsia/+/456573)
*   [450719: [camera][lib] Fix -Wconversion build errors](https://fuchsia-review.googlesource.com/c/fuchsia/+/450719)

## Sponsors

Reach out for questions or for status updates:

*   <leonardchan@google.com>
*   <phosek@google.com>
*   <mcgrathr@google.com>
*   <shayba@google.com>

[fxb58162]: https://bugs.fuchsia.dev/p/fuchsia/issues/detail?id=58162
[wconversion]: https://gcc.gnu.org/wiki/NewWconversion
