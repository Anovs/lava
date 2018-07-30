const {
    toPascalCase,
    getRawVkTypeName,
    getWrappedVkTypeName,
    getConstVkValueName,
    findEnumPrefix,
    toSnakeCase,
    getFullWrappedType,
    getFullRawType,
    blockToString,
    isCount,
    areCountAndArray,
    isPlural,
    cToRustVarName,
    argToString,
    getFieldInformation
} = require('./utils');

function generateVkBitFlagsDefinition(cDef) {
    cDef.rawTypeName = getRawVkTypeName(cDef.name);
    cDef.wrappedTypeName = getWrappedVkTypeName(cDef.name);

    const prefix = findEnumPrefix(cDef.name.replace('Flags', ''));
    const extSuffix = `_${cDef.extension.toUpperCase()}`;

    for (let field of cDef.fields) {
        const name = field.name.replace('_BIT', '');
        const suffix = name.endsWith(extSuffix) ? extSuffix : '';
        const strippedName = name.substring(prefix.length + 1, name.length - suffix.length);
        
        field.rustName = formatBitFlagsFieldName(strippedName);
    }

    return [
        genUses(),
        genRawType(cDef),
        genWrappedType(cDef),
        genImplVkRawType(cDef),
        genImplVkWrappedType(cDef),
        genImplVkDefault(cDef)
    ];
}

function genUses() {
    return [
        `utils::vk_type::*`
    ].map(str => `use ${str};`);
}

function genRawType(def) {
    return `pub type ${def.rawTypeName} = u32;`;
}

function genWrappedType(def) {
    return [
        `#[derive(Debug, Clone, Copy)]`,
        `pub struct ${def.wrappedTypeName}`,
        def.fields.map(field => `${field.rustName}: bool,`)
    ];
}

function genImplVkRawType(def) {
    return [
        `impl VkRawType<${def.wrappedTypeName}> for ${def.rawTypeName}`, [
            `fn vk_to_wrapped(src: &${def.rawTypeName}) -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.rustName}: (src & ${field.value}) != 0,`)
            ]
        ]
    ];
}

function genImplVkWrappedType(def) {
    return [
        `impl VkWrappedType<${def.rawTypeName}> for ${def.wrappedTypeName}`, [
            `fn vk_to_raw(src: &${def.wrappedTypeName}, dst: &mut ${def.rawTypeName})`, [
                `*dst = 0;`,
                ...def.fields.map(field => `if src.${field.rustName} { *dst |= ${field.value}; }`),
            ]
        ]
    ];
}

function genImplVkDefault(def) {
    return [
        `impl VkDefault for ${def.wrappedTypeName}`, [
            `fn vk_default() -> ${def.wrappedTypeName}`, [
                def.wrappedTypeName,
                def.fields.map(field => `${field.rustName}: false,`)
            ]
        ]
    ];
}

function formatBitFlagsFieldName(name) {
    return toSnakeCase(name)
        .replace(/^(\d)/, '_$1');
}

module.exports = {
    generateVkBitFlagsDefinition
};