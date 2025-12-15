//go:build linux

package main

/*
#cgo LDFLAGS: -L../lib -lwebp_validator -Wl,-rpath,$ORIGIN/../lib
#include "../include/webp_validator.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

type WebpInfo struct {
	IsValid    bool
	Width      uint32
	Height     uint32
	HasAlpha   bool
	IsAnimated bool
	NumFrames  uint32
	Error      string
}

func ValidateWebp(path string) WebpInfo {
	cPath := C.CString(path)
	defer C.free(unsafe.Pointer(cPath))

	result := C.validate_webp_ffi(cPath)

	info := WebpInfo{
		IsValid:    bool(result.is_valid),
		Width:      uint32(result.width),
		Height:     uint32(result.height),
		HasAlpha:   bool(result.has_alpha),
		IsAnimated: bool(result.is_animated),
		NumFrames:  uint32(result.num_frames),
	}

	if result.error_message != nil {
		info.Error = C.GoString(result.error_message)
		C.free_error_message(result.error_message)
	}

	return info
}
