.PHONY: images-redo
images-redo: ## Create webp and avif images 
	cargo run --manifest-path ./helpers/image-optimizer/Cargo.toml -- --recreate

.PHONY: images
images: ## Create webp and avif images 
	cargo run --manifest-path ./helpers/image-optimizer/Cargo.toml

.PHONY: images-watch
