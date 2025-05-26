#pragma once

#ifdef  WIN32
#define WINDOWS
#endif

/*
 * General Data Types.
*/
#include <stdint.h>
typedef int8_t       s8;
typedef int16_t      s16;
typedef int32_t       s32;
typedef uint8_t       u8;
typedef uint16_t      u16;
typedef uint32_t       u32;
typedef float               f32;
typedef double              f64;


typedef uint64_t  u64;
typedef int64_t  s64;


#ifndef __cplusplus
typedef enum { false = 0, true } bool;
#endif

#define FASTCALL    __fastcall
#define INLINE      __inline
