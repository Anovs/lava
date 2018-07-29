const { getEnumByName, getBitFlagsByName, getStructByName, getHandleByName, isHandle } = require('./vulkan_header');

const PRIMITIVE_TYPES = {
    uint64_t: 'u64',
    uint32_t: 'u32',
    uint16_t: 'u16',
    uint8_t: 'u8',
    int64_t: 'i64',
    int32_t: 'i32',
    int16_t: 'i16',
    int8_t: 'i8',
    char: 'c_char',
    float: 'f32',
    double: 'f64',
    size_t: 'usize',
    ssize_t: 'isize',
    void: 'c_void',
    VkAllocationCallbacks: 'c_void',
    VkDeviceSize: 'u64'
};

const INDENT = '    ';

Array.prototype.last = function() {
    return this[this.length - 1];
};

Array.prototype.beforeLast = function() {
    return this[this.length - 2];
};

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

function toUpperCase(str) {
    return str
        .replace(/[a-z][A-Z]/g, str => `${str[0]}_${str[1]}`)
        .toUpperCase();
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

function blockToString(block) {
    if (typeof block === 'string') {
        return block;
    } else {
        return _blocksToString(block).replace(/\n {/g, '\n{');
    }
}

function _blocksToString(blocks) {
    let result = '';

    let isFirst = true;

    for (let block of blocks) {
        if (typeof block === 'string') {
            if (!isFirst) {
                result += '\n';
            }
            result += block;
        } else if (isFirst) {
            result += _blocksToString(block);
        } else {
            result += ' {\n';
            result += _blocksToString(block).split('\n').map(str => INDENT + str).join('\n');
            result += '\n}';
        }

        isFirst = false;
    }

    return result.replace(/\n;/g, ';\n');
}

function isPlural(arg) {
    return arg && !arg.typeName.endsWith('s') && arg.name.endsWith('s');
}

function cToRustVarName(name) {
    name = name.replace(/^(p{1,2})[A-Z]/, str => str[str.length - 1]);

    return toSnakeCase(name);
}

function argToString(arg) {
    return arg.name ? `${arg.name}: ${arg.type}` : arg.type;
}

function doesFieldRepresentVersion(field) {
    return field.typeName === 'uint32_t' && /Version$/.test(field.name);
}

function doesFieldRepresentIndex(field) {
    return field.typeName === 'uint32_t' && /Index|Indices/.test(field.name);
}

function createStaticArray(typeName, arraySize, varName, functionName) {
    return `unsafe { let mut dst_array : [${typeName}, ${arraySize}] = mem::uninitialized(); ${functionName}(&${varName}, &mut dst_array); dst_array }`;
}

function getRawVkTypeName(cTypeName) {
    return `Raw${cTypeName}`;
}

function getWrappedVkTypeName(cTypeName) {
    return cTypeName;
}

function getFieldRawTypeName(field) {
    return PRIMITIVE_TYPES[field.typeName] || getRawVkTypeName(field.typeName);
}

function getFieldWrappedTypeName(field) {
    if (doesFieldRepresentVersion(field)) {
        return `VkVersion`;
    } else if (doesFieldRepresentIndex(field)) {
        return `usize`;
    }

    return PRIMITIVE_TYPES[field.typeName] || getWrappedVkTypeName(field.typeName);
}

class LifetimeSet {
    constructor() {
        this._nb = 0;
        this._types = {};
    }

    add(outlives) {
        const letter = "'" + String.fromCharCode('a'.charCodeAt(0) + this._nb);
        this._types[letter] = outlives;
        this._nb += 1;

        return letter;
    }

    getTypes() {
        return Object.keys(this._types);
    }

    getDefinitions() {
        return Object.values(this._types);
    }
}

function getStaticVkValueName(wrappedTypeName) {
    return `STATIC_${toUpperCase(wrappedTypeName)}`;
}


function getFieldsInformation(fields) {
    const infos = [];

    for (let field of fields) {
        const rawTypeName = getFieldRawTypeName(field);
        const wrappedTypeName = getFieldWrappedTypeName(field);
        const vkStaticValue = getStaticVkValueName(wrappedTypeName);

        const arraySize = field.arraySize;
        const isCount = !!field.countFor.length;
        const isPointerArray = field.isPointer && field.countField;
        const isPointerValue = field.isPointer && !isPointerArray;
        const isStaticArray = !field.isPointer && !!arraySize;
        const isPrimitiveType = rawTypeName === wrappedTypeName;
        const isHandleType = !isPrimitiveType && isHandle(field.typeName);

        const varName = '[VarName]';
        const countVarName = '[CountVarName]';
        const arrayVarName = '[ArrayVarName]';

        const varNameValue = cToRustVarName(field.name);
        const countVarNameValue = isPointerArray ? cToRustVarName(field.countField) : null;
        const arrayVarNameValue = isCount ? cToRustVarName(field.countFor[0]) : null;


        let rawType = null;
        let wrappedType = null;
        let toRaw = null;
        let toWrapped = null;
        let defValue = null;

        if (field.fullType === 'const void*') {
            rawType = `*const c_void`;
            toRaw = `ptr::null()`;
        } else if (isCount) {
            rawType = `u32`;

            let lenValue = `makeVarName(${cToRustVarName(field.countFor[0])}).len()`;

            for (let i = 1; i < field.countFor.length; ++i) {
                const otherLen = `makeVarName(${cToRustVarName(field.countFor[i])}.len()`;
                lenValue = `cmp::max(${lenValue}, ${otherLen})`;
            }

            toRaw = new Function('makeVarName', `return '${lenValue} as u32'.replace(/makeVarName\\((\\w+)\\)/g, function(_, name) { return makeVarName(name); })`);
        } else if (field.fullType === 'const char* const*') {
            rawType = `VkPtr<*mut c_char>`;
            wrappedType = `&[&str]`;
            toRaw = `VkPtr::new_string_array(${varName})`;
            defValue = `&[]`;
        } else if (field.fullType === 'const char*') {
            if (field.name === 'displayName') {
                rawType = '*const c_char';
                wrappedType =`String`;
                toWrapped = `new_string(${varName})`;
            } else {
                rawType = `VkPtr<c_char>`;
                wrappedType =`&str`;
                toRaw = `VkPtr::new_string(${varName})`;
                defValue = `""`;
            }
        } else if (field.fullType === 'char' && field.arraySize) {
            rawType = `[c_char; ${arraySize}]`;
            wrappedType = `String`;
            // toRaw = createStaticArray(rawTypeName, arraySize, varName, 'string_to_byte_array');
            toWrapped = `new_string(&${varName}[0] as *const c_char)`;
            // defValue = `String::new()`;
        } else if (isPrimitiveType) {
            if (isPointerArray) {
                rawType = `*const *${rawTypeName}`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `${varName}.as_ptr()`;
                // toWrapped = `new_array(${prevVarName}, ${varName})`;
                defValue = `&[]`;
            } else if (isPointerValue) {
                rawType = `*const *${rawTypeName}`;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `${varName} as *const ${rawTypeName}`;
                // toWrapped = varName; // Should never be used
                defValue = `&0`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;
                wrappedType = `Vec<${wrappedTypeName}>`;
                // toRaw = createStaticArray(rawTypeName, arraySize, varName, 'to_array');
                toWrapped = `new_array(${countVarName}, &${varName}[0] as *const ${rawTypeName})`;
                // defValue = `Vec::new()`;
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = varName;
                toWrapped = varName;
                defValue = `0`;
            }
        } else {
            if (isPointerArray) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = `&[${wrappedTypeName}]`;
                toRaw = `VkPtr::new_vk_array(${varName})`;
                // toWrapped = `new_vk_array(${prevVarName}, ${varName})`;
                defValue = `&[]`;
            } else if (isPointerValue) {
                rawType = `VkPtr<${rawTypeName}>`;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `VkPtr::new_vk_value(${varName})`;
                // toWrapped = `${rawTypeName}::vk_to_wrapped(${varName}.as_ref().unwrap())`;
                defValue = `&${vkStaticValue}`;
            } else if (isStaticArray) {
                rawType = `[${rawTypeName}; ${arraySize}]`;
                wrappedType = `Vec<${wrappedTypeName}>`;
                // toRaw = createStaticArray(rawTypeName, arraySize, varName, 'vk_to_raw_array');
                toWrapped = `new_vk_array(${countVarName}, &${varName}[0] as *const ${rawTypeName})`;
                // defValue = `Vec::new()`;
            } else if (isHandleType) {
                rawType = rawTypeName;
                wrappedType = `&${wrappedTypeName}`;
                toRaw = `vk_to_raw_value(&${varName})`;
                // toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = `&${vkStaticValue}`;
            } else {
                rawType = rawTypeName;
                wrappedType = wrappedTypeName;
                toRaw = `vk_to_raw_value(&${varName})`;
                toWrapped = `${rawTypeName}::vk_to_wrapped(&${varName})`;
                defValue = vkStaticValue;
            }
        }

        const info = {
            typeName: field.typeName,
            extension: field.extension,
            rawType: rawType,
            wrappedType: wrappedType,
            rawTypeName: rawTypeName,
            wrappedTypeName: wrappedTypeName,
            toRaw: stringToFunction(toRaw, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            toWrapped: stringToFunction(toWrapped, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue),
            defaultValue: defValue,
            varName: varNameValue
        };

        infos.push(info);
    }

    return infos;
}

function stringToFunction(statement, varName, countVarName, arrayVarName, varNameValue, countVarNameValue, arrayVarNameValue) {
    if (!statement) {
        return null;
    }

    if (typeof statement === 'function') {
        return statement;
    }

    let body = [
        `var str = '${statement}';`,
        `str = str.replace('${varName}', makeVarName("${varNameValue}"));`
    ];

    if (countVarNameValue) {
        body.push(`str = str.replace('${countVarName}', makeVarName("${countVarNameValue}"));`);
    }

    if (arrayVarNameValue) {
        body.push(`str = str.replace('${arrayVarName}', makeVarName("${arrayVarNameValue}"));`);
    }

    body.push(`return str;`);

    return new Function('makeVarName', body.join('\n'));
}

module.exports = {
    toSnakeCase,
    toPascalCase,
    toUpperCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    blockToString,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldsInformation,
    findEnumPrefix,
    getStaticVkValueName,
    LifetimeSet
};