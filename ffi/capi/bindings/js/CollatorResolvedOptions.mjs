// generated by diplomat-tool
import { CollatorAlternateHandling } from "./CollatorAlternateHandling.mjs"
import { CollatorCaseFirst } from "./CollatorCaseFirst.mjs"
import { CollatorCaseLevel } from "./CollatorCaseLevel.mjs"
import { CollatorMaxVariable } from "./CollatorMaxVariable.mjs"
import { CollatorNumericOrdering } from "./CollatorNumericOrdering.mjs"
import { CollatorStrength } from "./CollatorStrength.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";



/**
 * See the [Rust documentation for `ResolvedCollatorOptions`](https://docs.rs/icu/2.0.0/icu/collator/options/struct.ResolvedCollatorOptions.html) for more information.
 */
export class CollatorResolvedOptions {
    #strength;
    get strength() {
        return this.#strength;
    }
    #alternateHandling;
    get alternateHandling() {
        return this.#alternateHandling;
    }
    #caseFirst;
    get caseFirst() {
        return this.#caseFirst;
    }
    #maxVariable;
    get maxVariable() {
        return this.#maxVariable;
    }
    #caseLevel;
    get caseLevel() {
        return this.#caseLevel;
    }
    #numeric;
    get numeric() {
        return this.#numeric;
    }
    #internalConstructor(structObj, internalConstructor) {
        if (typeof structObj !== "object") {
            throw new Error("CollatorResolvedOptions's constructor takes an object of CollatorResolvedOptions's fields.");
        }

        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CollatorResolvedOptions is an out struct and can only be created internally.");
        }
        if ("strength" in structObj) {
            this.#strength = structObj.strength;
        } else {
            throw new Error("Missing required field strength.");
        }

        if ("alternateHandling" in structObj) {
            this.#alternateHandling = structObj.alternateHandling;
        } else {
            throw new Error("Missing required field alternateHandling.");
        }

        if ("caseFirst" in structObj) {
            this.#caseFirst = structObj.caseFirst;
        } else {
            throw new Error("Missing required field caseFirst.");
        }

        if ("maxVariable" in structObj) {
            this.#maxVariable = structObj.maxVariable;
        } else {
            throw new Error("Missing required field maxVariable.");
        }

        if ("caseLevel" in structObj) {
            this.#caseLevel = structObj.caseLevel;
        } else {
            throw new Error("Missing required field caseLevel.");
        }

        if ("numeric" in structObj) {
            this.#numeric = structObj.numeric;
        } else {
            throw new Error("Missing required field numeric.");
        }

        return this;
    }

    // Return this struct in FFI function friendly format.
    // Returns an array that can be expanded with spread syntax (...)
    _intoFFI(
        functionCleanupArena,
        appendArrayMap
    ) {
        let buffer = diplomatRuntime.DiplomatBuf.struct(wasm, 24, 4);

        this._writeToArrayBuffer(wasm.memory.buffer, buffer.ptr, functionCleanupArena, appendArrayMap);

        functionCleanupArena.alloc(buffer);

        return buffer.ptr;
    }

    static _fromSuppliedValue(internalConstructor, obj) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("_fromSuppliedValue cannot be called externally.");
        }

        if (obj instanceof CollatorResolvedOptions) {
            return obj;
        }

        return CollatorResolvedOptions.fromFields(obj);
    }

    _writeToArrayBuffer(
        arrayBuffer,
        offset,
        functionCleanupArena,
        appendArrayMap
    ) {
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 0, this.#strength.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 4, this.#alternateHandling.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 8, this.#caseFirst.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 12, this.#maxVariable.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 16, this.#caseLevel.ffiValue, Int32Array);
        diplomatRuntime.writeToArrayBuffer(arrayBuffer, offset + 20, this.#numeric.ffiValue, Int32Array);
    }

    // This struct contains borrowed fields, so this takes in a list of
    // "edges" corresponding to where each lifetime's data may have been borrowed from
    // and passes it down to individual fields containing the borrow.
    // This method does not attempt to handle any dependencies between lifetimes, the caller
    // should handle this when constructing edge arrays.
    static _fromFFI(internalConstructor, ptr) {
        if (internalConstructor !== diplomatRuntime.internalConstructor) {
            throw new Error("CollatorResolvedOptions._fromFFI is not meant to be called externally. Please use the default constructor.");
        }
        let structObj = {};
        const strengthDeref = diplomatRuntime.enumDiscriminant(wasm, ptr);
        structObj.strength = new CollatorStrength(diplomatRuntime.internalConstructor, strengthDeref);
        const alternateHandlingDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 4);
        structObj.alternateHandling = new CollatorAlternateHandling(diplomatRuntime.internalConstructor, alternateHandlingDeref);
        const caseFirstDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 8);
        structObj.caseFirst = new CollatorCaseFirst(diplomatRuntime.internalConstructor, caseFirstDeref);
        const maxVariableDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 12);
        structObj.maxVariable = new CollatorMaxVariable(diplomatRuntime.internalConstructor, maxVariableDeref);
        const caseLevelDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 16);
        structObj.caseLevel = new CollatorCaseLevel(diplomatRuntime.internalConstructor, caseLevelDeref);
        const numericDeref = diplomatRuntime.enumDiscriminant(wasm, ptr + 20);
        structObj.numeric = new CollatorNumericOrdering(diplomatRuntime.internalConstructor, numericDeref);

        return new CollatorResolvedOptions(structObj, internalConstructor);
    }


    constructor(structObj, internalConstructor) {
        return this.#internalConstructor(...arguments)
    }
}