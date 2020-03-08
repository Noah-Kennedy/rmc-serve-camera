# RMC Camera Server
A prototype system for serving camera data.

## Rationale
It is the goal of this repository to evaluate competing options for serving camera
feeds.

There are several elements of the system needing evaluation.

### Evaluation of Protocol Overhead
Several different methods have been proposed to serve camera data, including WebSocket
and direct HTTP requests/responses.

This repository contains competing implementations, which will be benchmarked against
each other.

### Evaluation of Quality/Bandwidth
Network bandwidth increases with image quality and framerate.
This relationship is not necessarily linear, and depends heavily on the scenario.

This repository allows for variable framerate and compression settings in order to allow
for effective analysis of the relationship.

## Overview
TODO