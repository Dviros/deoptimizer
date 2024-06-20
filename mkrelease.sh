
#!/bin/bash

# Create the release directory structure
mkdir -p release/src/arm
mkdir -p release/src/riscv
mkdir -p release/src/plugin
mkdir -p release/tests

# Copy core source files
cp src/main.rs release/src/
cp src/options.rs release/src/
cp src/utils.rs release/src/

# Copy ARM specific files
cp src/arm/analysis.rs release/src/arm/
cp src/arm/deoptimizer.rs release/src/arm/
cp src/arm/transforms.rs release/src/arm/
cp src/arm/encoding.rs release/src/arm/

# Copy RISCV specific files
cp src/riscv/analysis.rs release/src/riscv/
cp src/riscv/deoptimizer.rs release/src/riscv/
cp src/riscv/transforms.rs release/src/riscv/
cp src/riscv/encoding.rs release/src/riscv/

# Copy plugin system files
cp src/plugin.rs release/src/

# Copy example plugins
cp src/example_plugins.rs release/src/

# Copy visualization tool
cp src/visualization.rs release/src/

# Copy reporting module
cp src/reporting.rs release/src/

# Copy tests
cp tests/automated_tests.rs release/tests/

echo "Release package created successfully with ARM and RISCV support."
