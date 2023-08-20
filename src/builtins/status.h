// Prototypes for executing builtin_status function.
#ifndef FISH_BUILTIN_STATUS_H
#define FISH_BUILTIN_STATUS_H

#include "../maybe.h"
#include "../parser.h"

struct IoStreams;
using io_streams_t = IoStreams;

maybe_t<int> builtin_status(const parser_t &parser, io_streams_t &streams, const wchar_t **argv);
int builtin_status_ffi(void *parser, void *streams, const void *argv);
#endif
