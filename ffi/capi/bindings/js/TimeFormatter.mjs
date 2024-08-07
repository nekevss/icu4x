// generated by diplomat-tool
import { DataProvider } from "./DataProvider.mjs"
import { DateTime } from "./DateTime.mjs"
import { Error } from "./Error.mjs"
import { IsoDateTime } from "./IsoDateTime.mjs"
import { Locale } from "./Locale.mjs"
import { Time } from "./Time.mjs"
import { TimeLength } from "./TimeLength.mjs"
import wasm from "./diplomat-wasm.mjs";
import * as diplomatRuntime from "./diplomat-runtime.mjs";


/** An ICU4X TimeFormatter object capable of formatting an [`Time`] type (and others) as a string
*
*See the [Rust documentation for `TimeFormatter`](https://docs.rs/icu/latest/icu/datetime/struct.TimeFormatter.html) for more information.
*/

const TimeFormatter_box_destroy_registry = new FinalizationRegistry((ptr) => {
    wasm.icu4x_TimeFormatter_destroy_mv1(ptr);
});
export class TimeFormatter {
    // Internal ptr reference:
    #ptr = null;

    // Lifetimes are only to keep dependencies alive.
    // Since JS won't garbage collect until there are no incoming edges.
    #selfEdge = [];
    
    
    constructor(ptr, selfEdge) {
        
        this.#ptr = ptr;
        this.#selfEdge = selfEdge;
        // Unconditionally register to destroy when this object is ready to garbage collect.
        TimeFormatter_box_destroy_registry.register(this, this.#ptr);
    }

    get ffiValue() {
        return this.#ptr;
    }


    static createWithLength(provider, locale, length) {
        
        const diplomat_receive_buffer = wasm.diplomat_alloc(5, 4);
        const result = wasm.icu4x_TimeFormatter_create_with_length_mv1(diplomat_receive_buffer, provider.ffiValue, locale.ffiValue, length.ffiValue);
    
        try {
    
            if (!diplomatRuntime.resultFlag(wasm, diplomat_receive_buffer, 4)) {
                const cause = (() => {for (let i of Error.values) { if(i[1] === diplomatRuntime.enumDiscriminant(wasm, diplomat_receive_buffer)) return Error[i[0]]; } return null;})();
                throw new Error('Error: ' + cause.value, { cause });
            }
            return new TimeFormatter(diplomatRuntime.ptrRead(wasm, diplomat_receive_buffer), []);
        } finally {
        
            wasm.diplomat_free(diplomat_receive_buffer, 5, 4);
        
        }
    }

    formatTime(value) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_TimeFormatter_format_time_mv1(this.ffiValue, value.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    formatDatetime(value) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_TimeFormatter_format_datetime_mv1(this.ffiValue, value.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    formatIsoDatetime(value) {
        
        const write = wasm.diplomat_buffer_write_create(0);
        wasm.icu4x_TimeFormatter_format_iso_datetime_mv1(this.ffiValue, value.ffiValue, write);
    
        try {
    
            return diplomatRuntime.readString8(wasm, wasm.diplomat_buffer_write_get_bytes(write), wasm.diplomat_buffer_write_len(write));
        } finally {
        
            wasm.diplomat_buffer_write_destroy(write);
        
        }
    }

    

}