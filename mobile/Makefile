SHELL=/bin/bash

# view devices available to you with `flutter devices`
DEVICE=macos


.PHONY: develop
develop:
	concurrently \
		--names flutter,livereload \
		"flutter run -d $(DEVICE) --pid-file=/tmp/flutter.pid" \
		"make ping-flutter"


.PHONY: reload
reload:
	# trigger flutter_rust_bridge codegen
	just gen
	# send USR1 signal to flutter to trigger reload
	kill -USR1 $(shell cat /tmp/flutter.pid)



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
			find . | grep "\.\(rs\|dart\)$$" | entr -d -p make reload; \
		fi; \
		sleep 1; \
	done
