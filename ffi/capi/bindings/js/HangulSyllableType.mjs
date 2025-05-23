// generated by diplomat-tool
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/**
 * See the [Rust documentation for `HangulSyllableType`](https://docs.rs/icu/latest/icu/properties/props/struct.HangulSyllableType.html) for more information.
 */


export class HangulSyllableType {
    #value = undefined;

    static #values = new Map([
        ["NotApplicable", 0],
        ["LeadingJamo", 1],
        ["VowelJamo", 2],
        ["TrailingJamo", 3],
        ["LeadingVowelSyllable", 4],
        ["LeadingVowelTrailingSyllable", 5]
    ]);

    static getAllEntries() {
        return HangulSyllableType.#values.entries();
    }

    #internalConstructor(value) {
        if (arguments.length > 1 && arguments[0] === diplomatRuntime.internalConstructor) {
            // We pass in two internalConstructor arguments to create *new*
            // instances of this type, otherwise the enums are treated as singletons.
            if (arguments[1] === diplomatRuntime.internalConstructor ) {
                this.#value = arguments[2];
                return this;
            }
            return HangulSyllableType.#objectValues[arguments[1]];
        }

        if (value instanceof HangulSyllableType) {
            return value;
        }

        let intVal = HangulSyllableType.#values.get(value);

        // Nullish check, checks for null or undefined
        if (intVal != null) {
            return HangulSyllableType.#objectValues[intVal];
        }

        throw TypeError(value + " is not a HangulSyllableType and does not correspond to any of its enumerator values.");
    }

    static fromValue(value) {
        return new HangulSyllableType(value);
    }

    get value(){
        return [...HangulSyllableType.#values.keys()][this.#value];
    }

    get ffiValue(){
        return this.#value;
    }
    static #objectValues = [
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 0),
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 1),
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 2),
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 3),
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 4),
        new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.internalConstructor, 5),
    ];

    static NotApplicable = HangulSyllableType.#objectValues[0];
    static LeadingJamo = HangulSyllableType.#objectValues[1];
    static VowelJamo = HangulSyllableType.#objectValues[2];
    static TrailingJamo = HangulSyllableType.#objectValues[3];
    static LeadingVowelSyllable = HangulSyllableType.#objectValues[4];
    static LeadingVowelTrailingSyllable = HangulSyllableType.#objectValues[5];


    /**
     * See the [Rust documentation for `for_char`](https://docs.rs/icu/latest/icu/properties/props/trait.EnumeratedProperty.html#tymethod.for_char) for more information.
     */
    static forChar(ch) {

        const result = wasm.icu4x_HangulSyllableType_for_char_mv1(ch);

        try {
            return new HangulSyllableType(diplomatRuntime.internalConstructor, result);
        }

        finally {
        }
    }

    /**
     * Convert to an integer value usable with ICU4C and CodePointMapData
     *
     * See the [Rust documentation for `to_icu4c_value`](https://docs.rs/icu/latest/icu/properties/props/struct.HangulSyllableType.html#method.to_icu4c_value) for more information.
     */
    toIntegerValue() {

        const result = wasm.icu4x_HangulSyllableType_to_integer_value_mv1(this.ffiValue);

        try {
            return result;
        }

        finally {
        }
    }

    /**
     * Convert from an integer value from ICU4C or CodePointMapData
     *
     * See the [Rust documentation for `from_icu4c_value`](https://docs.rs/icu/latest/icu/properties/props/struct.HangulSyllableType.html#method.from_icu4c_value) for more information.
     */
    static fromIntegerValue(other) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_HangulSyllableType_from_integer_value_mv1(diplomatReceive.buffer, other);

        try {
            if (!diplomatReceive.resultFlag) {
                return null;
            }
            return new HangulSyllableType(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
        }

        finally {
            diplomatReceive.free();
        }
    }

    constructor(value) {
        return this.#internalConstructor(...arguments)
    }
}