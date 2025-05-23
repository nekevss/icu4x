// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { Locale } from "./Locale.mjs"
import { PluralCategories } from "./PluralCategories.mjs"
import { PluralCategory } from "./PluralCategory.mjs"
import { PluralOperands } from "./PluralOperands.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/**
 * See the [Rust documentation for `PluralRules`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html) for more information.
 */
const PluralRules_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_PluralRules_destroy_mv1(ptr);
});

export class PluralRules {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("PluralRules is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            PluralRules_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    get ffiValue() {
        return this.#ptr;
    }


    /**
     * Construct an [`PluralRules`] for the given locale, for cardinal numbers, using compiled data.
     *
     * See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_cardinal) for more information.
     */
    static createCardinal(locale) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_PluralRules_create_cardinal_mv1(diplomatReceive.buffer, locale.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {
            diplomatReceive.free();
        }
    }

    /**
     * Construct an [`PluralRules`] for the given locale, for cardinal numbers, using a particular data source.
     *
     * See the [Rust documentation for `try_new_cardinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_cardinal) for more information.
     */
    static createCardinalWithProvider(provider, locale) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_PluralRules_create_cardinal_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {
            diplomatReceive.free();
        }
    }

    /**
     * Construct an [`PluralRules`] for the given locale, for ordinal numbers, using compiled data.
     *
     * See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_ordinal) for more information.
     */
    static createOrdinal(locale) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_PluralRules_create_ordinal_mv1(diplomatReceive.buffer, locale.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {
            diplomatReceive.free();
        }
    }

    /**
     * Construct an [`PluralRules`] for the given locale, for ordinal numbers, using a particular data source.
     *
     * See the [Rust documentation for `try_new_ordinal`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.try_new_ordinal) for more information.
     */
    static createOrdinalWithProvider(provider, locale) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_PluralRules_create_ordinal_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue, locale.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError: ' + cause.value, { cause });
            }
            return new PluralRules(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {
            diplomatReceive.free();
        }
    }

    /**
     * Get the category for a given number represented as operands
     *
     * See the [Rust documentation for `category_for`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.category_for) for more information.
     */
    categoryFor(op) {

        const result = wasm.icu4x_PluralRules_category_for_mv1(this.ffiValue, op.ffiValue);

        try {
            return new PluralCategory(diplomatRuntime.internalConstructor, result);
        }

        finally {
        }
    }

    /**
     * Get all of the categories needed in the current locale
     *
     * See the [Rust documentation for `categories`](https://docs.rs/icu/latest/icu/plurals/struct.PluralRules.html#method.categories) for more information.
     */
    get categories() {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 6, 1, false);


        const result = wasm.icu4x_PluralRules_categories_mv1(diplomatReceive.buffer, this.ffiValue);

        try {
            return PluralCategories._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer);
        }

        finally {
            diplomatReceive.free();
        }
    }

    constructor(symbol, ptr, selfEdge) {
        return this.#internalConstructor(...arguments)
    }
}