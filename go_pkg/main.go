package main

import "fmt"

func main() {
	fmt.Println("=== WebP Validator - Go Calling Rust ===")

	testFiles := []struct {
		path        string
		description string
	}{
		{"../images/dynamic.webp", "dynamic webp"},
		{"../images/static.webp", "static webp"},
		{"../images/fake.webp", "fake webp (jpg renamed)"},
	}

	for _, test := range testFiles {
		fmt.Printf("testing: %s\n", test.description)
		fmt.Printf("  file: %s\n", test.path)

		info := ValidateWebp(test.path)

		if info.IsValid {
			fmt.Println("  result: valid webp file")
			fmt.Printf("  dimensions: %dx%d\n", info.Width, info.Height)
			fmt.Printf("  has alpha: %v\n", info.HasAlpha)
			fmt.Printf("  is animated: %v\n", info.IsAnimated)
			if info.IsAnimated {
				fmt.Printf("  frames: %d\n", info.NumFrames)
			}
		} else {
			fmt.Println("  result: invalid webp file")
			fmt.Printf("  error: %s\n", info.Error)
		}
		fmt.Println()
	}
}
