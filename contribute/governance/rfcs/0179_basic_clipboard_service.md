<!-- mdformat off(templates not supported) -->
{% set rfcid = "RFC-0179" %}
{% include "docs/contribute/governance/rfcs/_common/_rfc_header.md" %}
# {{ rfc.name }}: {{ rfc.title }}
{# Fuchsia RFCs use templates to display various fields from _rfcs.yaml. View the #}
{# fully rendered RFCs at https://fuchsia.dev/fuchsia-src/contribute/governance/rfcs #}
<!-- SET the `rfcid` VAR ABOVE. DO NOT EDIT ANYTHING ELSE ABOVE THIS LINE. -->

<!-- mdformat on -->

<!-- This should begin with an H2 element (for example, ## Summary).-->

## Summary

This RFC introduces two new framework-provided protocols,
`fuchsia.ui.clipboard.Writer` and `fuchsia.ui.clipboard.Reader`, and the service
that will implement them, which will allow users to perform copy and paste
operations on text content.

## Motivation

Many modern user-facing operating systems with graphical shells provide
clipboard functionality (see [Prior art](#prior-art-and-references)), allowing
users to interactively copy data onto a system-provided memory buffer or other
channel, and to later paste that data into another location.

In the past, Fuchsia had a [rudimentary clipboard protocol](#fuchsia)
implemented as a Modular agent, but this code was removed in 2019.

In this RFC, we propose introducing a new clipboard protocol and implementation
with which Fuchsia products will have the option to integrate. The most pressing
need is the ability to copy and paste Unicode text, so this will be the focus of
the first iteration.

The clipboard facilities of many existing operating systems were
[initially designed without security provisions](#prior-art-and-references),
allowing any process to observe and/or modify the clipboard at any time without
user awareness or intent. On Fuchsia, we aim to design with security in mind by:

*   guarding clipboard access behind granular capabilities, following the
    principle of least privilege
*   attempting to restrict clipboard access contingent on having input focus in
    the foreground window ("View" in Fuchsia's Scenic terminology)
*   offering background clipboard access only as a last resort

## Stakeholders

### Facilitator

davemoore@google.com

### Reviewers

*Fuchsia HCI:* neelsa@google.com, quiche@google.com

*Security:* palmer@google.com

*Privacy:* enoharemaien@google.com

*Chromium:* wez@google.com

*Flutter:* jmccandless@google.com

### Consulted

azaslavsky, carolineliu@google.com, chaopeng@google.com, cpu@google.com,
ddorwin@google.com, fmil@google.com, jsankey@google.com, tjdetwiler@google.com

### Socialization

*   Discussed in Fuchsia Input Team doc review
*   Discussed in Fuchsia Security Office Hours

## Design

### Access levels

Scopes of access to a clipboard in graphical shell environment can be
categorized into three levels:

1.  **Shell-mediated, focus-dependent** \
    A component gets access to the clipboard only in response to an explicit
    user action as determined by the graphical shell, and only if the component
    currently has input focus.
2.  **Focus-dependent** \
    A component can access the clipboard at any time while it has input focus.
3.  **Unrestricted** \
    A component can access the clipboard at any time.

In this RFC, we only cover the *(2) focus-dependent* scope.

The design and implementation of scopes (1) and (3) are not planned at this
time; they will require another RFC.

### Use cases

For the initial RFC, we consider a few simple but common use cases:

*   In the web browser, copying a URL from a web page's body into the address
    bar
*   Copying a shell command from the web browser into the terminal
*   Copying information from the web browser to the workstation product's
    feedback dialog (which is implemented in Flutter)

### Protocol and service

We introduce two new discoverable FIDL protocols,
`fuchsia.ui.clipboard.FocusedReaderRegistry` and
`fuchsia.ui.clipboard.FocusedWriterRegistry`, in the partner SDK. These
protocols will be implemented and exposed by a new component, `clipboard.cm`,
running in the session realm. The component will be included in the workstation
product, and can be used in any other Fuchsia products that require it.

Client components that are granted the `FocusedWriterRegistry` and
`FocusedReaderRegistry` capabilities will be able to request instances of
`fuchsia.ui.clipboard.Writer` and `fuchsia.ui.clipboard.Reader`, respectively.
They MAY request these connections at any time (assuming they possess valid
`ViewRef`s), but the `Writer` and `Reader`'s methods will return an error if the
client's view does not have input focus.

```fidl
library fuchsia.ui.clipboard;

/// A protocol that allows graphical clients that own
/// [`ViewRef`s](/src/development/graphics/scenic/concepts/view_ref) to request read ("paste")
/// access to the clipboard. Clients can register for access at any time, but `GetItem` calls will
/// only succeed while the view has input focus.
@discoverable
protocol FocusedReaderRegistry {
    /// If the `ViewRef` is valid, the clipboard server will allow the client to send commands using
    /// the given `Reader`. If the `ViewRef` later becomes invalid, the `Reader`'s channel will be
    /// closed.
    RequestReader(resource table {
        1: view_ref fuchsia.ui.views.ViewRef;
        2: reader_request server_end:Reader;
    }) -> (table {}) error ClipboardError;
};

/// A protocol that allows graphical clients that own `ViewRef`s to request write ("copy") access to
/// the clipboard. Clients can register for access at any time, but `SetItem` calls will only
/// succeed while the view has input focus.
@discoverable
protocol FocusedWriterRegistry {
    /// If the `ViewRef` is valid, the clipboard server will allow the client to send commands using
    /// the given `Writer`. If the `ViewRef` later becomes invalid, the `Writer`'s channel will be
    /// closed.
    RequestWriter(resource table {
        1: view_ref fuchsia.ui.views.ViewRef;
        2: writer_request server_end:Writer;
    }) -> (table {}) error ClipboardError;
};

/// Allows data to be read from the clipboard, i.e. pasted.
protocol Reader {
    /// Reads a single item from the clipboard. If the client's `View` does not have input focus, an
    /// error will be returned. If there is no item on the clipboard, `ClipboardError.EMPTY` will
    /// be returned.
    GetItem(table {}) -> (ClipboardItem) error ClipboardError;
};

/// Allows data to be written to the clipboard, i.e. copied.
protocol Writer {
    /// Writes a single item to the clipboard. If the client's `View` does not have input focus, an
    /// error will be returned.
    SetItem(ClipboardItem) -> (table {}) error ClipboardError;

    /// Clears the contents of the clipboard. If the client's `View` does not have input focus, an
    /// error will be returned.
    Clear(table {}) -> (table {}) error ClipboardError;
};

/// Set of errors that can be returned by the clipboard server.
type ClipboardError = flexible enum {
    /// An internal error occurred. All the client can do is try again later.
    INTERNAL = 1;

    /// The clipboard was empty, or the requested item(s) were not present on the clipboard.
    EMPTY = 2;

    /// The client sent an invalid request, e.g. missing requiring fields.
    INVALID_REQUEST = 3;

    /// The client sent the server an invalid `ViewRef` or a `ViewRef` that is already associated
    /// with another client.
    INVALID_VIEW_REF = 4;

    /// The client attempted to perform an operation that requires input focus, at a moment when
    /// it did not have input focus. The client should wait until it has focus again before
    /// retrying.
    UNAUTHORIZED = 5;
};
```

In the initial version, the clipboard will only support copying and pasting
UTF-8 strings up to 32 KB in size. Clients MAY specify a
[MIME type](https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types)
for the data; the default shall be `"text/plain;charset=UTF-8"`.

[Subsequent revisions](#future-work) will add support for VMOs, enabling
arbitrary data to be copied and pasted.

```fidl
/// The maximum length of a plain-text clipboard item in bytes. Although FIDL messages support
/// larger messages, this limit allows space to be reserved for potential other fields in the
/// message. Larger payloads will be supported by VMOs in `ClipboardItemData` in future revisions.
const MAX_TEXT_LENGTH uint32 = 32768;

/// The maximum length of a MIME Type identifier. Per
/// [IETF RFC 4288](https://datatracker.ietf.org/doc/html/rfc4288#section-4.2), a MIME type may have
/// up to 127 characters before and 127 characters after the slash, for a total of 255.
const MAX_MIME_TYPE_LENGTH uint32 = 255;

/// A single item on the clipboard, consisting of a MIME type hint and a payload.
type ClipboardItem = resource table {
    /// MIME type of the data, according to the client that placed the data on the clipboard.
    /// *Note:* The clipboard service does not validate clipboard items and does not guarantee that
    /// they conform to the given MIME type's specifications.
    1: mime_type_hint string:MAX_MIME_TYPE_LENGTH;
    /// The payload of the clipboard item.
    2: payload ClipboardItemData;
};

/// The payload of a `ClipboardItem`. Future expansions will support additional transport formats.
type ClipboardItemData = flexible resource union {
    /// A UTF-8 string.
    1: text string:MAX_TEXT_LENGTH;
};
```

## Implementation

This will happen in several stages:

1.  Submit the new `fuchsia.ui.clipboard` FIDL library, as previewed above, for
    API review.
2.  Implement a new clipboard server component running in the session realm that
    exposes the `fuchsia.ui.clipboard.FocusedWriterRegistry` and
    `fuchsia.ui.clipboard.FocusedReaderRegistry` protocols.
3.  Demonstrate integration with the new protocols with a simple component that
    manages a Scenic view.
4.  Integrate support for the new protocols into the Chromium and Flutter
    runners.

## Performance

Adding the new service will use additional storage space for the binary, as well
memory for the binary and clipboard contents. Each client that registers for
clipboard access will consume resources by keeping an open Zircon channel.

## Security considerations

A [security review](#security-review-outcome) is required.

### Cross-component communication

The introduction of a clipboard service constitutes a new cross-component
communication channel. This opens new possibilities for components to
intentionally or unintentionally exploit each other's vulnerabilities.

### Untrusted content

The clipboard service makes no guarantees about the trustworthiness of
`ClipboardItem` data or MIME type hints. Clients therefore SHOULD NOT trust what
they receive, and SHOULD validate that the data is appropriate for their use
case.

In particular, clients SHOULD perform any parsing, interpreting, or transforming
of complex formats in a lower-privilege "sandboxed" process, and in a
programming language safer than C/C++. (See the
[Rule of 2](https://chromium.googlesource.com/chromium/src/+/master/docs/security/rule-of-2.md)
for more.)

For the `ClipboardItemData.text` variant, UTF-8 validation is automatically
performed by the FIDL library used in every client (and in the clipboard
service).

However, even with plain, valid UTF-8 text, the ability for one component to
send arbitrary text to another over the clipboard can open the door to a variety
of exploit vectors, including:

*   overflow bugs in text widgets
*   bugs in text rendering stacks
*   [homograph attacks](https://en.wikipedia.org/wiki/Homoglyph#Unicode_homoglyphs)
    (deceiving users with visually similar glyphs that are actually different
    characters, e.g. to surreptitiously send users to a phishing domain)
*   bugs in application-specific text parsing
*   unexpected code pasted into command prompts

Most of these issues are already a concern for applications that process or
display any third-party or user-provided content, but clipboards pose an
additional challenge. A malicious application can, in response to a valid user
copy command, place unexpected data (that is not visibly selected) onto the
clipboard, tricking the user into acting as a
[confused deputy](https://en.wikipedia.org/wiki/Confused_deputy_problem) when
pasting it.

### Unauthorized access

As alluded to [above](#access-levels), a component may want to read from or
write to the clipboard without authorization, or without user knowledge.

This is mitigated by:

*   Protocols that can be used to grant or deny access to copy and paste
    capabilities granularly
*   In the `fuchsia.ui.clipboard.Focused*` protocols, the requirement that
    clipboard access be granted only to a foreground view that has input focus

In [future expansions](#future-work) of the clipboard API, we may provide a
protocol for observing clipboard API events that a system shell could use to
show visual notifications whenever the clipboard is accessed.

In the future, with the addition of untrusted components and new clipboard use
cases, we will have to reconsider whether unauthorized read attempts should
silently return empty clipboard items instead of `ClipboardError.UNAUTHORIZED`,
to reduce the information revealed about clipboard access.

### `ViewRef` and focus validation

The clipboard service relies on Scenic's
[focus chain](/docs/development/graphics/scenic/concepts/focus_chain.md) system
to determine which view is currently focused and therefore has the right to
access the clipboard. Therefore, the clipboard's determination of clipboard
focus is *only as reliable as Scenic's determination of input focus*, which has
some flaws:

*   [`ViewRef`s](/docs/development/graphics/scenic/concepts/view_ref.md), which
    make up a focus chain, can easily be cloned and sent from one component to
    another. By this mechanism, malicious components can collaborate to
    impersonate each other for the purposes of input focus. (Though this
    requires that at a minimum, the original owner of a `ViewRef` trust the
    recipient of the cloned `ViewRef`.)
*   Focus changes are subject to
    [race conditions](/docs/development/graphics/scenic/concepts/focus_chain.md#race_between_view_focus_event_and_focus_chain).

### Security review outcome

*   This MVP API is limited, which limits avenues for attack.
*   We rely on the underlying FIDL deserialization to correctly validate UTF-8
    strings when they are received by the clipboard service. This is an attack
    surface, but we believe it is safe because the service's string
    deserialization relies on Rust's `std::str::String` implementation, which is
    memory-safe and heavily tested both within Fuchsia and in the broader Rust
    ecosystem.
*   We believe the `ViewRef` solution is a good foundation for tracking view
    focus and user intent, with the
    [caveats above](#viewref-and-focus-validation).

## Privacy considerations

A privacy review is required.

### Unauthorized pasting

There is a privacy risk in unauthorized access to the contents of a clipboard,
which would allow malicious components to obtain any private data the user has
placed on the keyboard. As described [above](#unauthorized-access), the
`fuchsia.ui.clipboard.Focused*` protocols mitigate this risk by requiring views
to at least have input focus (and hence be visible in the foreground) in order
to access the clipboard. However, this does mean that an application can grab
the contents of the clipboard as soon as it obtains focus momentarily, even if
this is not the the user's intent. In the [future](#future-work), system shell
notifications will alert the user whenever a component reads the contents of the
clipboard.

### Side channel attacks

In future iterations of the clipboard service, after the addition of arbitrary
data types and lengths, there may be a risk that memory analysis may reveal
information (e.g. size of the clipboard buffer) that could hint at its contents.

### Persistence of clipboard contents

At this stage, with support only for copying short strings, clipboard contents
will be stored in memory, not on disk.

Clipboard contents will not be logged or exposed via
[Inspect](/docs/development/diagnostics/inspect).

### Access across security contexts

Fuchsia products may have UI elements running in different security contexts,
e.g. on behalf of different users, or in a pre-authentication context. Fuchsia
MUST prevent clipboard content from being shared across security context
boundaries. For example, if a logged in user copies their password and later
locks the screen, it must not be possible to paste that password into the lock
screen dialog.

This separation MAY be achieved by running separate instances of the clipboard
service in each security context.

## Testing

This feature will be tested with unit and integration tests:

-   Unit tests within the clipboard service
-   Integration tests of the clipboard service as a whole
-   Integration tests of the interactions among the clipboard service, Scenic,
    the Input Pipeline, and the Flutter or Chromium runners.

## Documentation

The `fuchsia.ui.clipboard` APIs will be documented with fidldoc.

Use of the protocol will be illustrated with a well-commented, simple component
that manages a Scenic view (see [Implementation](#implementation)).

## Drawbacks, alternatives, and unknowns

There is no viable alternative to offering a system-wide clipboard service on a
user-facing operating system. This functionality could not be offered by runners
alone, as they would not have the ability to copy and paste data across
different runtimes.

Within the design choices described, an alternative approach might have been to
start with a more restrictive "shell-mediated, focus-dependent" clipboard
protocol, or the completely unrestricted protocol (see
[Access levels](#access-levels)).

The shell-mediated approach, while more secure, may be too restrictive to be
practical for many use cases. For example, it would

*   prevent applications from offering copy and paste commands in context menus
*   interfere with the functionality of web clipboard APIs in Chromium-based
    runners

The unrestricted approach, while useful for some niche applications, would pose
too much of a privacy risk to be offered as a default.

By starting with a middle-ground access level based on input focus, we will be
able to:

*   prioritize some security and privacy guarantees in the clipboard service
    from the start
*   encourage the principle of least privilege in the runners' integrations of
    the Fuchsia clipboard
*   avoid restrictions that are too onerous for practical integration with
    existing runners

## Future work

*   Provide a protocol for observing clipboard API events (reads and writes)
    that a system shell could use to show visual notifications when the
    clipboard is accessed.

*   Expand the set of supported data formats and payload sizes, particularly
    through the use of VMOs owned by the sending client.

## Prior art and references

### Fuchsia

Fuchsia previously had a
[minimal clipboard API](https://fuchsia.googlesource.com/fuchsia/+/02930bf563b779128de0a25695432be8a6d0e5c2/sdk/fidl/fuchsia.modular/clipboard/clipboard.fidl),
implemented as a Modular Framework agent, that allowed any component with the
`fuchsia.modular.Clipboard` capability to store or retrieve a UTF-8 string.
(This functionality was purged in Nov 2019.)

### Linux: X11

X11 offers multiple content storage areas called "selections," of which the most
common are `CLIPBOARD` and `PRIMARY` (the implicit text selection clipboard).

The sending application announces to the X server that it "owns" one of these
selections, in a given data format, in a particular window
(`XSetSelectionOwner()`). Then it waits for further events.

The receiving application, in one of its windows, requests a selection to be
converted into a particular format that it supports (`XConvertSelection()`).

The X server forwards the request to the sending application, which, if it
supports the requested format, responds by sending data through the X server to
the receiving application. If the content is large, it must be chunked into
segments of up to 256 KB.

If the originating window is destroyed, the selection is lost, so in practice
(1) most applications keep their selections in an invisible window that the user
won't close and (2) common Linux distributions include a clipboard manager that
takes ownership of the selection to keep it alive even if the original owning
application exits.

For more details, see
https://www.uninformativ.de/blog/postings/2017-04-02/0/POSTING-en.html.

### Linux: Wayland

The sending application, which must be focused, notifies the compositor that it
has a `wl_data_source`, indicates which MIME types that data source supports,
and registers an event listener. It then waits for a `send` event.

The receiving application, which must in turn be focused when attempting to
paste, listens for data `offer` events to determine if the clipboard has been
populated. When it wishes to paste, it calls `wl_data_offer_receive`, passing in
the requested MIME type and a file descriptor (usually the write end of a pipe).

The sending application receives a `send` event and writes to the given file
descriptor; the receiving application reads the other end.

For more details, see
https://emersion.fr/blog/2020/wayland-clipboard-drag-and-drop/.

### Windows (win32)

The system clipboard is obtained by calling `OpenClipboard()` and passing in the
current window's handle. The sending application clears any existing data by
calling `EmptyClipboard()` and then calls `SetClipboardData()`, passing in an
integer data type ID and the data itself. The memory for the data being sent
needs to have been allocated using `GlobalAlloc()`.

There are several standard clipboard data types; alternatively, it is possible
to call `RegisterClipboardFormat()` for custom global formats (apparently
persistent until reboot), or to use an ID within a specific range to indicate a
private clipboard format. For non-private formats, the OS takes ownership of the
object that is passed in and becomes responsible for its eventual destruction;
for private formats, the originating window remains responsible for cleanup when
the clipboard is destroyed. For lazy format conversions, the originating window
can pass a `NULL` data value to `SetClipboardData()`, and later, in response to
`WM_RENDERFORMAT`, render the requested format and replace the placeholder with
another call to `SetClipboardData()`. Developers are encouraged to set clipboard
data in as many formats as possible.

The receiving application also retrieves a handle to the global clipboard for
its window, checks the list of available formats (including formats explicitly
placed by the sending application as well those offered for automatic conversion
by the OS), calls `GetClipboardData()` to obtain a handle to a clipboard object
for a particular format, and then `GlobalLock()` to lock that global resource
and get access to its contents.

Methods are also provided for windows to register to monitor changes to the
contents of the clipboard.

For more details, see
https://docs.microsoft.com/en-us/windows/win32/dataxchg/using-the-clipboard and
https://docs.microsoft.com/en-us/windows/win32/dataxchg/clipboard-operations.

### Android

The sending application creates a `ClipData` object with a list of supported
MIME types and populates the `ClipData` with one or more items, which can be a
string, a content URI pointing to any data, or an Intent (for application
shortcuts). The sending application then obtains a reference to the global
`ClipboardManager` object and passes the `ClipData` object into
`setPrimaryClip()`.

If copying a content URI, the sending application must export a
`ContentProvider` that can serve data for that URI.

The receiving application obtains a reference to the global `ClipboardManager`,
checks if it has a primary clip, and then checks if it supports the data type of
any of the `ClipData.Item`s. If pasting a plain string, the receiving
application can simply call `getText()`. If pasting from a content URI, the
receiving application must create a `ContentResolver` instance, `query()` it
with the given URI, and then retrieve data from the returned `Cursor`.

As of Android 12, the OS shows a toast message when one application accesses
`ClipData` that was sent by another application.

For more details, see
https://developer.android.com/guide/topics/text/copy-paste.

### MacOS

The system-wide "pasteboard" is accessed through the `NSPasteboard.general`
field.

The sending application copies items by passing into the method `writeObjects()`
an array of objects that implement the `NSPasteboardWriting` protocol.
Implementers include strings and other common data types, as well
`NSPasteboardItem`, which serves a wrapper for custom data types.
`NSPasteboardWriting` provides a list of supported Uniform Type Identifiers
(UTI, Apple's equivalent of MIME type), as well as whether the data is available
immediately or "promised". Correspondingly, `NSPasteboardItem` can directly wrap
data or a data provider.

On the receiving end, an application can query the general `NSPasteboard` for
types that it can read, including types that can automatically be converted by
filter services. It can then choose to read all or some selection of the items
stored on the pasteboard.

For more details, see
[​​https://developer.apple.com/documentation/appkit/nspasteboard](https://developer.apple.com/documentation/appkit/nspasteboard).

### iOS

The iOS clipboard API is similar to that of MacOS. The system-wide pasteboard is
accessed through `UIPasteboard.general`.

For sending, there are a variety of methods for adding one or more items,
labeled with a UTI type, to the pasteboard. It is also possible to insert
`NSItemProviders` that will supply values lazily. For convenience, several
standard data types are given their own readable/writable array properties on
`UIPasteboard` instances: `strings`, `images`, `urls`, and `colors` -- as well
as singular versions of each of these, for accessing just the first *item* of
each type.

On the receiving side, one can retrieve any selection of items, by index or by
type.

As of iOS 14, retrieving pasteboard contents that were placed there by another
application triggers a system notification. To reduce spurious notifications
before actually pasting, iOS offers clients the ability to query whether certain
data types are present on the pasteboard (`hasStrings`, `hasImages`) without
accessing the data.

For more details, see
https://developer.apple.com/documentation/uikit/uipasteboard.

### Web API

Although clipboard interactions on web pages are primarily handled by the web
browser itself (subject to per-OS peculiarities), there are also JavaScript APIs
available that allow web pages to interact with the clipboard without
necessarily relying on direct user commands.

The older `ClipboardEvent` API allowed scripts to listen for `"cut"`, `"copy"`,
or `"paste"` events on a DOM `Element`, and then access the event's
`clipboardData` field, which allowed calling `setData` or `getData` by MIME
type. It was also possible to programmatically invoke `"cut"`, `"copy"`, or
`"paste"` on the currently focused element. Out of privacy considerations,
programmatic pasting is no longer possible, while on `"cut"` and `"copy"`
events, the clipboard contents cannot be read.

A new, asynchronous `Clipboard` API is now available, guarded by per-site user
permissions. If the user grants permission, a script can access
`navigator.clipboard`, and then `writeText()` or `readText()`, or `write()`
`ClipboardItem`s containing one or more blobs keyed by MIME type. (Non-image
MIME types are still experimental in some browsers.)

For more details, see https://whatwebcando.today/clipboard.html and
https://developer.mozilla.org/en-US/docs/Web/API/Clipboard.

### ChromeOS

Chrome extensions can use the Clipboard APIs described above, subject to
permissions.
