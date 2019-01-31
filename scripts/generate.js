#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const rimraf = require('rimraf');
const mkdirp = require('mkdirp');

const { getAllEnums, getAllBitFlags, getAllStructs, getAllHandles, getAllFunctions, getAllExtensionNames, getAllTypedefs } = require('./parse');
const { blockToString, toSnakeCase, getWrappedVkTypeName, getRawVkTypeName, documentType } = require('./utils');
const { generateVkStructDefinition } = require('./structs');
const { generateVkEnumDefinition } = require('./enums');
const { generateVkBitFlagsDefinition } = require('./bit_flags');
const { generateVkHandleDefinition } = require('./handles');
const { generateFunctionTableDefinition } = require('./function_table');
const { generateRootFunctionsDefinitions } = require('./root_functions');

const ROOT_DIR_PATH         = path.join(__dirname, '..');
const OUTPUT_DIR_PATH       = path.join(ROOT_DIR_PATH, 'src', 'vulkan');
const STATIC_FILES_DIR_PATH = path.join(__dirname, 'static');

const GENERATED_HEADER      = '// Generated by `scripts/generate.js`\n\n';
const COPIED_HEADER         = '// Copied from `scripts/static/`\n\n';

const STATIC_VK_FUNCTIONS = [
    'vkEnumerateInstanceVersion',
    'vkEnumerateInstanceExtensionProperties',
    'vkEnumerateInstanceLayerProperties',
    'vkCreateInstance'
];

let STRUCTS = null;

function removeFiles() {
    rimraf.sync(OUTPUT_DIR_PATH);
}

function generateFiles() {
    const vkTypes = [
        generateRootFunctions(),
        generateFunctionTable(),
        generateExtensionNames(),
        ...generateEnums(),
        ...generateBitFlags(),
        ...generateStructs(),
        ...generateHandles(),
        ...generateTypedefs()
    ];

    writeVkTypes(vkTypes);
    copyStaticFiles(STATIC_FILES_DIR_PATH, OUTPUT_DIR_PATH);
    writeModFile(OUTPUT_DIR_PATH);
}

function copyStaticFiles(srcDirPath, dstDirPath) {
    const fileNames = fs.readdirSync(srcDirPath);

    mkdir(dstDirPath);

    fileNames.forEach(fileName => {
        const sourcePath = path.join(srcDirPath, fileName);
        const targetPath = path.join(dstDirPath, fileName);
        const stats = fs.statSync(sourcePath);

        if (stats.isFile()) {
            const fileContent = fs.readFileSync(sourcePath, 'utf8');

            fs.writeFileSync(targetPath, COPIED_HEADER + fileContent, 'utf8');
        } else if (stats.isDirectory()) {
            copyStaticFiles(sourcePath, targetPath);
        }
    
    });
}

function writeVkTypes(types) {
    for (let type of types) {
        let { name, extension, definition } = type;
        extension = extension || 'vk';

        const dirPath = extension ? path.join(OUTPUT_DIR_PATH, extension) : OUTPUT_DIR_PATH;
        const fileName = toSnakeCase(name) + '.rs';
        const filePath = path.join(dirPath, fileName);
        const fileContent = GENERATED_HEADER + definition.filter(x => x).map(blockToString).join('\n\n');

        mkdir(dirPath);
        // console.log(fileContent);
        fs.writeFileSync(filePath, fileContent, 'utf8');
    }
}

function mkdir(dirPath) {
    mkdirp.sync(dirPath);
}

function writeModFile(dirPath) {
    const filePath = path.join(dirPath, 'mod.rs');
    const files = [];
    const directories = [];

    fs.readdirSync(dirPath).forEach(name => {
        if (name !== 'mod.rs') {
            const stats = fs.statSync(path.join(dirPath, name));

            if (stats.isFile()) {
                files.push(name);
            } else if (stats.isDirectory()) {
                directories.push(name);
            }
        }
    });

    const moduleNames = files.map(name => name.replace('.rs', ''));
    const content = [
        directories.map(name => `pub mod ${name};`),
        moduleNames.map(name => `mod ${name};`),
        moduleNames.filter(x => x !== 'vk').map(name => `pub use self::${name}::*;`),
    ].map(list => list.join('\n')).filter(x => x).join('\n\n');

    fs.writeFileSync(filePath, GENERATED_HEADER + content);

    directories.forEach(dirName => writeModFile(path.join(dirPath, dirName)));
}

function generateExtensionNames() {
    return {
        name: 'ExtensionNames',
        extension: 'constants',
        definition: getAllExtensionNames().map(({name, value}) => {
            const doc = `/// \`${value}\``;
            return `${doc}\npub const ${name} : &str = ${value};`
        })
    };
}

function generateTypedefs() {
    return getAllTypedefs().map(({baseType, newType}) => {
        const prefix = baseType.extension ? `${baseType.extension}::` : '';
        const baseTypeDef = STRUCTS.find(struct => struct.name === baseType.name && struct.extension === baseType.extension);
        const lifetimes = baseTypeDef ? baseTypeDef.lifetimes : '';
        const baseTypeExt = baseType.extension || 'vk';

        return {
            name: newType.name,
            extension: newType.extension,
            definition: [
                `pub type ${getWrappedVkTypeName(newType.name)}${lifetimes} = super::super::${baseTypeExt}::${prefix}${getWrappedVkTypeName(baseType.name)}${lifetimes};`,
                `#[doc(hidden)]\npub type ${getRawVkTypeName(newType.name)} = super::super::${baseTypeExt}::${prefix}${getRawVkTypeName(baseType.name)};`
            ]
        };
    });
}

function generateVkTypes(cTypes, generateFunction) {
    return cTypes.map(cDef => {
        const rustDefinition = generateFunction(cDef);

        return {
            name: cDef.name,
            extension: cDef.extension,
            definition: rustDefinition
        };
    });
}

function generateRootFunctions() {
    return {
        name: 'RootFunctions',
        extension: '',
        definition: generateRootFunctionsDefinitions(getAllFunctions().filter(func => STATIC_VK_FUNCTIONS.includes(func.name)))
    };
}

function generateFunctionTable() {
    return {
        name: 'VkInstanceFunctionTable',
        extension: '',
        definition: generateFunctionTableDefinition(getAllFunctions().filter(func => !STATIC_VK_FUNCTIONS.includes(func.name)))
    };
}

function generateEnums() {
    return generateVkTypes(getAllEnums(), generateVkEnumDefinition);
}

function generateBitFlags() {
    return generateVkTypes(getAllBitFlags(), generateVkBitFlagsDefinition);
}

function generateStructs() {
    const structs = getAllStructs()
        .filter(struct => struct.fields.every(field => !field.fullType.includes('PFN')));

    STRUCTS = structs;

    return generateVkTypes(structs, generateVkStructDefinition);
}

function isDestroyFunction(func) {
    return func.name.includes('Destroy') || func.name === "vkFreeMemory";
}

function generateHandles() {
    const handles = getAllHandles();
    const functions = getAllFunctions().filter(func => !STATIC_VK_FUNCTIONS.includes(func.name));

    for (let handle of handles) {
        handle.functions = [];
    }

    const destroyFunctions = functions.filter(isDestroyFunction);

    for (let destroyFunction of destroyFunctions) {
        const parentArg = destroyFunction.args.first();
        const destroyedArg = destroyFunction.args.beforeLast();

        if (parentArg !== destroyedArg) {
            const parent = { name: parentArg.typeName, extension: parentArg.extension };

            handles.find(handle => handle.name === destroyedArg.typeName).parent = parent;
        }
    }

    const vkInstance = handles.find(h => h.name === 'VkInstance');

    for (let func of functions) {
        const firstArgType = func.args[0].typeName;
        const secondArg = func.args[1];
        const secondArgType = secondArg && secondArg.typeName;

        let handle = handles.find(handle => {
            return handle.parent &&
                firstArgType === handle.parent.name &&
                secondArgType === handle.name &&
                (!secondArg.isOptional || destroyFunctions.includes(func));
        });

        if (!handle) {
            handle = handles.find(handle => firstArgType === handle.name);
        }

        if (!handle) {
            handle = vkInstance;
        }

        handle.functions.push(func);
    }

    return generateVkTypes(handles, generateVkHandleDefinition);
}

module.exports = {
    generateFiles,
    removeFiles
};