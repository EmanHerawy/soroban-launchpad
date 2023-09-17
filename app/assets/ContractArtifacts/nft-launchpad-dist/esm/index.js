import { xdr } from 'soroban-client';
import { Buffer } from "buffer";
import { scValStrToJs, addressToScVal } from './convert.js';
import { invoke } from './invoke.js';
export * from './constants.js';
export * from './server.js';
export * from './invoke.js';
;
;
export class Ok {
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
export class Err {
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
if (typeof window !== 'undefined') {
    //@ts-ignore Buffer exists
    window.Buffer = window.Buffer || Buffer;
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
export async function deploy({ deployer, salt, admin, base_uri, name, symbol }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'deploy',
        args: [((i) => addressToScVal(i))(deployer),
            ((i) => xdr.ScVal.scvBytes(i))(salt),
            ((i) => addressToScVal(i))(admin),
            ((i) => xdr.ScVal.scvString(i))(base_uri),
            ((i) => xdr.ScVal.scvString(i))(name),
            ((i) => xdr.ScVal.scvString(i))(symbol)],
    };
    try {
        // @ts-ignore Type does exist
        const response = await invoke(invokeArgs);
        return new Ok(scValStrToJs(response.xdr));
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
export async function is_paused({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'is_paused',
    };
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValStrToJs(response.xdr);
}
export async function get_admin({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'get_admin',
    };
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return scValStrToJs(response.xdr);
}
export async function set_admin({ admin }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'set_admin',
        args: [((i) => addressToScVal(i))(admin)],
    };
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return;
}
export async function pause({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'pause',
    };
    try {
        // @ts-ignore Type does exist
        const response = await invoke(invokeArgs);
        return new Ok(scValStrToJs(response.xdr));
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
export async function unpause({ signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'unpause',
    };
    try {
        // @ts-ignore Type does exist
        const response = await invoke(invokeArgs);
        return new Ok(scValStrToJs(response.xdr));
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
export async function init({ admin }, { signAndSend, fee } = { signAndSend: false, fee: 100 }) {
    let invokeArgs = {
        signAndSend,
        fee,
        method: 'init',
        args: [((i) => addressToScVal(i))(admin)],
    };
    // @ts-ignore Type does exist
    const response = await invoke(invokeArgs);
    return;
}
