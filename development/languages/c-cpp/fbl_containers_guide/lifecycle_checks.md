# Lifecycle checks

To assist developers in catching some simple mistakes, there are a couple of
checks run by `fbl::` containers when node state structure and
containers are destroyed. They are implemented using `ZX_DEBUG_ASSERT` and are
therefore only present when debug asserts are enabled.

First, it is illegal for a node state structure to be destroyed while it exists
in a container. Doing so is considered to be an error and will trigger an
assert. Always make sure that objects are not allowed to be destroyed while
still in a container. This is usually fairly easy to arrange when using managed
pointers to track object lifecycle as objects in a container will need to have
given ownership of at least one object reference to the container, and so the
object cannot destruct until it leaves the container and can have its final
reference go away.

Second, it is illegal for a container of unmanaged pointers to be destroyed
while there are still objects in it. Containers of managed pointers will
automatically `clear()` themselves upon destruction dropping the managed
references that they hold, but containers of unmanaged pointers will not do the
same. If there are still unmanaged pointers in the container when it is
destroyed, it is likely that this was an error and that memory may have been
leaked. Therefore, allowing a non-empty container of unmanaged pointers to be
destroyed will trigger an assert.

