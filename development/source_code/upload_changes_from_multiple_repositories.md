
# Upload changes from multiple repositories

Certain changes require modifying more than one repository simultaneously.  There are two
supported methods for accomplishing this: soft and hard transitions.

In general, prefer soft transitions over hard transitions (see
[Making changes across multiple petals](working_across_petals.md#hard-and-soft-transitions) for
further details). This means that if a change to one repository depends on a change in another
repository, you must wait until the respective commit has been rolled before the dependent commit
can be submitted to the queue.

Most multi-Petal changes (including
[updates to FIDL protocols](workflow_tips_and_faq.md#q_how_do_i_update_a_fidl_protocol)) should
be attempted using a soft transition, whereas coordinating
[changes across multiple Petals](workflow_tips_and_faq.md#q_how_do_i_coordinate_changes_across_multiple_petals)
*may* require a hard transition.  Typically, one should use the techniques described in the above
references to avoid hard transitions wherever possible.


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
following `git` command to upload your changes from each repository you have modified:

```
git push origin HEAD:refs/for/main
```

Note that this command must be run from the working directory of each repository.


