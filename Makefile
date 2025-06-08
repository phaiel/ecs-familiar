# ECS Familiar - Schema-First Generation Pipeline
# Following best practices for Copier-based code generation

.PHONY: all clean generate test check format fix lint validate help

# Default target - full pipeline
all: validate generate check

# Validate schemas before generation
validate:
	@echo "🔍 Validating YAML schemas..."
	python3 validate_schemas.py

# Generate entire hot path from cold path schemas  
generate:
	@echo "🏗️ Generating hot path from schemas..."
	python3 generate_all.py

# Check compilation without building
check:
	@echo "🔧 Checking compilation..."
	cd hot_path && cargo check

# Build the project
build:
	@echo "📦 Building project..."
	cd hot_path && cargo build

# Run tests
test:
	@echo "🧪 Running tests..."
	cd hot_path && cargo test

# Format generated code
format:
	@echo "🎨 Formatting generated code..."
	cd hot_path && cargo fmt

# Apply automatic fixes
fix:
	@echo "🔧 Applying automatic fixes..."
	cd hot_path && cargo fix --allow-dirty --allow-staged

# Lint the code
lint:
	@echo "🔍 Linting code..."
	cd hot_path && cargo clippy -- -D warnings

# Clean generated files
clean:
	@echo "🧹 Cleaning generated files..."
	rm -rf hot_path/src/gen/
	rm -f hot_path/src/lib.rs
	rm -f hot_path/src/main.rs
	cd hot_path && cargo clean

# Full regeneration pipeline with formatting
regen: clean validate generate format check

# CI pipeline - validate then regenerate
ci: validate
	@echo "🤖 Running CI pipeline..."
	python3 generate_all.py
	cd hot_path && cargo fmt
	cd hot_path && cargo check
	@echo "✅ CI pipeline completed successfully"

# Development iteration
dev: validate generate
	cd hot_path && cargo run -- --schema-test

# Show usage
help:
	@echo "🧬 ECS Familiar - Schema-First Generation Pipeline"
	@echo ""
	@echo "TARGETS:"
	@echo "  all      - Full pipeline: validate → generate → check"
	@echo "  validate - Validate YAML schemas"
	@echo "  generate - Generate hot path from schemas"
	@echo "  check    - Check compilation without building"
	@echo "  build    - Build the project"  
	@echo "  test     - Run tests"
	@echo "  format   - Format generated code"
	@echo "  fix      - Apply automatic fixes"
	@echo "  lint     - Lint the code"
	@echo "  clean    - Clean generated files"
	@echo "  regen    - Full regeneration with clean"
	@echo "  ci       - CI pipeline with validation"
	@echo "  dev      - Development iteration with test"
	@echo "  help     - Show this help"
	@echo ""
	@echo "EXAMPLES:"
	@echo "  make ci                    # Run CI pipeline"
	@echo "  make dev                   # Quick development iteration"
	@echo "  make regen && make lint    # Full regen with linting" 