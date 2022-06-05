SHELL:=/bin/bash

TAILWIND_FLAGS=-c ./tailwind.config.js -o ./tailwind.css
TAILWIND_DEV_FLAGS=--watch
TAILWIND_PROD_FLAGS=--minify

# frontend static site container (nginx-based)
DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=recipe_front
TAG?=$(shell git describe --tags)
CONTAINER=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME):$(TAG)



.PHONY: setup
setup:
	@# check installation of npm package `tailwindcss`
	@which tailwindcss > /dev/null; \
	if [[ $$? -ne 0 ]]; then \
		echo "fatal: must install tailwindcss with ``npm install --global tailwindcss``"; \
		exit 1; \
	fi


.PHONY: develop
develop: setup
	# the top-level makefile checks for the installation of `concurrently` and
	# errors nicely if it's missing, so we will just use it here
	concurrently \
		"tailwindcss $(TAILWIND_FLAGS) $(TAILWIND_DEV_FLAGS)" \
		"trunk serve"


.PHONY: clean
clean:
	trunk clean
	cargo clean
	rm -rf tailwind.css


.PHONY: build
build: setup clean
	tailwindcss $(TAILWIND_FLAGS) $(TAILWIND_PROD_FLAGS)
	trunk build --release
	docker buildx build --platform linux/amd64 --push -t $(CONTAINER) .


.PHONY: deploy
deploy: build
ifdef CI
	terraform init -input=false
endif
	terraform apply -auto-approve

