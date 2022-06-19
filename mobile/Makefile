SHELL=/bin/bash

# view devices available to you with `flutter devices`
DEVICE=macos

# the `ping-flutter` rule can't startup until flutter itself is up and running,
# because it consumes flutter's PID at /tmp/flutter.pid. Therefore, this should
# be roughly set to be greater than flutter's startup time on your system
FLUTTER_STARTUP_DELAY=20


.PHONY: develop
develop:
	concurrently \
		--names flutter,livereload \
		"flutter run -d $(DEVICE) --pid-file=/tmp/flutter.pid" \
		"sh -c \"sleep $(FLUTTER_STARTUP_DELAY) && make ping-flutter\""


# flutter doesn't live reload on file change. The default CLI waits for a `r`
# to be sent to STDIN, but that doesn't work nicely here since we're running
# many layers of concurrentl processes by the top level Makefile, and we don't
# want to keep a separate terminal just for flutter.
#
# Luckily, there is a workaround. The flutter process will reload if it recieves
# a SIGUSR1 signal. So, we can separately run this ping script to ask flutter
# to reload on file change from this separate process.
.PHONY: ping-flutter
ping-flutter:
	while true; \
	do \
		if [ -f /tmp/flutter.pid ]; then \
			find . | grep "\.\(rs\|dart\)$$" | entr -d -p kill -USR1 $(shell cat /tmp/flutter.pid); \
		fi; \
		sleep 1; \
	done
