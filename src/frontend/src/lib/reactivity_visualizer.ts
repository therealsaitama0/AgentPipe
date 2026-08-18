import React from 'react';
import { createRoot } from 'react-dom/client';
import ReactDOMServer, { type DocumentNode } from 'html-webpack-plugin';
import './lib/reactivity_visualizer.ts'; // TypeScript wrapper for HTML5 script module support
import dynamicLoader from '../utils/dynamic_loader.js';

// ============================================================================
// UTILITY: Dynamic Loader Wrapper (Universal Plugin Transpiler)
// ============================================================================
const UniversalPlugin = {
    id: 'universal-plugin',
    name: 'DYNAMIC_LOADER_PLUGIN',
    
    // Generates a script tag for HTML5 `<script type="module">` based on runtime environment.
    generateScriptTag(env, version): string {
        let rawContent;

        if (env === 'browser') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'nodejs') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'react-native') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'android') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'ios') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'webview') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'test') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'react-native') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'android') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
</head>`;

        } else if (env === 'ios') {
            const script = `<!DOCTYPE html>
<html lang="en" style="${{ ...styleObject }}">
<head>
    <meta charset="UTF-8">
    ${JSON.stringify(script)} <!-- Script tag for HTML5 `<script type="module">` -->
