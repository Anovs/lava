#!/usr/bin/env node

const path = require('path');
const fs = require('fs');
const { parseType, parseFunction, isHandle } = require('./parse_vulkan_h');

const ROOT = path.join(__dirname, '..');
const DST_DIR_NAME = 'vk';
const DST_DIR_PATH = path.join(ROOT, 'src', DST_DIR_NAME);
const SCHEMA = require('./schema');

const HEADER = '// Generated by `scripts/generate_vk.js`';

const IGNORED_TYPES = [
    'VkDebugReportCallbackCreateInfo',
    'VkAllocationCallbacks'
];

const IGNORED_EXTERN_FUNCTIONS = [
    'vkCreateDebugReportCallbackEXT',
    'vkDestroyDebugReportCallbackEXT'
];

const PRIMITIVE_TYPES = {
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    void: 'c_void',
    VkAllocationCallbacks: 'c_void'
};

const UNSAFE_FUNCTIONS = [
    'copy_as_string',
    'copy_as_string_vec',
    'copy_as_c_string',
    'free_c_string',
    'copy_as_c_string_array',
    'free_c_string_array',
    'copy_as_c_ptr',
    'free_c_ptr',
    'copy_as_c_array',
    'free_c_array',
    'vec_from_c_ptr'
];

const META_TYPE_TO_BUILD_FUNC = {
    handle: buildHandle,
    struct: buildStruct,
    enum: buildEnum,
    flags: buildBitFlags,
    special: buildSpecial
};

const SPECIAL_TYPES_BUILD_FUNC = {
    VkBool32: buildVkBool32,
    VkDeviceSize: buildVkDeviceSize
};

const DEFINITIONS = {};

main();

function main() {
    buildType('VkDebugReportFlagsEXT');
    // buildType('VkDebugReportObjectTypeEXT');
    // buildType('VkDebugReportCallbackCreateInfoEXT');
    buildType('VkInstance');

    if (!fs.existsSync(DST_DIR_PATH)) {
        fs.mkdirSync(DST_DIR_PATH);
    }

    Object.entries(DEFINITIONS).forEach(([typeName, typeDefinition]) => {
        if (typeDefinition) {
            const fileContent = generateTypeFileContent(typeName, typeDefinition);
            writeVkType(typeName, fileContent);
        }
    });


    writeStaticFiles(DST_DIR_PATH);
    refreshModRs(DST_DIR_PATH);
}

function writeStaticFiles(dirPath) {
    const srcDir = path.join(__dirname, 'static');
    const fileNames = fs.readdirSync(srcDir);

    fileNames.forEach(fileName => {
        const fileContent = fs.readFileSync(path.join(srcDir, fileName), 'utf8');
        const targetFilePath = path.join(dirPath, fileName.replace('vk_', ''));
        
        fs.writeFileSync(targetFilePath, `${HEADER}\n\n${fileContent}`, 'utf8');
    });
}

function refreshModRs(dirPath) {
    const filePath = path.join(dirPath, 'mod.rs');
    const moduleNames = fs.readdirSync(dirPath).filter(name => name !== 'mod.rs').map(str => str.replace('.rs', ''));
    const content = [
        HEADER,
        ``,
        ...moduleNames.map(name => `mod ${name};`),
        ``,
        ...moduleNames.map(name => `pub use self::${name}::*;`),
    ].join('\n');

    fs.writeFileSync(filePath, content);
}

function writeVkType(name, blocks) {
    blocks.unshift(HEADER);

    const moduleName = toSnakeCase(name);
    const filePath = path.join(DST_DIR_PATH, `${moduleName}.rs`);
    const fileContent = blocks.map(block => {
        if (Array.isArray(block)) {
            return blockToStr(block);
        } else {
            return block;
        }
    }).filter(x => !!x).join('\n\n');

    // console.log(fileContent);

    fs.writeFileSync(filePath, fileContent, 'utf8');
}

function methodToBlock(method) {
    const { name, args, returnType, body, public, unsafe } = method;

    return [
        `\n${public ? 'pub ' : ''}fn ${name}(${args.map(({name, type}) => (name ? `${name}: `: '') + type).join(', ')})` + (returnType ? ` -> ${returnType}` : ''),
        unsafe ? [`unsafe`, body] : body
    ];
}

function externalArgumentToRust(arg) {
    const { name, fullType, typeName, isPointer, isConst } = arg;

    const type = `${isPointer ? `*${isConst ? 'const ' : 'mut '}` : ''}${toRawTypeName(typeName)}`;

    return `${toSnakeCase(name)}: ${type}`;
}

function externalMethodToBlock(method) {
    const { name, returnType, args } = method;

    return `fn ${name}(${args.map(externalArgumentToRust).join(', ')})${returnType !== 'void' ? `-> RawVkResult`: ''};`;
}

function typeToBlock(typeName, definition) {
    const metaType = definition.type;

    if (metaType === 'typedef') {
        return `pub type ${typeName} = ${definition.target};`;
    } else if (metaType === 'struct') {
        const headers = [
            definition.reprC ? '#[repr(C)]' : `#[derive(Debug)]`
        ];

        if (definition.copiable) {
            headers.push('#[derive(Copy, Clone)]');
        }

        const lifetimes = lifetimesToStr(definition.lifetimes);

        return [`${headers.join('\n')}\npub struct ${typeName}${lifetimes}`, definition.fields.map(({name, type, public}) => `${public ? 'pub ' : ''}${name}: ${type},`)];
    } else if (metaType === 'enum') {
        const header = [
            '#[repr(i32)]',
            '#[derive(Debug, PartialEq, Copy, Clone)]'
        ].join('\n');
        
        return [`${header}\npub enum ${typeName}`, definition.fields.map(({name, value}) => `${name} = ${value},`)];
    }
}

function generateTypeFileContent(typeName, definition) {
    const { rawTypeName, wrappedTypeName, uses, rawDefinition, wrappedDefinition, methods, rawDropMethod, wrappedDropMethod, externFunctions, fromRawToWrapped, fromWrappedToRaw, flagsMethods } = definition;

    const baseUses = ['vk::*', 'std::os::raw::c_char'];

    if (rawDropMethod || wrappedDropMethod) {
        baseUses.push(`std::ops::Drop`);
    }

    const lt = lifetimesToStr(wrappedDefinition.lifetimes);

    const blocks = [
        baseUses.concat(Array.from(uses || [])).map(use => `use ${use};`).join('\n'),
        typeToBlock(rawTypeName, rawDefinition),
        typeToBlock(wrappedTypeName, wrappedDefinition),
        methods && methods.length ? [`impl ${wrappedTypeName}`, flatten(methods.map(methodToBlock))] : '',
        flagsMethods && flagsMethods.length ? [`impl VkFlags for ${wrappedTypeName}`, flatten(flagsMethods.map(methodToBlock))] : '',
        fromWrappedToRaw ? [`impl${lt} VkFrom<${wrappedTypeName}${lt}> for ${rawTypeName}`, methodToBlock(fromWrappedToRaw)] : '',
        fromRawToWrapped ? [`impl${lt} VkFrom<${rawTypeName}> for ${wrappedTypeName}${lt}`, methodToBlock(fromRawToWrapped)] : '',
        rawDropMethod ? [`impl Drop for ${rawTypeName}`, methodToBlock(rawDropMethod)] : '',
        wrappedDropMethod ? [`impl${lt} Drop for ${wrappedTypeName}${lt}`, methodToBlock(wrappedDropMethod)] : '',
        externFunctions && externFunctions.length ? [`extern`, externFunctions.map(externalMethodToBlock)] : ''
    ];

    return blocks;
}

function buildType(typeName) {
    typeName = formatCTypeName(typeName);

    if (!typeName.startsWith('Vk') || IGNORED_TYPES.includes(typeName) || typeName in DEFINITIONS) {
        return;
    }

    DEFINITIONS[typeName] = null;

    const cDef = parseType(typeName);
    const metaType = cDef.type;
    const rawInfo = metaType === 'handle' ? (SCHEMA[typeName] || {}) : cDef;
    const definition = META_TYPE_TO_BUILD_FUNC[metaType](typeName, rawInfo);

    definition.metaType = metaType;
    DEFINITIONS[typeName] = definition;
}

function buildVkBool32() {
    return {
        rawTypeName: 'RawVkBool32',
        wrappedTypeName: 'VkBool32',
        rawDefinition: { type: 'typedef', target: 'u32' },
        wrappedDefinition: { type: 'typedef', target: 'bool' },
        fromRawToWrapped: { name: 'vk_from', args: [{name: 'value', type: '&RawVkBool32'}], returnType: 'Self', body: [`*value != 0`] },
        fromWrappedToRaw: { name: 'vk_from', args: [{name: 'value', type: '&VkBool32'}], returnType: 'Self', body: [`if *value { 1 } else { 0 }`] }
    };
}

function buildVkDeviceSize() {
    return {
        rawTypeName: 'RawVkDeviceSize',
        wrappedTypeName: 'VkDeviceSize',
        rawDefinition: { type: 'typedef', target: 'u64' },
        wrappedDefinition: { type: 'typedef', target: 'u64' },
        fromRawToWrapped: { name: 'vk_from', args: [{name: 'value', type: '&u64'}], returnType: 'u64', body: [`*value`] }
    };
}

function buildSpecial(typeName) {
    return SPECIAL_TYPES_BUILD_FUNC[typeName](typeName);
}

function createLifetimeIdCounter() {
    return {
        _list: [],
        list() {
            return this._list;
        },
        next() {
            const lifetimeId = String.fromCharCode('a'.charCodeAt(0) + this._list.length);
            this._list.push(lifetimeId);

            return lifetimeId;
        }
    };
}

function lifetimesToStr(list) {
    return (list && list.length) ? `<${list.map(x => `'${x}`).join(', ')}>` : '';
}

function formatStructureTypeName(str) {
    return str
        .substring(2)
        .replace(/[A-Z]+$/, str => str.charAt(0) + str.substring(1).toLowerCase());
}

function buildStruct(typeName, parsed) {
    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);

    const uses = [
        `std::ptr::null`,
        'libc::*'
    ];

    const lifetimeIdCounter = createLifetimeIdCounter();
    const srcVar = 'value';
    const rawFields = [];
    const wrappedFields = [];
    const fromRawToWrappedFields = [];
    const fromWrappedToRawFields = [];
    const dropStatements = [];
    const disableRawToWrapped = parsed.fields.some(arg => arg.isPointer);

    parsed.fields.forEach((field, index) => {
        buildType(field.typeName);

        const prevField = parsed.fields[index - 1];
        const nextField = parsed.fields[index + 1];

        const rustFieldName = getRustFieldName(field);
        const rawRustFieldType = getRawRustFieldType(field);
        const isCount = areCountAndArray(field, nextField);
        const isArrayPtr = areCountAndArray(prevField, field);
        const arraySize = field.arraySize;
        const isStaticArray = !!field.arraySize;
        const isPointer = field.isPointer;
        const isTypeChar = field.typeName === 'char';
        const isPrimitiveType = !!PRIMITIVE_TYPES[field.typeName];
        const isHandleType = isHandle(field.typeName);
        const representIndex = doesFieldRepresentIndex(field);
        const representVersion = doesRepresentVersion(field);

        rawFields.push({ name: rustFieldName, type: rawRustFieldType });

        if (field.name === 'sType') {
            fromWrappedToRawFields.push(`${rustFieldName}: VkFrom::vk_from(&VkStructureType::${formatStructureTypeName(typeName)})`);
        } else if (field.name === 'pNext') {
            fromWrappedToRawFields.push(`${rustFieldName}: null()`);
        } else if (isCount) {
            const vecFieldName = getRustFieldName(nextField);
            fromWrappedToRawFields.push(`${rustFieldName}: ${srcVar}.${vecFieldName}.len() as u32`);
        } else if (representVersion) {
            fromWrappedToRawFields.push(`${rustFieldName}: vk_make_version(&${srcVar}.${rustFieldName})`);
            fromRawToWrappedFields.push(`${rustFieldName}: vk_from_version(${srcVar}.${rustFieldName})`);
            wrappedFields.push({ name: rustFieldName, type: '[u32; 3]', public: true });
        } else {
            const src = `${srcVar}.${rustFieldName}`;
            const ref = isHandleType ? '' : '&';
            const wrappedToRawCollectionConversion = isPrimitiveType
                ? (representIndex ? `.iter().map(|x| *x as u32).collect()` : '')
                : `.iter().map(|x| VkFrom::vk_from(x)).collect()`;
            const wrappedToRawSingleEltConversion = isPrimitiveType
                ? (representIndex ? `${src} as u32` : src)
                : `VkFrom::vk_from(${ref}${src})`;
            const rawToWrappedCollectionConversion = isPrimitiveType
                ? (representIndex ? `.iter().map(|x| *x as usize).collect()` : '')
                : `.iter().map(|x| VkFrom::vk_from(x)).collect()`;
            const rawToWrappedSingleEltConversion = isPrimitiveType
                ? (representIndex ? `${src} as usize` : src)
                : `VkFrom::vk_from(&${isPointer ? `(*${src})` : src})`;
            const wrappedTypeName = representIndex ? 'usize' : toWrappedTypeName(field.typeName);
            let wrappedFieldType = null;

            if (isArrayPtr) {
                const countVar = `${srcVar}.${getRustFieldName(prevField)}`;

                if (isStringArray(field)) {
                    wrappedFieldType = `Vec<String>`;
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_string_array(&${src})`);
                    fromRawToWrappedFields.push(`${rustFieldName}: copy_as_string_vec(${countVar}, ${src} as ${rawRustFieldType.replace(/\*mut /g, '*const ')})`);
                    dropStatements.push(`free_c_string_array(self.${rustFieldName});`);
                } else {
                    wrappedFieldType = `Vec<${wrappedTypeName}>`;
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_array(&${src}${wrappedToRawCollectionConversion})`);
                    fromRawToWrappedFields.push(`${rustFieldName}: vec_from_c_ptr(${countVar}, ${src})${rawToWrappedCollectionConversion}`);
                    dropStatements.push(`free_c_array(self.${rustFieldName});`);
                }
            } else if (isPointer) {
                if (isTypeChar) {
                    wrappedFieldType = 'String';
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_string(&${src})`);
                    fromRawToWrappedFields.push(`${rustFieldName}: copy_as_string(${src})`)
                    dropStatements.push(`free_c_string(self.${rustFieldName});`);
                } else {
                    if (isHandleType) {
                        const lifetimeId = lifetimeIdCounter.next();
                        wrappedFieldType = `&'${lifetimeId} ${wrappedTypeName}`
                    } else {
                        wrappedFieldType = wrappedTypeName;
                    }
                    fromWrappedToRawFields.push(`${rustFieldName}: copy_as_c_ptr(${wrappedToRawSingleEltConversion})`);
                    fromRawToWrappedFields.push(`${rustFieldName}: ${rawToWrappedSingleEltConversion}`);
                    dropStatements.push(`free_c_ptr(self.${rustFieldName});`);
                }
            } else if (isStaticArray) {
                if (isTypeChar) {
                    wrappedFieldType = 'String';
                    fromWrappedToRawFields.push(`${rustFieldName}: [0; ${arraySize}]`);
                    fromRawToWrappedFields.push(`${rustFieldName}: copy_as_string(&${src} as *const c_char)`);
                } else {
                    wrappedFieldType = `[${wrappedTypeName}; ${arraySize}]`;
                    fromWrappedToRawFields.push(`${rustFieldName}: ${src}${wrappedToRawCollectionConversion}`);
                    fromRawToWrappedFields.push(`${rustFieldName}: ${src}${rawToWrappedCollectionConversion}`);
                }
            } else {
                if (isHandleType) {
                    const lifetimeId = lifetimeIdCounter.next();
                    wrappedFieldType = `&'${lifetimeId} ${wrappedTypeName}`
                } else {
                    wrappedFieldType = wrappedTypeName;
                }
                if (field.name.startsWith('old')) {
                    wrappedFieldType = `Option<${wrappedFieldType}>`;
                    fromWrappedToRawFields.push(`${rustFieldName}: match ${src} {\n    Some(raw) => ${wrappedToRawSingleEltConversion.replace(src, 'raw')},\n    None => VK_NULL_HANDLE\n}`);
                } else {
                    fromWrappedToRawFields.push(`${rustFieldName}: ${wrappedToRawSingleEltConversion}`);
                }

                fromRawToWrappedFields.push(`${rustFieldName}: ${rawToWrappedSingleEltConversion}`);
            }

            wrappedFields.push({ name: rustFieldName, type: wrappedFieldType, public: true });
        }
    });

    const flagsMethods = [];

    if (wrappedFields.every(({type}) => type === 'bool')) {
        flagsMethods.push(...getNoneAndAllMethods(wrappedFields));
    }

    const lifetimes = lifetimeIdCounter.list();

    const rawDefinition = { type: 'struct', reprC: true, fields: rawFields };
    const wrappedDefinition = { type: 'struct', reprC: false, fields: wrappedFields, lifetimes: lifetimes };

    const fromWrappedToRaw = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${wrappedTypeName}`}],
        returnType: 'Self',
        body: [`Self`, fromWrappedToRawFields.map(x => `${x},`)],
        unsafe: fromWrappedToRawFields.some(containsUnsafeFunction),
        lifetimes: lifetimes
    };

    const fromRawToWrapped = disableRawToWrapped ? null : {
        name: 'vk_from',
        args: [{name: 'value', type: `&${rawTypeName}`}],
        returnType: 'Self',
        body: [`Self`, fromRawToWrappedFields.map(x => `${x},`)],
        unsafe: fromRawToWrappedFields.some(containsUnsafeFunction),
        lifetimes: lifetimes
    };

    const rawDropMethod = !dropStatements.length ? null : {
        name: 'drop',
        args: [{type: '&mut self'}],
        returnType: null,
        body: dropStatements,
        unsafe: dropStatements.some(containsUnsafeFunction)
    };

    return {
        uses,
        rawTypeName, wrappedTypeName,
        rawDefinition, wrappedDefinition,
        fromRawToWrapped, fromWrappedToRaw,
        rawDropMethod, flagsMethods
    };
}

function containsUnsafeFunction(str) {
    return UNSAFE_FUNCTIONS.some(func => str.includes(func));
}

function doesRepresentVersion(field) {
    return field.typeName === 'uint32_t' && /Version$/.test(field.name);
}

function doesFieldRepresentIndex(field) {
    return field.typeName === 'uint32_t' && /Index|Indices/.test(field.name);
}

function isStringArray(field) {
    return field.fullType === 'const char* const*';
}

function getRawRustFieldType(field) {
    const { name, typeName, isPointer, arraySize } = field;

    if (name === 'pNext') {
        return `*const c_void`;
    } else if (isStringArray(field)) {
        return `*mut *mut c_char`;
    } else {
        const rawTypeName = toRawTypeName(typeName);

        if (isPointer) {
            return `*mut ${rawTypeName}`;
        } else if (arraySize) {
            return `[${rawTypeName}; ${arraySize}]`;
        } else {
            return rawTypeName;
        }
    }
}

function getRustFieldName({name}) {
    return toSnakeCase(name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]));
}

function areCountAndArray(field1, field2) {
    return field1 && field2 && field1.name.endsWith('Count') && field1.fullType === 'uint32_t' && field2.isPointer && field2.typeName !== 'char';
}

function findEnumPrefix(typeName) {
    if (typeName === 'VkResult') {
        return 'VK_';
    }

    return typeName
        .replace(/[A-Z]+$/, '')
        .replace(/[A-Z]+/g, `_$&`)
        .toUpperCase()
        .substring(1);
}

function buildEnum(typeName, rawInfo) {
    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);
    const rawDefinition = { type: 'typedef', target: 'i32' };

    const prefix = findEnumPrefix(typeName);
    const fields = rawInfo.fields.map(({name, value}) => {
        let rustName = toPascalCase(name.substring(prefix.length));

        if (/^\d/.test(rustName)) {
            rustName = `_${rustName}`;
        }

        return {
            name: rustName,
            value: value
        };
    });

    const wrappedDefinition = { type: 'enum', fields: fields };

    const fromWrappedToRaw = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${wrappedTypeName}`}],
        returnType: 'Self',
        body: [`*value as i32`]
    };

    const fromRawToWrapped = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${rawTypeName}`}],
        returnType: 'Self',
        body: [`*((value as *const i32) as *const ${wrappedTypeName})`],
        unsafe: true
    };

    return { rawTypeName, wrappedTypeName, rawDefinition, wrappedDefinition, fromWrappedToRaw, fromRawToWrapped };
}

function buildBitFlags(typeName, rawInfo) {
    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);
    const rawDefinition = { type: 'typedef', target: 'u32' };

    const prefixToRemove = typeName
        .substring(0, typeName.indexOf('Flags'))
        .replace(/[a-z]+/g, `$&_`).toUpperCase();

    const prefixToAdd = typeName === 'VkSampleCountFlags' ? 'bit' : '';

    const fields = rawInfo.fields.map(({name, value}) => {
        return {
            name: prefixToAdd + name.substring(prefixToRemove.length).replace('_BIT', '').toLowerCase(),
            value: value
        };
    });

    const wrappedDefinition = {
        type: 'struct',
        copiable: true,
        fields: fields.map(({name}) => ({ name: name, type: 'bool', public: true }))
    };

    const fromWrappedToRaw = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${wrappedTypeName}`}],
        returnType: 'Self',
        body: [['0'].concat(fields.map(({name, value}) => `+ (if value.${name} { ${value} } else { 0 })`))]
    }

    const fromRawToWrapped = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${rawTypeName}`}],
        returnType: 'Self',
        body: [`Self`, fields.map(({name, value}) => `${name}: (value & ${value}) != 0,`)]
    }

    const flagsMethods = getNoneAndAllMethods(fields);

    return { rawTypeName, wrappedTypeName, rawDefinition, wrappedDefinition, fromWrappedToRaw, fromRawToWrapped, flagsMethods };
}

function getNoneAndAllMethods(fields) {
    return [
        {
            name: 'none',
            args: [],
            returnType: 'Self',
            body: [`Self`, fields.map(({name}) => `${name}: false,`)]
        },
        {
            name: 'all',
            args: [],
            returnType: 'Self',
            body: [`Self`, fields.map(({name}) => `${name}: true,`)]
        }
    ];
}

function buildHandle(typeName, rawInfo) {
    const uses = new Set([
        `std::vec::Vec`,
        `std::ptr::null`,
        `libc::c_void`,
        `glfw::*`
    ]);

    const rawTypeName = toRawTypeName(typeName);
    const wrappedTypeName = toWrappedTypeName(typeName);
    const fields = [];
    const rawDefinition = { type: 'typedef', target: 'RawVkHandle' };
    const wrappedDefinition = { type: 'struct', fields: fields };
    const methods = [];
    const externFunctions = [];
    let wrappedDropMethod = null;
    let rawDropMethod = null;
    let fromWrappedToRaw = null;
    let fromRawToWrapped = null;

    fields.push({name: '_handle', type: rawTypeName});
    
    methods.push({
        name: 'handle',
        args: [{type: '&self'}],
        returnType: rawTypeName,
        body: ['self._handle'],
        public: true
    });

    if (rawInfo.drop) {
        const cMethodName = rawInfo.drop;
        const cMethod = parseFunction(cMethodName);
        const cArgs = cMethod.args.map(arg => {
            if (arg.typeName === 'VkAllocationCallbacks') {
                return 'null()';
            } else {
                buildType(arg.typeName);

                const rawArgType = toRawTypeName(arg.typeName);
                let field = fields.find(({type}) => type === rawArgType);

                if (!field) {
                    field = {name: `_${arg.name}`, type: rawArgType};
                    fields.push(field);
                }

                return `self.${field.name}`;
            }
        });
        const body = [`${cMethodName}(${cArgs.join(', ')});`];
        
        wrappedDropMethod = {
            name: 'drop',
            args: [{type: '&mut self'}],
            returnType: null,
            body: body,
            unsafe: true
        };

        if (!IGNORED_EXTERN_FUNCTIONS.includes(cMethod.name)) {
            externFunctions.push(cMethod);
        }
    }

    Object.entries(rawInfo).forEach(([methodName, def]) => {
        if (methodName === 'drop') {
            return;
        }

        const [target, option] = def.split(';').map(x => x.trim());

        let statements = [];
        let returnType = null;
        let methodArguments = [];
        let static = false;
        let mutSelf = false;
        let unsafe = false;
        let setAdditionalFields = [];

        if (!target.includes('::')) {
            const cMethod = parseFunction(target);

            buildType(cMethod.returnType);
            cMethod.args.forEach((arg => buildType(arg.typeName)));

            const cMethodArgs = cMethod.args;
            const lastArg = cMethodArgs[cMethodArgs.length - 1];
            const instanceVarName = toSnakeCase(lastArg.typeName.substring(2));
            const beforeLastArg = cMethodArgs[cMethodArgs.length - 2];
            const createSomething = lastArg.isPointer && !lastArg.isConst;
            const createList = createSomething && beforeLastArg.isPointer && !beforeLastArg.isConst && beforeLastArg.typeName === 'uint32_t';
            const isConstructor = createSomething && lastArg.typeName === typeName;
            const returnVkResult = cMethod.returnType === 'VkResult';
            const argsForCMethod = cMethodArgs.map((arg, index) => {
                if (createSomething && index === cMethodArgs.length - 1) {
                    return 'ptr';
                } else if (createList && index === cMethodArgs.length - 2) {
                    return 'count';
                } else if (arg.typeName === 'VkAllocationCallbacks' || arg.name === 'pLayerName') {
                    return 'null()';
                } else if (arg.typeName === 'GLFWwindow' && arg.isPointer) {
                    methodArguments.push({
                        name: 'glfw_window',
                        type: `&GlfwWindow`
                    });

                    return `glfw_window.handle()`;
                } else {
                    const rawArgType = toRawTypeName(arg.typeName);
                    const wrappedArgType = toWrappedTypeName(arg.typeName);
    
                    if (arg.isPointer && arg.isConst) {
                        // Structure that needs to be passed to the rust method
    
                        const rustArgName = toSnakeCase(arg.name.substring(1));
                        const rawVarName = `raw_${rustArgName}`;
                        const ptrVarName = `${rawVarName}_ptr`;
    
                        statements.push(
                            `let mut ${rawVarName} = ${rawArgType}::vk_from(${rustArgName});`, // TODO: properly convert VkBool32
                            `let ${ptrVarName} = &mut ${rawVarName} as *mut ${rawArgType};`
                        );
    
                        methodArguments.push({
                            name: rustArgName,
                            type: `&${wrappedArgType}`
                        });
    
                        return ptrVarName;
                    } else if (!arg.isPointer) {
                        if (arg.typeName.startsWith('Vk')) {
                            const field = fields.find(({type}) => type === rawArgType);
    
                            if (field && !isConstructor) {
                                return `self.${field.name}`;
                            } else {
                                const rustArgName = toSnakeCase(arg.name);
                                const handleName = `${rustArgName}_handle`;
    
                                statements.push(`let ${handleName} = ${rustArgName}.handle();`);
                                methodArguments.push({ name: rustArgName, type: `&${wrappedArgType}` });
    
                                if (isConstructor && field) {
                                    setAdditionalFields.push(`${instanceVarName}.${field.name} = ${handleName};`);
                                }
    
    
                                return handleName;
                            }
                        } else {
                            const rustArgName = toSnakeCase(arg.name);
                            const convertToUsize = rustArgName.endsWith('_index') && wrappedArgType === 'u32';
                            const rustArgType = convertToUsize ? 'usize' : wrappedArgType;
                            methodArguments.push({ name: rustArgName, type: rustArgType });
    
                            return rustArgName + (convertToUsize ? ' as u32' : '');
                        }
                    }
                }
    
                return '<missing>';
            });
    
            if (!IGNORED_EXTERN_FUNCTIONS.includes(cMethod.name)) {
                externFunctions.push(cMethod);
            }

            unsafe = true;

            if (isConstructor) {
                static = true;
            }
    
            if (createSomething) {
                const returnedTypeName = toWrappedTypeName(lastArg.typeName);
                returnType = returnedTypeName;
    
                if (createList) {
                    returnType = `Vec<${returnType}>`;
                }
    
                if (returnVkResult) {
                    returnType = `Result<${returnType}, VkResult>`;
                }
    
                const vkWrapperCallName = `vk_call_retrieve_${createList ? 'list' : 'single'}${returnVkResult ? '' : '_unchecked'}`;
                const lambdaArgs = createList ? '|count, ptr|' : '|ptr|';
                const callback = `|${instanceVarName} : &mut ${returnedTypeName}| { ${setAdditionalFields.join(' ')} }`;
                statements.push(`${vkWrapperCallName}(\n    ${lambdaArgs} ${cMethod.name}(${argsForCMethod.join(', ')}),\n    ${callback}\n)`);
            } else {
                throw new Error(`method ${cMethod.name} does not seem to retrieve anything`);
            }
        } else {
            const [targetTypeName, targetMethodName] = target.split('::');

            buildType(targetTypeName);

            const rustTargetMethodName = toSnakeCase(targetMethodName);
            const targetMethod = DEFINITIONS[targetTypeName].methods.find(m => m.name === rustTargetMethodName);
            const targetCallArgs = [];

            targetMethod.args.forEach(({type, name}) => {
                if (type === `&${wrappedTypeName}`) {
                    targetCallArgs.push('self');
                } else {
                    targetCallArgs.push(name);
                    methodArguments.push({name, type});
                }
            });

            returnType = targetMethod.returnType
            statements.push(`${targetTypeName}::${rustTargetMethodName}(${targetCallArgs.join(', ')})`);
        }

        if (option === 'store') {
            const doesReturnResult = returnType.startsWith('Result');
            const unwrappedReturnResult = doesReturnResult ? returnType.substring(returnType.indexOf('<') + 1, returnType.indexOf(',')) : returnType;
            const fieldName = `_${toSnakeCase(unwrappedReturnResult.substring(2))}_list`;

            uses.add('std::mem::ManuallyDrop');
            mutSelf = true;
            fields.push({name: fieldName, type: `ManuallyDrop<Vec<${unwrappedReturnResult}>>`});
            returnType = doesReturnResult ? `Result<(), VkResult>` : null;

            statements[statements.length - 1] = `let result = ${statements[statements.length - 1]};`;

            if (doesReturnResult) {
                statements.push(`match result`, [
                    `Ok(value) => `, [
                        `self.${fieldName}.push(value);`,
                        `Ok(())`
                    ],
                    `Err(error) => Err(error)`
                ]);
            } else {
                statements.push(`self.${fieldName}.push(value)`);
            }

            wrappedDropMethod.body.unshift(`ManuallyDrop::drop(&mut self.${fieldName});`);
        }

        if (!static) {
            methodArguments.unshift({type: mutSelf ? '&mut self': '&self'});
        }
        
        const method = {
            name: toSnakeCase(methodName),
            args: methodArguments,
            returnType: returnType,
            body: statements,
            public: true,
            unsafe: unsafe
        };

        methods.push(method);
    });

    fromWrappedToRaw = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${wrappedTypeName}`}],
        returnType: 'Self',
        body: [`value._handle`]
    };

    fromRawToWrapped = {
        name: 'vk_from',
        args: [{name: 'value', type: `&${rawTypeName}`}],
        returnType: 'Self',
        body: ['Self', fields.map(({name, type}) => {
            const isVec = type.includes('Vec');
            const isMainHandle = name === '_handle';
            const initValue = isMainHandle ? '*value' : (isVec ? 'ManuallyDrop::new(Vec::new())' : 'VK_NULL_HANDLE');

            return `${name}: ${initValue},`
        })]
    }

    // methods.forEach(method => console.log(methodToStr(method)));

    return { rawTypeName, wrappedTypeName, uses, rawDefinition, wrappedDefinition, methods, rawDropMethod, wrappedDropMethod, externFunctions, fromRawToWrapped, fromWrappedToRaw };
}


function formatCTypeName(typeName) {
    return typeName
        .replace('FlagBits', 'Flags')
        .replace(/GLFW[a-z]/, str => `Glfw${str[4].toUpperCase()}`)
        .replace(/^PFN_v/, 'V');
}

function toRawTypeName(typeName) {
    return PRIMITIVE_TYPES[typeName] || `Raw${formatCTypeName(typeName)}`;
}

function toWrappedTypeName(typeName) {
    if (typeName === 'VkBool32') {
        return 'bool';
    }

    return PRIMITIVE_TYPES[typeName] || formatCTypeName(typeName);
}

function blockToStr(block, indent) {
    const spaces = indentToSpaces(indent);
    let result;

    if (typeof block === 'string') {
        result = `\n${block.split('\n').map(line => `${spaces}${line}`).join('\n')}`;
    } else {
        result = ` {${block.map(b => blockToStr(b, inc(indent))).join('')}\n${spaces}}`
    }

    if (indent === undefined) {
        result = result.substring(2, result.length - 2).trim();
    }

    return result;
}

function inc(value) {
    return value === undefined ? 0 : value + 1;
}

function indentToSpaces(indent) {
    if (!indent) return '';

    return new Array(indent).fill('    ').join('');
}

function toSnakeCase(str) {
    return str
        .replace(/[A-Z0-9]+/g, str => str.charAt(0) + str.substring(1).toLowerCase())
        .split(/(?=[A-Z])/).join('_').toLowerCase()
        .replace(/_+/g, '_');
}

function toPascalCase(str) {
    return str.split('_')
    .map(word => word.charAt(0).toUpperCase() + word.substring(1).toLowerCase()).join('')
    .replace(/\d[a-z](?=\d)/g, str => str[0] + str[1].toUpperCase());
}

function flatten(array) {
    const result = [];

    for (let i = 0; i < array.length; ++i) {
        const elt = array[i];

        if (Array.isArray(elt)) {
            elt.forEach(x => result.push(x));
        } else {
            result.push(elt);
        }
    }

    return result;
}