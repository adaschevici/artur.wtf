RECREATE ?= false

# Conditionally set the RECREATE_FLAG variable
ifeq ($(RECREATE),true)
	RECREATE_FLAG = --recreate
else
	RECREATE_FLAG = 
endif

.PHONY: images-redo images images-watch
images: ## Create webp and avif images 
	cargo run --manifest-path ./helpers/image-optimizer/Cargo.toml -- $(RECREATE_FLAG)

images-watch:
	@echo "Watching for image changes..."
