# Dot Dot Considered Harmful

Child processes on Fuchsia are only capable of accessing the resources provided
to them -- this is an essential idea encompassing microkernels, and other
“capability-based” systems. If a handle is provided to a service, access to
that handle implies the client can use it.

Intuitively, this concept can be applied to filesystems: If a handle is
provided to a directory, it should imply access to resources within that
directory (and additionally, their subdirectories). Unfortunately, however, a
holdout from POSIX prevents directory handles from cleanly integrating with
these concepts in a capability system: “..”. If a handle is provided to a
directory, the client can simply request “..”, and the handle will be
“upgraded” to access the parent directory, with broader scope. As a
consequence, this implies that a handle to a directory can be upgraded
arbitrarily to access the entire filesystem.

Traditionally, filesystems have tried to combat this using "chroot", which
changes the notion of a filesystem root, preventing access beyond ".." in
trivial cases of path traversal. However, this approach has some problems:

  * Chroot changes the notion of root on a coarse, "per-program" basis, not on
    a per-descriptor basis
  * Chroots are often misused (i.e., fchdir to a different open handle which
    sits outside the chroot)
  * Chroots are not "on by default", so it may be tempting for programs to
    simply not use them.

To overcome these deficiencies, Fuchsia does not implement traditional dot dot
semantics on filesystem servers, which would allow open directories to traverse
upward. More specifically, it disallows access to “..”, preventing clients
from trivially accessing parent directories. This provides some strong
properties for process creation: If an application manager only wants to give a
process access to "/data/my_private_data", then it can simply provide a handle
to that open directory to the child process, and it will "automatically" be
sandboxed.

## What about paths which can be resolved without the filesystem server?

Certain paths, such as “foo/../bar”, which can be transformed to “bar”, can be
determined without accessing a filesystem server in the absence of symbolic
links (and at the time of writing, symbolic links do not exist on Fuchsia).
These paths may be canonicalized, or cleaned, on the client-side, prior to
sending path-based requests to filesystem servers: the libfdio library already
does this for any fdio operations which are eventually transmitted to
filesystem servers in a function called `__fdio_cleanpath`.

## What about shell traversal?

I.e, if someone “cd”s into a directory, how can they leave? Internally, the
notion of “CWD” isn’t merely a file descriptor to an open directory; rather,
it’s a combination of “file descriptor” and “absolute path interpreted to mean
CWD”. If all operations to cd act on this absolute path, then “..” can always
be resolved locally on a client, rather than being transmitted to a filesystem
server. For example, if the CWD is “/foo/bar”, and a user calls “cd ..”, then
the underlying call may be transformed into “chdir /foo/bar/..”, which can be
canonicalized to “/foo”.

Once these hurdles have been overcome, the benefits of removing “..” are
enormous: access to filesystem resources fits naturally within the capability
system, [sandboxing](sandboxing.md) new processes becomes massively easier, and
resource access can more naturally be composed through filesystem
[namespaces](namespaces.md).
