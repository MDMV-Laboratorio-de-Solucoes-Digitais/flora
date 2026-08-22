import prettier from 'eslint-config-prettier';
import path from 'node:path';
import js from '@eslint/js';
import svelte from 'eslint-plugin-svelte';
import { defineConfig, includeIgnoreFile } from 'eslint/config';
import globals from 'globals';
import ts from 'typescript-eslint';
import unicorn from 'eslint-plugin-unicorn';

const gitignorePath = path.resolve(import.meta.dirname, '.gitignore');

export default defineConfig(
	{ ignores: ['src/lib/components/ui/**', 'src/lib/utils.js', 'src/lib/utils.ts'] },
	includeIgnoreFile(gitignorePath),
	js.configs.recommended,
	ts.configs.recommended,
	svelte.configs.recommended,
	prettier,
	svelte.configs.prettier,
	unicorn.configs['flat/recommended'],
	{
		languageOptions: { globals: { ...globals.browser, ...globals.node } },
		rules: {
			'unicorn/single-line-block-comment-style': 'off',
			'unicorn/no-top-level-assignment-in-function': 'off',
			'no-undef': 'off',
			'no-console': 'error',
			'@typescript-eslint/no-explicit-any': 'error',
			'unicorn/prevent-abbreviations': 'off',
			'unicorn/no-null': 'off',
			'unicorn/filename-case': 'off',
			'svelte/no-navigation-without-resolve': 'off'
		}
	},
	{
		files: ['**/*.svelte', '**/*.svelte.ts', '**/*.svelte.js'],
		languageOptions: {
			parserOptions: {
				projectService: true,
				extraFileExtensions: ['.svelte'],
				parser: ts.parser
			}
		}
	}
);
