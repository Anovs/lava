#!/usr/bin/env node

const path = require('path');
const fs = require('fs');

const {
    DST_DIR_NAME,
    DST_DIR_PATH,
    VULKAN_H
} = require('./constants');

const {
    cToRustVarName,
    capitalizeVarName,
    cToRustEnumValue,
    toRawTypeName,
    toTrueTypeName
} = require('./utils');

main();

function main() {
    writeHandles();
}

function writeHandles() {
    const match = VULKAN_H.match(/\n(?:VK_DEFINE_HANDLE|VK_DEFINE_NON_DISPATCHABLE_HANDLE)\((\w+)\)/g);
    const handlers = match.map(line => line.substring(line.indexOf('(') + 1, line.indexOf(')')));

    const fileContent = [
        'use vk::VkHandle;',
        handlers.map(type => `pub type Raw${type} = VkHandle;`).join('\n'),
        handlers.map(type => `pub type ${type} = VkHandle;`).join('\n')
    ].join('\n\n');
    const filePath = path.join(DST_DIR_PATH, 'vk_handlers.rs');

    fs.writeFileSync(filePath, fileContent, 'utf8');
}