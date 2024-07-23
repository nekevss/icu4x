// generated by diplomat-tool
import type { pointer, char } from "./diplomat-runtime.d.ts";


/** An ICU4X Time object representing a time in terms of hour, minute, second, nanosecond
*
*See the [Rust documentation for `Time`](https://docs.rs/icu/latest/icu/calendar/struct.Time.html) for more information.
*/
export class Time {
    

    get ffiValue(): pointer;


    static create(hour: number, minute: number, second: number, nanosecond: number): Time;

    static fromString(v: string): Time;

    static midnight(): Time;

    get hour(): number;

    get minute(): number;

    get second(): number;

    get nanosecond(): number;

    

}