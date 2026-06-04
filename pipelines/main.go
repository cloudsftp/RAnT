package main

import (
	"context"
	"dagger/r-an-t/internal/dagger"
)

type RanT struct{}

// Runs all tests and linters
func (m *RanT) TestAndLint(
	ctx context.Context,
	// +defaultPath="/"
	source *dagger.Directory,
) error {
	var err error

	r := dag.Rust(dagger.RustOpts{
		Packages: []string{
			"g++",
			"fontconfig-dev",
			"fontconfig-static",
			"freetype-static",
		},
	})

	err = r.Check(ctx, source)
	if err != nil {
		return err
	}

	err = r.Test(ctx, source)
	if err != nil {
		return err
	}

	/*
		err = r.Lint(ctx, source)
		if err != nil {
			return err
		}
	*/

	return nil
}
