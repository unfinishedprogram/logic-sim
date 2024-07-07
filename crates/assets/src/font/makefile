# Makefile

# Define the make commands
.PHONY: build build-font

build: build-font

build-font:
	msdf-atlas-gen \
		-font ./custom-msdf.otf\
		-type mtsdf \
		-format png \
		-size 32 \
		-pxrange 4 \
		-imageout ./custom.png \
		-json ./custom-msdf.json \
		-pxpadding 4 \
		-yorigin top \
