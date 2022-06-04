SHELL:=bash

# documentation site container
DOCKER_ACCOUNT=jdevries3133
CONTAINER_NAME=recipe_docs
TAG?=$(shell git describe --tags)
CONTAINER=$(DOCKER_ACCOUNT)/$(CONTAINER_NAME):$(TAG)

VENVDIR=venv
WITH_VENV=source $(VENVDIR)/bin/activate &&


.PHONY: venv
venv:
	@if [[ -d "$(VENVDIR)" ]]; then \
		echo "venv already exists"; \
		exit 0; \
	fi; \
	python3 -m venv $(VENVDIR); \
	$(WITH_VENV) \
		pip install --upgrade pip && \
		pip install -r requirements.txt


.PHONY: develop
develop: venv
	@# roughly check if documentation dependencies were installed
	if [ "$(shell pip freeze | grep mkdocs)" == "" ]; then \
		$(WITH_VENV) pip install -r requirements.docs.txt; \
	fi
	$(WITH_VENV) mkdocs serve


.PHONY: deploy
deploy:
	docker buildx build --platform linux/amd64 --push -t $(CONTAINER) .
	terraform init -input=false
	terraform apply -auto-approve

