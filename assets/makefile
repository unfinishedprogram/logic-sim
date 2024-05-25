# Makefile

# Define the make commands
.PHONY: build build-font build-sprites build-dot

build: build-font build-sprites build-dot

build-font:
	msdf-atlas-gen \
		-font ./custom-msdf.otf\
		-type mtsdf \
		-format png \
		-size 64 \
		-pxrange 4 \
		-imageout ./custom.png \
		-json ./custom-msdf.json \
		-yorigin top \

build-sprites:
	msdfgen \
		-pxrange 4 \
		-scale 2 \
		-dimensions 256 128 \
		-svg gates/filled.svg \
		-o gates/spritesheet-msdf.png

build-dot:
	msdfgen \
		-pxrange 4 \
		-scale 2 \
		-dimensions 32 32 \
		-svg dot/filled.svg \
		-o dot/spritesheet-msdf.png