const fs = require("fs");
// https://www.convertjson.com/html-table-to-json.htm
// https://www.nesdev.org/wiki/CPU_unofficial_opcodes

// !! includes unofficial opcodes !!
const json = require("./files/original.json");
const official = require("./files/official.json");
const assert = require("assert");

// http://www.6502.org/tutorials/6502opcodes.html
const addr_modes = {
    null: { name: 'IMPL', examples: ['{OP}'] }, // NOP | BRK
    '#i': { name: 'IMM', examples: ['{OP} #$BB'] }, // LDA #$BB

    '(a)': { name: 'IND', examples: ['{OP} ($LLHH)'] }, // JMP ($LLHH)
    '(d,x)': { name: 'INDX', examples: ['{OP} ($BB,X)'] }, // LDA ($BB,X)
    '(d),y': { name: 'INDY', examples: ['{OP} ($BB),Y'] }, // LDA ($BB),Y

    'd': { name: 'ZP', examples: ['{OP} $BB'] }, // DEC $BB
    'd,x': { name: 'ZPX', examples: ['{OP} $BB,X'] },  // DEC $BB,X
    'd,y': { name: 'ZPY', examples: ['{OP} $BB,Y'] }, // DEC $BB,Y

    // note: all branches are relative mode & two bytes length
    // BXX label
    '*+d': { name: 'REL', examples: ['{OP} $BB', '{OP} label'] },  // BPL $BB | BPL label

    'a': { name: 'ABS', examples: ['{OP} $LLHH'] }, // ADC $4400 | JMP $5597
    'a,x': { name: 'ABSX', examples: ['{OP} $LLHH,X'] }, // ADC $LLHH,X
    'a,y': { name: 'ABSY', examples: ['{OP} $LLHH,Y'] }  // ADC $LLHH,Y
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
    // (instr, mode) => [(opcode, examples)*]
    const row_fmt = '((Instr::{INSTR}, AdrMode::{ADRMODE}), vec![{OPCODES}])';
    const json = instrToOpcodes();
    const rows = [];

    // group into instr, adrmode pairs
    const key = (instr, mode) => instr + '::' + mode.name;
    const get = (k) => k.split('::');
    const group = {};
    for (const [instr, json_modes] of Object.entries(json)) {
        for (const [opcode, mode] of Object.entries(json_modes)) {
            const h = key(instr, mode);
            if (!group[h]) {
                group[h] = [];
            }
            group[h].push({opcode, mode});
        }
    }

    for (const [k, op_mode] of Object.entries(group)) {
        const [instr, addr] = get(k);
        const opcodes = op_mode.map((o) => {
            const {opcode, mode} = o;
            const examples =  mode.examples.map((e) => `"${e}".to_string()`).join(', ');
            const fmt = 'Opcode::new({OPCODE}, {OFFICIAL}, vec![{EXAMPLES}])';
            let off_value = official[instr + '+' + mode.name];
            return inject(fmt, {
                'OPCODE': opcode,
                'OFFICIAL' : off_value ? (off_value.opcode == opcode) : false,
                'EXAMPLES': examples
            });
        });
        const start = opcodes.length > 1 ? '\n\t' : '';
        const end = opcodes.length > 1 ? '\n' : '';
        const curr_row = inject(row_fmt, {
            'INSTR': instr,
            'ADRMODE': addr,
            'OPCODES': start + opcodes.join(',\n\t') + end
        });
        rows.push(curr_row + ',');
    }

    return rows.join('\n');
}

fs.writeFileSync("./files/unofficial_opcode_to_instr.json", JSON.stringify(opcodeToInstruction(), null, 2));
fs.writeFileSync("./files/unofficial_instr_to_opcodes.json", JSON.stringify(instrToOpcodes(), null, 2));

fs.writeFileSync("./files/opcodes.rs", instrToRustMap());