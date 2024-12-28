#!/usr/bin/make
include .env
export $(shell sed 's/=.*//' .env)
compose=docker-compose -f docker-compose.yml

.DEFAULT_GOAL := help

.PHONY: rm-speca
rm-speca:
	$(compose) up openapi-generator

.PHONY: speca-gen
speca-gen:
	$(compose) up openapi-generator

.PHONY: speca
speca: rm-speca speca-gen

.PHONY: migrate
migrate:
    sqlx migrate run

.PHONY: clippy
clippy:
    cargo clippy  -- -D warnings


