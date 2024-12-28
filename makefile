#!/usr/bin/make
include .env
export $(shell sed 's/=.*//' .env)
compose=docker-compose -f docker-compose.yml

.DEFAULT_GOAL := help

.PHONY: migrate
migrate:
    sqlx migrate run

.PHONY: clippy
clippy:
    cargo clippy  -- -D warnings

.PHONY: fmt
fmt:
    cargo fmt -- --check


