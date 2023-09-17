import { Buffer } from "buffer";
export * from './constants.js';
export * from './server.js';
export * from './invoke.js';
export type u32 = number;
export type i32 = number;
export type u64 = bigint;
export type i64 = bigint;
export type u128 = bigint;
export type i128 = bigint;
export type u256 = bigint;
export type i256 = bigint;
export type Address = string;
export type Option<T> = T | undefined;
export type Typepoint = bigint;
export type Duration = bigint;
export interface Error_ {
    message: string;
}
export interface Result<T, E = Error_> {
    unwrap(): T;
    unwrapErr(): E;
    isOk(): boolean;
    isErr(): boolean;
}
export declare class Ok<T> implements Result<T> {
    readonly value: T;
    constructor(value: T);
    unwrapErr(): Error_;
    unwrap(): T;
    isOk(): boolean;
    isErr(): boolean;
}
export declare class Err<T> implements Result<T> {
    readonly error: Error_;
    constructor(error: Error_);
    unwrapErr(): Error_;
    unwrap(): never;
    isOk(): boolean;
    isErr(): boolean;
}
export declare function deploy({ deployer, salt, admin, base_uri, name, symbol }: {
    deployer: Address;
    salt: Buffer;
    admin: Address;
    base_uri: string;
    name: string;
    symbol: string;
}, { signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<Result<Address>>;
export declare function is_paused({ signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<boolean>;
export declare function get_admin({ signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<Address>;
export declare function set_admin({ admin }: {
    admin: Address;
}, { signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<void>;
export declare function pause({ signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<Result<boolean>>;
export declare function unpause({ signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<Result<boolean>>;
export declare function init({ admin }: {
    admin: Address;
}, { signAndSend, fee }?: {
    signAndSend?: boolean;
    fee?: number;
}): Promise<void>;
