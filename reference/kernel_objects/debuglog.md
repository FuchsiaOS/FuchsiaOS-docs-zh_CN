# Log

## NAME

Debuglog - Kernel debuglog

## SYNOPSIS

Debuglog objects allow userspace to read and write to kernel debug logs.

## DESCRIPTION

TODO

## NOTES

Debuglog objects will likely cease being generally available to userspace
processes in the future.

## SYSCALLS

 - [`zx_debuglog_create()`] - create a kernel managed debuglog reader or writer
 - [`zx_debuglog_write()`] - write log entry to debuglog
 - [`zx_debuglog_read()`] - read log entries from debuglog

[`zx_debuglog_create()`]: /docs/reference/syscalls/debuglog_create.md
[`zx_debuglog_read()`]: /docs/reference/syscalls/debuglog_read.md
[`zx_debuglog_write()`]: /docs/reference/syscalls/debuglog_write.md
