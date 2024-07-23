// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";

// Base enumerator definition
/** See the [Rust documentation for `IsoFormat`](https://docs.rs/icu/latest/icu/datetime/time_zone/enum.IsoFormat.html) for more information.
*/
export class IsoTimeZoneFormat {
    constructor(value : IsoTimeZoneFormat | string);

    get value() : string;

    get ffiValue() : number;

    static Basic : IsoTimeZoneFormat;

    static Extended : IsoTimeZoneFormat;

    static UtcBasic : IsoTimeZoneFormat;

    static UtcExtended : IsoTimeZoneFormat;


    

}