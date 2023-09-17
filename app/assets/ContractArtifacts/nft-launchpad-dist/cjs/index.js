"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __exportStar = (this && this.__exportStar) || function(m, exports) {
    for (var p in m) if (p !== "default" && !Object.prototype.hasOwnProperty.call(exports, p)) __createBinding(exports, m, p);
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.init = exports.unpause = exports.pause = exports.set_admin = exports.get_admin = exports.is_paused = exports.deploy = exports.Err = exports.Ok = void 0;
const soroban_client_1 = require("soroban-client");
const buffer_1 = require("buffer");
const convert_js_1 = require("./convert.js");
const invoke_js_1 = require("./invoke.js");
__exportStar(require("./constants.js"), exports);
__exportStar(require("./server.js"), exports);
__exportStar(require("./invoke.js"), exports);
;
;
class Ok {
    value;
    constructor(value) {
        this.value = value;
    }
    unwrapErr() {
        throw new Error('No error');
    }
    unwrap() {
        return this.value;
    }
    isOk() {
        return true;
    }
    isErr() {
        return !this.isOk();
    }
}
exports.Ok = Ok;
class Err {
    error;
    constructor(error) {
        this.error = error;
    }
    unwrapErr() {
        return this.error;
    }
    unwrap() {
        throw new Error(this.error.message);
    }
    isOk() {
        return false;
    }
    isErr() {
        return !this.isOk();
    }
}
exports.Err = Err;
if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || buffer_1.Buffer;
}
const regex = /ContractError\((\d+)\)/;
function getError(err) {
    const match = err.match(regex);
    if (!match) {
        return undefined;
    }
    if (Errors == undefined) {
        return undefined;
    }
    // @ts-ignore
    let i = parseInt(match[1], 10);
    if (i < Errors.length) {
        return new Err(Errors[i]);
    }
    return undefined;
}
const Errors = [
    { message: "" },
    { message: "" },
    { message: "" }
];
async function deploy({ deployer, salt, admin, base_uri, name, symbol }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'deploy',
        args: [((i) => (0, convert_js_1.addressToScVal)(i))(deployer),
            ((i) => soroban_client_1.xdr.ScVal.scvBytes(i))(salt),
            ((i) => (0, convert_js_1.addressToScVal)(i))(admin),
            ((i) => soroban_client_1.xdr.ScVal.scvString(i))(base_uri),
            ((i) => soroban_client_1.xdr.ScVal.scvString(i))(name),
            ((i) => soroban_client_1.xdr.ScVal.scvString(i))(symbol)],
    };
    try {
        // @ts-ignore Type does exist
        const response = await (0, invoke_js_1.invoke)(invokeArgs);
        return new Ok((0, convert_js_1.scValStrToJs)(response.xdr));
    }
    catch (e) {
        //@ts-ignore
        let err = getError(e.message);
        if (err) {
            return err;
        }
        else {
            throw e;
        }
    }
}
exports.deploy = deploy;
async function is_paused({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'is_paused',
    };
    // @ts-ignore Type does exist
    const response = await (0, invoke_js_1.invoke)(invokeArgs);
    return (0, convert_js_1.scValStrToJs)(response.xdr);
}
exports.is_paused = is_paused;
async function get_admin({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'get_admin',
    };
    // @ts-ignore Type does exist
    const response = await (0, invoke_js_1.invoke)(invokeArgs);
    return (0, convert_js_1.scValStrToJs)(response.xdr);
}
exports.get_admin = get_admin;
async function set_admin({ admin }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'set_admin',
        args: [((i) => (0, convert_js_1.addressToScVal)(i))(admin)],
    };
    // @ts-ignore Type does exist
    const response = await (0, invoke_js_1.invoke)(invokeArgs);
    return;
}
exports.set_admin = set_admin;
async function pause({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'pause',
    };
    try {
        // @ts-ignore Type does exist
        const response = await (0, invoke_js_1.invoke)(invokeArgs);
        return new Ok((0, convert_js_1.scValStrToJs)(response.xdr));
    }
    catch (e) {
        //@ts-ignore
        let err = getError(e.message);
        if (err) {
            return err;
        }
        else {
            throw e;
        }
    }
}
exports.pause = pause;
async function unpause({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'unpause',
    };
    try {
        // @ts-ignore Type does exist
        const response = await (0, invoke_js_1.invoke)(invokeArgs);
        return new Ok((0, convert_js_1.scValStrToJs)(response.xdr));
    }
    catch (e) {
        //@ts-ignore
        let err = getError(e.message);
        if (err) {
            return err;
        }
        else {
            throw e;
        }
    }
}
exports.unpause = unpause;
async function init({ admin }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'init',
        args: [((i) => (0, convert_js_1.addressToScVal)(i))(admin)],
    };
    // @ts-ignore Type does exist
    const response = await (0, invoke_js_1.invoke)(invokeArgs);
    return;
}
exports.init = init;
