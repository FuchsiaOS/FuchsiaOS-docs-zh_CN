
# Upload changes from multiple repositories

Changes in two or more separate repos will be automatically tracked for you by
Gerrit if you use the same topic.

Multipart changes that are tracked in Gerrit using the same topic will be tested together.
These changes can be landed in Gerrit at the same time with `Submit Whole Topic`. Topics
can be edited using the Gerrit UI on your browser.

## Using jiri upload {#using-jiri-upload}

To upload changes together, you need to create a branch with same name on all repositories.

Do the following:

1.  Make and commit the first change in a Fuchsia repository:

    1.  Go to the repository:

        ```
        cd examples/fortune
        ```
    1.  Create a new branch; for example, *add_my_new_feature*:

        ```
        git checkout -b add_my_new_feature
        ```
    1.  Edit and add the files related to the feature:

        ```
        git add <my_feature_related_files>
        ```
    1.  Commit your first change:

        ```
        git commit
        ```

1.  Make and commit the second change in another Fuchsia repository:

    1.  Go to the second repository:

        ```
        cd fuchsia/build
        ```
    1.  Create a new branch with the same name, *add_my_new_feature*:

        ```
        git checkout -b add_my_new_feature
        ```

    1.  Edit and add the files related to the feature:

        ```
        git add <more_of_my_feature_related_files>
        ```
    1.  Commit your second change:

        ```
        git commit
        ```

1.  Use `-multipart` to upload all changes with the same branch name across repos:

    ```
    jiri upload -multipart
    ```

    Or

    ```
    jiri upload -multipart -topic="custom_topic"
    ```

After the changes are submitted, clean up the local branches:

```
cd examples/fortune
git branch -d add_my_new_feature
```
And

```
cd fuchsia/build
git branch -d add_my_new_feature
```

## Using Git command

You can also use the `git` command to upload all changes across repositories.
The steps are identical as the steps in [Using jiri
upload](#using-jiri-upload); however, instead of `jiri upload -multipart` in Step 3, use the
following `git` command to upload your changes:

```
git push origin HEAD:refs/for/master%topic=add_my_new_feature
```

