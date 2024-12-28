#!/usr/bin/make
include .env
export $(shell sed 's/=.*//' .env)
compose=docker-compose -f docker-compose.yml

.DEFAULT_GOAL := help

.PHONY: wakeapp
wakeapp:
	cp .env.example .env
	$(compose) up -d --force-recreate --remove-orphans
	$(compose) exec php-fpm bin/console d:m:m -n
	make speca

.PHONY: build
build:
	$(compose) up -d --build --force-recreate --remove-orphans

.PHONY: speca
speca:
	$(compose) up openapigen

.PHONY: tests
tests:
	$(compose) exec php-fpm bin/phpunit --coverage-html /var/www/var/coverage

.PHONY: ecs
ecs:
	$(compose) exec php-fpm ./vendor/bin/ecs check src $(arg1)

.PHONY: stan
stan:
	$(compose) exec php-fpm ./vendor/bin/phpstan