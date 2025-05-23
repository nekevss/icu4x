// generated by diplomat-tool
import type { UtcOffset } from "./UtcOffset"
import type { pointer, codepoint } from "./diplomat-runtime.d.ts";


/**
 * See the [Rust documentation for `VariantOffsets`](https://docs.rs/icu/latest/icu/time/zone/struct.VariantOffsets.html) for more information.
 */


export class VariantOffsets {
    get standard(): UtcOffset;
    get daylight(): UtcOffset | null;

}