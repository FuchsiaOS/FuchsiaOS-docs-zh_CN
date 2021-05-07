# Lookup API

Two main questions arise from the discussion around [message
translation](message_translation.md):

1.  What does the _concrete_ interface to the `[Lookup]` service look like in
    the programmer's language of choice? And,
1.  How do translations make their way to my program so that they are available
    to use?

We will answer these in turn.

## Lookup API

The Lookup API library is used to obtain translated strings.  A simplified view
of the  Lookup API in C++ is as follows:

```cpp
class Lookup {
public:
  enum class Status {
    // No error.
    OK = 0,
    // The resource was unavailable as requested.
    UNAVAILABLE = 1,
  };
  static fit::result<std::unique_ptr<Lookup>, Lookup::Status>
    New(const std::vector<std::string>& locale_ids);
  fit::result<std::string_view, Lookup::Status> String(uint64_t message_id);
};
```

> For the time being, the Lookup API is a C++ API only.  There are no conceptual
> issues with extending it to support other languages.

The actual API can be seen in the file
<code>[lookup.h](/src/lib/intl/lookup/cpp/lookup.h)</code>, and is essentially
the same as shown above, except that it contains documentation, construction
and testing overhead.  At the time of this writing, only a high level C++ API
is available for use.  We will be adding high level APIs in other languages as
need arises.  A [low-level C API](/src/lib/intl/lookup/rust/lookup.h) is
available as a basis for writing [FFI
bindings](https://en.wikipedia.org/wiki/Foreign_function_interface) to this
functionality in other languages.  As a special case,
[rust](https://www.rust-lang.org/) does not need the FFI bindings since the
low-level implementation is in rust and can be interfaced with directly; but an
actual rust API has not been formulated yet.

A basic usage of the Lookup API looks like this:

```cpp
std::vector<std::string> locale_ids = {"nl-NL"};
auto result = Lookup::New(locale_ids);
if (result.is_error()) {
  return;
}
auto lookup = result.value();
auto lookup_result = lookup.string(42);
if (lookup_result.is_error()) {
  // handle error
  return;
}
std::string_view message = lookup_result.value();
// Use `message`.
```

The example is taken from the [`lookup.h`
documentation](/src/lib/intl/lookup/cpp/lookup.h#10). Knowing the API, this
example is fairly straightforward, save for one thing: the call
<code>lookup.string(...)</code> uses a magic number <code>42</code> to look up
a message.  It is fair of you as a programmer to ask where this number comes
from.  The next section addresses this question.
