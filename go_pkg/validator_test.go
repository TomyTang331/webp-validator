package main

import (
	"bytes"
	"fmt"
	"image"
	_ "image/gif"
	_ "image/jpeg"
	_ "image/png"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
	_ "golang.org/x/image/webp"
)

// ValidateWebpByStdLib 使用Go标准库验证图片
// 注意：Go标准库的webp包不支持动态WebP，会报错
func ValidateWebpByStdLib(path string) error {
	data, err := os.ReadFile(path)
	if err != nil {
		return fmt.Errorf("failed to read file: %w", err)
	}

	_, _, err = image.Decode(bytes.NewReader(data))
	if err != nil {
		return fmt.Errorf("invalid image file: %w", err)
	}

	return nil
}

func TestValidateStaticWebp(t *testing.T) {
	info := ValidateWebp("../images/static.webp")

	assert.True(t, info.IsValid, "static webp should be valid")
	assert.False(t, info.IsAnimated, "static webp should not be animated")
	assert.Equal(t, uint32(0), info.NumFrames, "static webp should have 0 frames")
	assert.Greater(t, info.Width, uint32(0), "width should be greater than 0")
	assert.Greater(t, info.Height, uint32(0), "height should be greater than 0")

	t.Logf("static webp validated successfully: %dx%d", info.Width, info.Height)
}

func TestValidateDynamicWebp(t *testing.T) {
	info := ValidateWebp("../images/dynamic.webp")

	assert.True(t, info.IsValid, "dynamic webp should be valid")
	assert.True(t, info.IsAnimated, "dynamic webp should be animated")
	assert.Greater(t, info.NumFrames, uint32(1), "dynamic webp should have multiple frames")
	assert.Greater(t, info.Width, uint32(0), "width should be greater than 0")
	assert.Greater(t, info.Height, uint32(0), "height should be greater than 0")

	t.Logf("dynamic webp validated successfully: %dx%d, %d frames", info.Width, info.Height, info.NumFrames)
}

func TestValidateFakeWebp(t *testing.T) {
	info := ValidateWebp("../images/fake.webp")

	assert.False(t, info.IsValid, "fake webp should be invalid")
	assert.NotEmpty(t, info.Error, "fake webp should have error message")
	assert.Contains(t, info.Error, "webp format validation failed", "error should indicate validation failure")

	t.Logf("fake webp correctly rejected: %s", info.Error)
}

func TestValidateNonexistentFile(t *testing.T) {
	info := ValidateWebp("../images/nonexistent.webp")

	assert.False(t, info.IsValid, "nonexistent file should be invalid")
	assert.NotEmpty(t, info.Error, "nonexistent file should have error message")

	t.Logf("nonexistent file correctly handled: %s", info.Error)
}

// TestCompareWithStdLib 测试动态WebP，证明Go标准库无法处理
// 标准库的image.Decode对动态WebP会报错
func TestCompareWithStdLib(t *testing.T) {
	dynamicWebpPath := "../images/dynamic.webp"

	// 1. 使用Rust库验证
	rustResult := ValidateWebp(dynamicWebpPath)
	require.True(t, rustResult.IsValid, "rust library should validate dynamic webp")
	require.True(t, rustResult.IsAnimated, "rust library should detect animation")
	require.Greater(t, rustResult.NumFrames, uint32(1), "rust library should detect multiple frames")
	t.Logf("rust library result: valid=%v, animated=%v, frames=%d",
		rustResult.IsValid, rustResult.IsAnimated, rustResult.NumFrames)

	// 2. 使用Go标准库验证
	stdlibErr := ValidateWebpByStdLib(dynamicWebpPath)
	assert.Error(t, stdlibErr, "golang stdlib should fail on animated webp")
	if stdlibErr != nil {
		t.Logf("golang stdlib result: error (expected) - %v", stdlibErr)
		t.Logf("this proves golang stdlib cannot handle animated webp files")
	}

	t.Log("conclusion: rust library has full webp support including animation")
	t.Log("golang stdlib lacks animated webp support")
}

// BenchmarkValidateWebp 性能测试
func BenchmarkValidateWebp(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ValidateWebp("../images/static.webp")
	}
}

func BenchmarkValidateWebpStdLib(b *testing.B) {
	for i := 0; i < b.N; i++ {
		ValidateWebpByStdLib("../images/static.webp")
	}
}
