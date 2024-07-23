// generated by diplomat-tool
import { DataError } from "./DataError.mjs"
import { DataProvider } from "./DataProvider.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** See the [Rust documentation for `ComposingNormalizer`](https://docs.rs/icu/latest/icu/normalizer/struct.ComposingNormalizer.html) for more information.
*/

const ComposingNormalizer_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_ComposingNormalizer_destroy_mv1(ptr);
});
export class ComposingNormalizer {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        ComposingNormalizer_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static createNfc(provider) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_ComposingNormalizer_create_nfc_mv1(diplomat_receive_buffer, provider.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new ComposingNormalizer(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    static createNfkc(provider) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_ComposingNormalizer_create_nfkc_mv1(diplomat_receive_buffer, provider.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = DataError[Array.from(DataError.values.keys())[diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)]];
                throw new Error('DataError: ' + cause.value, { cause });
            }
            return new ComposingNormalizer(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    normalize(s) {
        
        const sSlice = diplomatRuntime.DiplomatBuf.str8(wasm, s);
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_ComposingNormalizer_normalize_mv1(this.ffiValue, sSlice.ptr, sSlice.size, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            sSlice.free();
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    isNormalized(s) {
        
        const sSlice = diplomatRuntime.DiplomatBuf.str16(wasm, s);
        const result = wasm.icu4x_ComposingNormalizer_is_normalized_utf16_mv1(this.ffiValue, sSlice.ptr, sSlice.size);
    
        try {
    
            return result;
        } finally {
        
            sSlice.free();
        
        }
    }

    isNormalizedUpTo(s) {
        
        const sSlice = diplomatRuntime.DiplomatBuf.str16(wasm, s);
        const result = wasm.icu4x_ComposingNormalizer_is_normalized_utf16_up_to_mv1(this.ffiValue, sSlice.ptr, sSlice.size);
    
        try {
    
            return result;
        } finally {
        
            sSlice.free();
        
        }
    }

    

}