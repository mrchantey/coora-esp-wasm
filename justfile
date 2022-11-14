set windows-shell := ["C:/tools/cygwin/bin/sh.exe","-c"]

default:
	just --list

js-workspaces := 'coora-docs coora-app'


@sync:
	for workspace in {{js-workspaces}}; do \
		just sync-one $workspace; \
	done

@sync-one dest:
	just copy ../koorabel/config/typescript/tsconfig.base.json {{dest}}/config/typescript
	just copy ../koorabel/config/typescript/moduleExt.d.ts {{dest}}/config/typescript
	just copy ../koorabel/config/eslint/.eslintrc.json {{dest}}/config/eslint

# -p make dir if doesnt exist
# -rf handle directories and overwrite
copy src dest:
	mkdir -p {{dest}}
	cp -rf {{src}} {{dest}}