SHELL:=/bin/bash

TAILWIND_FLAGS=-c ./tailwind.config.js -o ./tailwind.css
TAILWIND_DEV_FLAGS=--watch


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
	concurrently \
		"tailwindcss $(TAILWIND_FLAGS) $(TAILWIND_DEV_FLAGS)" \
		"trunk serve"
