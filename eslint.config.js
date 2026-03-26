import vue from 'eslint-plugin-vue';
import vueParser from 'vue-eslint-parser';
import * as typescriptParser from '@typescript-eslint/parser';

export default [
    {
        files: ['**/*.vue'],
        languageOptions: {
            parser: vueParser,
            parserOptions: {
                parser: typescriptParser,
                sourceType: 'module',
                ecmaVersion: 'latest',
                extraFileExtensions: ['.vue'],
            },
        },
        plugins: {
            vue,
        },
        rules: {
            'vue/multi-word-component-names': 'off',
        },
    },
];
