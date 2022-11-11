# screen-ruler 
Simple service to configure screens and monitors according to predefined rules.

## Maturity
This project is really early alpha and works ~~well~~ on my machine. This project is not ready for public consumption and might never be.

## Motivation
Previous I used kscreen (which is a port of KDE) on Manjaro but it sporadically lost its settings and failed to switch to my TV when the reciever were turned on. 

I created this project to create a more deterministic feeling screen manager. 

## End goal
Three projects:
- screen-ruler - service that manages screens
- screen-ruler-gui - Manage and configure screens and rules
- screen-ruler-settings - Shared settings object between gui and service

Capabilities:
  - Switch between screen setups according to rules
  - Override rules with keyboard commands to change to specific setup

## MVP
1. Switch between hardcoded screen setups - Done
2. Use settings file to configure screen setups (with offsets)
3. Run as a service (Be able to disable kscreen)
4. Configure screen-ruler with GUI

