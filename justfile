set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]

default:
	just --list

js-workspaces := 'coora-docs coora-app'


sync:
	for workspace in {{js-workspaces}}; do \
		just sync-one $workspace; \
	done

sync-one dest:
	cp ../koorabel/config/typescript/tsconfig.json {{dest}}/config/typescript