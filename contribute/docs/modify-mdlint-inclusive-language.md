# Modifying non-inclusive words in `mdlint`

Fuchsia uses `mdlint`, a Markdown linter to enforce specific rules
for documentation in the Fuchsia Source Tree. `mdlint` has a non-inclusive
language detector that uses a JSON file of non-inclusive words and their
respective alternatives.

To update, remove, or add to the non-inclusive language list:

{% dynamic if user.is_googler %}

Note: You may need to modify another file to update the linting tools used for Fuchsia.
See [go/fuchsia-non-inclusive-list][non-inclusive-list].

{% dynamic endif %}

## Update the list

1. Open [`/tools/mdlint/rules/respectful_code_words.json`][json-url].
1. To add or modify suggestions for a non-inclusive word, append a comma and
   write your new suggestion.  For example:

   ```none
   "master": "main, primary,{{ '<var>' }}new suggestion{{ '</var>' }}",
   ```

## Remove from list

1. Open [`/tools/mdlint/rules/respectful_code_words.json`][json-url].
1. Delete the entire line containing the non-inclusive word and suggestions.

    Note: If you delete the last word in the list, remove the comma from the new final word line.

## Add to list

1. Open [`/tools/mdlint/rules/respectful_code_words.json`][json-url].
1. Add a newline between the words based upon the alphabetical order of the words in the list.
1. Add your word with its respective inclusive suggestions. For example:

    Note: If your new word is at placed at the end of the JSON, do not end the line with a comma (`,`).

   ```none
   "{{ '<var>' }}non-inclusive word{{ '</var>' }}": "{{ '<var>' }}first inclusive suggestion, second inclusive suggestion...{{ '</var>' }}",
   ```

[json-url]: https://ci.android.com/edit?repo=fuchsia/fuchsia/main&file=tools/mdlint/rules/respectful_code_words.json
[non-inclusive-list]: http://go/fuchsia-non-inclusive-list