const fs = require("fs");
// https://www.convertjson.com/html-table-to-json.htm
// https://www.nesdev.org/wiki/CPU_unofficial_opcodes

// !! includes unofficial opcodes !!
const json = require("./files/original.json");
const assert = require("assert");

// http://www.6502.org/tutorials/6502opcodes.html
const addr_modes = {
    null: { name: 'IMPL', examples: ['{OP}'] }, // NOP | BRK
    '#i': { name: 'IMM', examples: ['{OP} #$44'] }, // LDA #$44

    '(a)': { name: 'IND', examples: ['{OP} ($5597)'] }, // JMP ($5597)
    '(d,x)': { name: 'INDX', examples: ['{OP} ($44,X)'] }, // LDA ($44,X)
    '(d),y': { name: 'INDY', examples: ['{OP} ($44),Y'] }, // LDA ($44),Y

    'd': { name: 'ZP', examples: ['{OP} $44'] }, // DEC $44
    'd,x': { name: 'ZPX', examples: ['{OP} $44,X'] },  // DEC $44,X
    'd,y': { name: 'ZPY', examples: ['{OP} $44,Y'] }, // DEC $44,Y

    // note: all branches are relative mode & two bytes length
    // BXX label
    '*+d': { name: 'REL', examples: ['{OP} $10', '{OP} label'] },  // BPL $10 | BPL label

    'a': { name: 'ABS', examples: ['{OP} $4400'] }, // ADC $4400 | JMP $5597
    'a,x': { name: 'ABSX', examples: ['{OP} $4400,X'] }, // ADC $4400,X
    'a,y': { name: 'ABSY', examples: ['{OP} $4400,Y'] }  // ADC $4400,Y
};


// {a}, map = {a: 'a'} = a
function inject(str, map) {
    const re = /\{([_\w\s]+)?\}/g;
    const repl = str.replace(re, (_, q) => map[q.trim()]);
    return repl;
}

function deepCopy(obj) {
    return JSON.parse(JSON.stringify(obj));
}

function opcodeToInstruction() {
    const ans = {};
    const pad = (s) => (s.length == 1 ? "0" : "") + s;
    for (const row of json) {
        const opcode_a = row["FIELD1"];
        for (const [opcode_b, raw_instr] of Object.entries(row)) {
            if (opcode_b == "FIELD1") {
                continue;
            }
            const [instr, mode] = raw_instr.split("\n");
            const h = opcode_a + opcode_b;
            ans[h] = {
                hex: '0x' + pad((
                    parseInt("0x" + opcode_a) 
                    + parseInt("0x" + opcode_b.substring(1))
                ).toString(16)).toUpperCase(),
                instr,
            };
            const notation = mode ?? null;
            const ptr_details = addr_modes[notation];
            assert(ptr_details);
            const details = deepCopy(ptr_details);
            details.examples = details.examples.map((s) => inject(s, { OP: instr }))
            ans[h].mode = {
                notation, ...ans[h].mode, ...details
            }; 
        }
    }
    return ans;
}

function instrToOpcodes() {
    const json = opcodeToInstruction();
    const ans = {};
    for (const v of Object.values(json)) {
        if (!ans[v.instr]) {
            ans[v.instr] = {};
        }
        ans[v.instr][v.hex] = {...v.mode};
    }
    return ans;
}

// example rust code generation
function instrToRustMap() {
    const format = '(Opcode::new(Instr::{INSTR}, AdrMode::{MODE}, vec![{EXAMPLES}]), {OPCODE})';
    const json = instrToOpcodes();
    const rows = [];
    for (const [instr, json_modes] of Object.entries(json)) {
        for (const [opcode, mode] of Object.entries(json_modes)) {
            const {_, name, examples} = mode;
            const variables = {
                'INSTR': instr, 
                'MODE': name, 
                'EXAMPLES': examples.map((e) => `"${e}".to_string()`).join(', '), 
                'OPCODE': opcode
            };
            rows.push(inject(format, variables) + ',');
        }
        rows.push('');
    }
    return rows.join('\n');
}

fs.writeFileSync("./files/opcode_to_instr.json", JSON.stringify(opcodeToInstruction(), null, 2));
fs.writeFileSync("./files/instr_to_opcodes.json", JSON.stringify(instrToOpcodes(), null, 2));

fs.writeFileSync("./files/opcodes.rs", instrToRustMap());