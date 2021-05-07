## Message Translation

The first functionality that Fuchsia's localization provides is message
translation.  Conceptually this gives a program the ability to display
user-facing messages in the user's language of choice.  This is achieved using
locale-sensitive formatter printing.  The program needs to keep the
"localization context" where user's localization preferences are stored.  When,
in the course of program execution, there comes a point that a message "Hello
world!" needs to be displayed on screen in the user's native language (for
example Spanish would be encoded as "es", for European Spanish, and "es-419"
for Spanish as used in the Americas), the program can look up a translation by
providing an abstract `[Lookup]` service with the original message and the
desired translation.  Conceptually this will amount to a line in the code that
matches this general pattern:

```
[Lookup].String({locale-ids=["es-419"]}, MSG_Hello_World) ⇒ (yields the translation)
```

In the above example, `[Lookup]` can be any sort of callable endpoint: it could
be a library exposed function, or could be an interface point for an
[RPC](https://en.wikipedia.org/wiki/Remote_procedure_call) stub that fetches
the translation over the network.  The abstract operation of "fetching a
translated message" is here called `String` to distinguish it from other
possible calls to typed data, such as `StringArray` or others.

Note, messages can get quite a bit more elaborate than that, which is why we
typically don't want the program authors to handle them directly, but rather
through message IDs.

Two more things are of note:

1.  The language identifiers are specified as [Unicode locale
	IDs](http://unicode.org/reports/tr35/#Unicode_locale_identifier) (hence the
	named parameter `locale-ids` in the example), and multiple such locale IDs
	can be provided at once.  This is because users may have more than a single
	preferred language, and may have a hierarchy of languages by preference.
	This allows the localization system to choose the best available message,
	possibly in more than a single language in a single session.
1.  The messages are not specified as their string representation in code.
	Rather, they are referred to by a unique message identifier. In the example
	above, this was arbitrarily named `MSG_Hello_World`.  While schools of
	thought differ on whether strings should be internalized or externalized,
	we opted for the latter.  Our main reasons were to keep the source code
	free from linguistic concerns, which makes the translation toolchain
	somewhat easier to maintain, and makes translations at scale easier to
	manage.

Two main questions arise from the example above:

1.  What does the _concrete_ interface to the `[Lookup]` service look like in
	the programmer's language of choice? And,
1.  How do translations make their way to my program so that they are available
	to use?

We will answer these in turn when we talk about the [Lookup API](lookup_api.md).
