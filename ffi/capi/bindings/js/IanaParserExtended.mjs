// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import { TimeZoneAndCanonicalAndNormalized } from "./TimeZoneAndCanonicalAndNormalized.mjs"
import { TimeZoneAndCanonicalAndNormalizedIterator } from "./TimeZoneAndCanonicalAndNormalizedIterator.mjs"
import { TimeZoneAndCanonicalIterator } from "./TimeZoneAndCanonicalIterator.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";

const IanaParserExtended_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_IanaParserExtended_destroy_mv1(ptr);
});

/**
 * A mapper between IANA time zone identifiers and BCP-47 time zone identifiers.
 *
 * This mapper supports two-way mapping, but it is optimized for the case of IANA to BCP-47.
 * It also supports normalizing and canonicalizing the IANA strings.
 *
 * See the [Rust documentation for `IanaParserExtended`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtended.html) for more information.
 */
export class IanaParserExtended {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];

    #internalConstructor(symbol, ptr, selfEdge) {
        if (symbol !== diplomatRuntime.internalConstructor) {
            console.error("IanaParserExtended is an Opaque type. You cannot call its constructor.");
            return;
        }
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;

        // Are we being borrowed? If not, we can register.
        if (this.#selfEdge.length === 0) {
            IanaParserExtended_box_destroy_registry.register(this, this.#ptr);
        }

        return this;
    }
    /** @internal */
    get ffiValue() {
        return this.#ptr;
    }


    /**
     * Create a new {@link IanaParserExtended} using compiled data
     *
     * See the [Rust documentation for `new`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtended.html#method.new) for more information.
     */
    #defaultConstructor() {

        const result = wasm.icu4x_IanaParserExtended_create_mv1();

        try {
            return new IanaParserExtended(diplomatRuntime.internalConstructor, result, []);
        }

        finally {
        }
    }

    /**
     * Create a new {@link IanaParserExtended} using a particular data source
     *
     * See the [Rust documentation for `new`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtended.html#method.new) for more information.
     */
    static createWithProvider(provider) {
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 5, 4, true);


        const result = wasm.icu4x_IanaParserExtended_create_with_provider_mv1(diplomatReceive.buffer, provider.ffiValue);

        try {
            if (!diplomatReceive.resultFlag) {
                const cause = new DataError(diplomatRuntime.internalConstructor, diplomatRuntime.enumDiscriminant(wasm, diplomatReceive.buffer));
                throw new globalThis.Error('DataError.' + cause.value, { cause });
            }
            return new IanaParserExtended(diplomatRuntime.internalConstructor, diplomatRuntime.ptrRead(wasm, diplomatReceive.buffer), []);
        }

        finally {
            diplomatReceive.free();
        }
    }

    /**
     * See the [Rust documentation for `parse`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.parse) for more information.
     */
    parse(value) {
        let functionCleanupArena = new diplomatRuntime.CleanupArena();

        const valueSlice = functionCleanupArena.alloc(diplomatRuntime.DiplomatBuf.sliceWrapper(wasm, diplomatRuntime.DiplomatBuf.str8(wasm, value)));
        const diplomatReceive = new diplomatRuntime.DiplomatReceiveBuf(wasm, 20, 4, false);

        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.icu4x_IanaParserExtended_parse_mv1(diplomatReceive.buffer, this.ffiValue, valueSlice.ptr);

        try {
            return TimeZoneAndCanonicalAndNormalized._fromFFI(diplomatRuntime.internalConstructor, diplomatReceive.buffer, aEdges);
        }

        finally {
            functionCleanupArena.free();

            diplomatReceive.free();
        }
    }

    /**
     * See the [Rust documentation for `iter`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.iter) for more information.
     */
    iter() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.icu4x_IanaParserExtended_iter_mv1(this.ffiValue);

        try {
            return new TimeZoneAndCanonicalIterator(diplomatRuntime.internalConstructor, result, [], aEdges);
        }

        finally {
        }
    }

    /**
     * See the [Rust documentation for `iter_all`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtendedBorrowed.html#method.iter_all) for more information.
     */
    iterAll() {
        // This lifetime edge depends on lifetimes 'a
        let aEdges = [this];


        const result = wasm.icu4x_IanaParserExtended_iter_all_mv1(this.ffiValue);

        try {
            return new TimeZoneAndCanonicalAndNormalizedIterator(diplomatRuntime.internalConstructor, result, [], aEdges);
        }

        finally {
        }
    }

    /**
     * Create a new {@link IanaParserExtended} using compiled data
     *
     * See the [Rust documentation for `new`](https://docs.rs/icu/2.0.0/icu/time/zone/iana/struct.IanaParserExtended.html#method.new) for more information.
     */
    constructor() {
        if (arguments[0] === diplomatRuntime.exposeConstructor) {
            return this.#internalConstructor(...Array.prototype.slice.call(arguments, 1));
        } else if (arguments[0] === diplomatRuntime.internalConstructor) {
            return this.#internalConstructor(...arguments);
        } else {
            return this.#defaultConstructor(...arguments);
        }
    }
}